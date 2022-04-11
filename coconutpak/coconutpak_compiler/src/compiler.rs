use crate::error::CompilerError;
use crate::error::CompilerError::XmlError;
use escaper::encode_minimal;
use html_parser::{Dom, Element, Node};
use itertools::Itertools;
use kindkapibari_core::language::Language;
use kindkapibari_core::manifest::CoconutPakManifest;
use kindkapibari_core::output::CoconutPakOutput;
use kindkapibari_core::responses::{Message, Response};
use kindkapibari_core::tags::Tags;
use kindkapibari_core::text::TextContainer;
use language_tags::LanguageTag;
use log::{error, warn};
use pulldown_cmark::{html, Event, Options, Parser, Tag};
use std::collections::HashMap;
use std::io::Read;
use std::path::PathBuf;
use std::{
    borrow::Cow,
    fmt::{Display, Formatter},
    fs::File,
    ops::{Deref, DerefMut},
    path::Path,
    str::FromStr,
};
use walkdir::WalkDir;
use xml::attribute::OwnedAttribute;
use xml::name::OwnedName;
use xml::namespace::Namespace;
use xml::reader::{Error, XmlEvent};
use xml::{EventReader, EventWriter};

const ALLOWED_TAGS: [&str; 20] = [
    "CoconutPakText",
    "subnamespace",
    "langcode",
    "description",
    "responses",
    "response",
    "message",
    "i",
    "b",
    "color",
    "strike",
    "under",
    "inline",
    "super",
    "sub",
    "highlight",
    "quote",
    "wave",
    "shaky",
    "spoiler",
];

const ALLOWED_CHARS: &[char] = &[
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '_', '-',
];

const ALLOWED_START_CHARS: &[char] = &[
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

pub struct Compiler<'a> {
    manifest: CoconutPakManifest,
    source_path: String,
    mock_renderer: Registry<'a>,
}

impl Compiler {
    pub fn new(source_path: Option<String>) -> Result<Compiler, CompilerError> {
        // default to CWD
        let source_path =
            source_path.unwrap_or(std::env::current_dir()?.to_string_lossy().to_string());

        let mut registry = Registry::new();
        registry.set_strict(true);

        // try and find a Coconut.toml
        let mut fs_coconut_manifest = match File::open(&source_path + "Coconut.toml") {
            Ok(cm) => cm,
            Err(why) => {
                return Err(CompilerError::FileError {
                    file: source_path + "Coconut.toml",
                    why: why.to_string(),
                })
            }
        };

        let mut read_string_manifest = String::new();
        if fs_coconut_manifest
            .read_to_string(&mut read_string_manifest)
            .is_err()
        {
            return Err(CompilerError::FileError {
                file: source_path + "Coconut.toml",
                why: "Failed to read to string".to_string(),
            });
        }

        let mut manifest = match toml::from_str::<CoconutPakManifest>(&read_string_manifest) {
            Ok(m) => m,
            Err(why) => return Err(CompilerError::BadManifest(why.to_string())),
        };

        // check for README
        let readme = match File::open(&source_path + "README.md") {
            Ok(mut readme) => {
                let mut rme_string = String::new();
                readme.read_to_string(&mut rme_string);
                rme_string
            }
            Err(why) => {
                warn!("Could not find README!");
                String::new()
            }
        };

        manifest.readme = readme;

        // check for source dir
        // i thought
        if let Ok(true) = PathBuf::from_str(&source_path + "src").map(|x| x.is_dir()) {
            Ok(Compiler {
                manifest,
                source_path: (&source_path + "src").to_string(),
                mock_renderer: registry,
            })
        } else {
            Err(CompilerError::SourcePathInvalid)
        }
    }

    pub fn compile(self) -> Result<CoconutPakOutput, CompilerError> {
        // check manifest for bad characters
        if !check_name_good(self.manifest.name) {
            return Err(CompilerError::BadField {
                field: "name".to_string(),
                why: "Failed Check Name Good".to_string(),
            });
        }
        if !self
            .manifest
            .tags
            .join("")
            .replace(ALLOWED_CHARS, "")
            .is_empty()
        {
            return Err(CompilerError::InvalidCharacters {
                field: "tags".to_string(),
                bad_char: format!("Allowed Characters: {:?}", ALLOWED_CHARS),
            });
        }
        if !self
            .manifest
            .categories
            .join("")
            .replace(ALLOWED_CHARS, "")
            .is_empty()
        {
            return Err(CompilerError::InvalidCharacters {
                field: "categories".to_string(),
                bad_char: format!("Allowed Characters: {:?}", ALLOWED_CHARS),
            });
        }
        // see whats inside src

        let mut files_inside_src_texts = HashMap::new();
        // let files_inside_src_texts = vec![];
        // let files_inside_src_texts = vec![];
        // TODO: Different Types (themes, animals, etc)

        for maybe_entry in WalkDir::new(self.source_path).into_iter() {
            let entry = match maybe_entry {
                Ok(e) => e,
                Err(why) => {
                    warn!("Skipping file due to {why}")
                }
            };

            if entry.metadata().unwrap().is_file() && !entry.path_is_symlink() {
                if entry
                    .file_name()
                    .to_string_lossy()
                    .to_lowercase()
                    .ends_with(".copt")
                {
                    // read file to string and store
                    let mut file = File::open(entry.path())?;
                    let mut read_to = String::new();
                    file.read_to_string(&mut read_to)?;
                    files_inside_src_texts
                        .insert(entry.path().to_string_lossy().to_string(), read_to);
                }
                // add others as well later
            }
        }

        let mut error_out = false;

        let texts = vec![];
        for text_to_compile in files_inside_src_texts {
            let text = match self.compile_text(text_to_compile.1) {
                Ok(t) => t,
                Err(why) => {
                    let filename = text_to_compile.0;
                    error_out = true;
                    error!("Failed to compile {filename}: {why}");
                    continue;
                }
            };
        }

        Ok(CoconutPakOutput {
            edition: self.manifest.clone().version,
            manifest: self.manifest.clone(),
            register_text_containers: {
                if !texts.is_empty() {
                    Some(texts)
                } else {
                    None
                }
            },
        })
    }

    fn compile_text(&self, intext: String) -> Result<TextContainer, CompilerError> {
        // markup
        let mut markup = Options::all();
        let parser = Parser::new_ext(&intext, options);
        let mut text = String::new();
        html::push_html(&mut text, parser);

        // check DOM
        if let Err(why) = Dom::parse(&text) {
            return Err(CompilerError::XmlError {
                why: why.to_string(),
            });
        }

        let mut xml = EventReader::from_str(&text);

        // check if its CoconutPakText
        match xml.next().map_err(|why| CompilerError::XmlError {
            why: why.to_string(),
        })? {
            XmlEvent::StartElement { name, .. } => {
                if name != "CoconutPakText" {
                    return Err(CompilerError::XmlError {
                        why: "Expected CoconutPakText".to_string(),
                    });
                }
            }
            _ => {}
        }

        // subnamespace, tag, langcode, description

        let subnamespace = match xml.next() {
            Ok(XmlEvent::StartElement { name, .. }) => {
                if name != "subnamespace" {
                    return Err(CompilerError::XmlError {
                        why: "Expected subnamespace".to_string(),
                    });
                }
                if let Ok(XmlEvent::CData(data)) = xml.next() {
                    if data.starts_with(ALLOWED_START_CHARS)
                        && data.replace(ALLOWED_CHARS, "") == ""
                    {
                        data
                    } else {
                        return Err(CompilerError::BadText {
                            why: "Disallowed Character in subnamespace".to_string(),
                        });
                    }
                } else {
                    return Err(CompilerError::XmlError {
                        why: "Expected Data".to_string(),
                    });
                }
            }
            _ => {
                return Err(CompilerError::XmlError {
                    why: "Expected subnamespace".to_string(),
                });
            }
        };

        if let Ok(XmlEvent::EndElement { .. }) = xml.next() {
        } else {
            return Err(CompilerError::XmlError {
                why: "Expected Exit out of previous tag".to_string(),
            });
        }

        let tag = match xml.next() {
            Ok(XmlEvent::StartElement { name, .. }) => {
                if name != "tag" {
                    return Err(CompilerError::XmlError {
                        why: "Expected tag".to_string(),
                    });
                }
                if let Ok(XmlEvent::CData(data)) = xml.next() {
                    if data.starts_with(ALLOWED_START_CHARS)
                        && data.replace(ALLOWED_CHARS, "") == ""
                    {
                        data.parse::<Tags>().map_err(|why| CompilerError::BadText {
                            why: why.to_string(),
                        })?
                    } else {
                        return Err(CompilerError::BadText {
                            why: "Disallowed Character in tag".to_string(),
                        });
                    }
                } else {
                    return Err(CompilerError::XmlError {
                        why: "Expected Data".to_string(),
                    });
                }
            }
            _ => {
                return Err(CompilerError::XmlError {
                    why: "Expected tag".to_string(),
                });
            }
        };

        if let Ok(XmlEvent::EndElement { .. }) = xml.next() {
        } else {
            return Err(CompilerError::XmlError {
                why: "Expected Exit out of previous tag".to_string(),
            });
        }

        let langcode = match xml.next() {
            Ok(XmlEvent::StartElement { name, .. }) => {
                if name != "langcode" {
                    return Err(CompilerError::XmlError {
                        why: "Expected langcode".to_string(),
                    });
                }
                if let Ok(XmlEvent::CData(data)) = xml.next() {
                    if data.starts_with(ALLOWED_START_CHARS)
                        && data.replace(ALLOWED_CHARS, "") == ""
                    {
                        LanguageTag::parse(&data).map_err(|why| CompilerError::BadText {
                            why: why.to_string(),
                        })?
                    } else {
                        return Err(CompilerError::BadText {
                            why: "Disallowed Character in langcode".to_string(),
                        });
                    }
                } else {
                    return Err(CompilerError::XmlError {
                        why: "Expected Data".to_string(),
                    });
                }
            }
            _ => {
                return Err(CompilerError::XmlError {
                    why: "Expected langcode".to_string(),
                });
            }
        };

        if let Ok(XmlEvent::EndElement { .. }) = xml.next() {
        } else {
            return Err(CompilerError::XmlError {
                why: "Expected Exit out of previous tag".to_string(),
            });
        }

        let description = match xml.next() {
            Ok(XmlEvent::StartElement { name, .. }) => {
                if name != "description" {
                    return Err(CompilerError::XmlError {
                        why: "Expected description".to_string(),
                    });
                }
                if let Ok(XmlEvent::CData(data)) = xml.next() {
                    if data.starts_with(ALLOWED_START_CHARS)
                        && data.replace(ALLOWED_CHARS, "") == ""
                    {
                        data
                    } else {
                        return Err(CompilerError::BadText {
                            why: "Disallowed Character in description".to_string(),
                        });
                    }
                } else {
                    return Err(CompilerError::XmlError {
                        why: "Expected Data".to_string(),
                    });
                }
            }
            _ => {
                return Err(CompilerError::XmlError {
                    why: "Expected description".to_string(),
                });
            }
        };

        if let Ok(XmlEvent::EndElement { .. }) = xml.next() {
        } else {
            return Err(CompilerError::XmlError {
                why: "Expected Exit out of previous tag".to_string(),
            });
        }

        // response
        if let Ok(XmlEvent::StartElement {
            name, attributes, ..
        }) = xml.next()
        {
            if name != "responses" {
                return Err(CompilerError::XmlError {
                    why: "Expected responses".to_string(),
                });
            }
            let mut depth = 0;
            let mut message_start_set = false;
            let mut current_response = Response::default();
            let mut current_message = Message::default();
            let mut response_vec = vec![];
            loop {
                let xml_event_copy = xml.next().clone();
                match xml_event_copy {
                    Ok(xml_event) => match xml_event {
                        XmlEvent::StartElement {
                            name, attributes, ..
                        } => {
                            let attributes = owned_attribute_vec_to_hashmap(attributes);
                            if name == "response" {
                                current_response.name = attributes
                                    .get("name")
                                    .ok_or(CompilerError::BadAttr {
                                        attribute: "name".to_string(),
                                        value: "None".to_string(),
                                        why: "Does not exist".to_string(),
                                    })?
                                    .to_string();
                                current_response.probability = attributes
                                    .get("probability")
                                    .map(|prob| {
                                        prob.parse::<f32>().map_err(|x| CompilerError::BadAttr {
                                            attribute: "probability".to_string(),
                                            value: prob.to_string(),
                                            why: x.to_string(),
                                        })
                                    })
                                    .ok_or(CompilerError::BadAttr {
                                        attribute: "probability".to_string(),
                                        value: "None".to_string(),
                                        why: "Does not exist.".to_string(),
                                    })
                                    .flatten()?;
                                current_response.usable_for_welcome = attributes
                                    .get("welcome")
                                    .map(|prob| {
                                        prob.parse::<bool>().map_err(|x| CompilerError::BadAttr {
                                            attribute: "welcome".to_string(),
                                            value: prob.to_string(),
                                            why: x.to_string(),
                                        })
                                    })
                                    .ok_or(CompilerError::BadAttr {
                                        attribute: "welcome".to_string(),
                                        value: "None".to_string(),
                                        why: "Does not exist.".to_string(),
                                    })
                                    .flatten()?;

                                loop {
                                    let inner_message_copy = xml.next().clone();

                                    match inner_message_copy {
                                        Ok(event) => match event {
                                            XmlEvent::StartElement {
                                                name, attributes, ..
                                            } => {
                                                if name == "message" {
                                                    let attributes =
                                                        owned_attribute_vec_to_hashmap(attributes);
                                                    let wait = attributes
                                                        .get("wait")
                                                        .map(|prob| {
                                                            prob.parse::<f32>().map_err(|x| {
                                                                CompilerError::BadAttr {
                                                                    attribute: "wait".to_string(),
                                                                    value: prob.to_string(),
                                                                    why: x.to_string(),
                                                                }
                                                            })
                                                        })
                                                        .ok_or(CompilerError::BadAttr {
                                                            attribute: "probability".to_string(),
                                                            value: "None".to_string(),
                                                            why: "Does not exist.".to_string(),
                                                        })
                                                        .flatten()?;

                                                    if wait > 5_f32 {
                                                        return Err(CompilerError::BadAttr {
                                                            attribute: "wait".to_string(),
                                                            value: wait.to_string(),
                                                            why: "wait > 5.0, max wait is 5"
                                                                .to_string(),
                                                        });
                                                    }
                                                    loop {
                                                        let message_event = xml.next().clone();
                                                        let mut event_writer =
                                                            EventWriter::new(Vec::new());

                                                        match message_event {
                                                            Ok(event) => match event {
                                                                XmlEvent::StartElement {
                                                                    name,
                                                                    attributes,
                                                                    ..
                                                                } => {
                                                                    match name.to_string().as_str()
                                                                    {
                                                                        "u" => {
                                                                            event_writer.write(
                                                                                XmlEvent::StartElement {
                                                                                    name: "u".to_string().into(),
                                                                                    attributes: vec![].into(),
                                                                                    namespace: Default::default(),
                                                                                }
                                                                            )
                                                                        },
                                                                        "super" => {event_writer.write(
                                                                            XmlEvent::StartElement {
                                                                                name: "sup".to_string().into(),
                                                                                attributes: vec![].into(),
                                                                                namespace: Default::default(),
                                                                            }
                                                                        )},
                                                                        "sub" => {event_writer.write(
                                                                            XmlEvent::StartElement {
                                                                                name: "sub".to_string().into(),
                                                                                attributes: vec![].into(),
                                                                                namespace: Default::default(),
                                                                            }
                                                                        )},
                                                                        "highlight" => {event_writer.write(
                                                                            XmlEvent::StartElement {
                                                                                name: "mark".to_string().into(),
                                                                                attributes: vec![].into(),
                                                                                namespace: Default::default(),
                                                                            }
                                                                        )},
                                                                        "wave" => {event_writer.write(
                                                                            XmlEvent::StartElement {
                                                                                name: "span".to_string().into(),
                                                                                attributes: vec![
                                                                                    OwnedAttribute {
                                                                                        name: "class".parse().unwrap(),
                                                                                        value: "text-wave".to_string()
                                                                                    }
                                                                                ].into(),
                                                                                namespace: Default::default(),
                                                                            }
                                                                        )},
                                                                        "shaky" => {event_writer.write(
                                                                            XmlEvent::StartElement {
                                                                                name: "span".to_string().into(),
                                                                                attributes: vec![OwnedAttribute {
                                                                                    name: "class".parse().unwrap(),
                                                                                    value: "text-shaky".to_string()
                                                                                }].into(),
                                                                                namespace: Default::default(),
                                                                            }
                                                                        )},
                                                                        "br" => {event_writer.write(
                                                                            XmlEvent::StartElement {
                                                                                name: "br".to_string().into(),
                                                                                attributes: vec![].into(),
                                                                                namespace: Default::default(),
                                                                            }
                                                                        )},
                                                                        "spoiler" => {event_writer.write(
                                                                            XmlEvent::StartElement {
                                                                                name: "span".to_string().into(),
                                                                                attributes: vec![
                                                                                    OwnedAttribute {
                                                                                        name: "class".parse().unwrap(),
                                                                                        value: "text-spoiler".to_string()
                                                                                    }
                                                                                ].into(),
                                                                                namespace: Default::default(),
                                                                            }
                                                                        )},
                                                                        name => {event_writer.write(
                                                                            XmlEvent::StartElement {
                                                                                name: name.to_string().into(),
                                                                                attributes,
                                                                                namespace: Default::default(),
                                                                            }
                                                                        )},
                                                                    }
                                                                }
                                                                XmlEvent::EndElement {
                                                                    name,
                                                                    ..
                                                                } => {
                                                                    match name.to_string().as_str()
                                                                    {
                                                                        "u" => {
                                                                            event_writer.write(
                                                                                XmlEvent::EndElement { name: "u".to_string().into(), }
                                                                        },
                                                                        "super" => {event_writer.write(
                                                                            XmlEvent::EndElement {
                                                                                name: "sup".to_string().into(),
                                                                            }
                                                                        )},
                                                                        "sub" => {event_writer.write(
                                                                            XmlEvent::EndElement {
                                                                                name: "sub".to_string().into(),
                                                                            }
                                                                        )},
                                                                        "highlight" => {event_writer.write(
                                                                            XmlEvent::EndElement {
                                                                                name: "mark".to_string().into(),
                                                                            }
                                                                        )},
                                                                        "wave" => {event_writer.write(
                                                                            XmlEvent::EndElement {
                                                                                name: "span".to_string().into(),
                                                                            }
                                                                        )},
                                                                        "shaky" => {event_writer.write(
                                                                            XmlEvent::EndElement {
                                                                                name: "span".to_string().into(),
                                                                            }
                                                                        )},
                                                                        "br" => {event_writer.write(
                                                                            XmlEvent::EndElement {
                                                                                name: "br".to_string().into(),
                                                                            }
                                                                        )},
                                                                        "spoiler" => {event_writer.write(
                                                                            XmlEvent::EndElement {
                                                                                name: "span".to_string().into(),
                                                                            }
                                                                        )},
                                                                        name => {event_writer.write(
                                                                            XmlEvent::EndElement {
                                                                                name: name.to_string().into(),
                                                                            }
                                                                        )},
                                                                    }
                                                                }
                                                                XmlEvent::CData(data)
                                                                | XmlEvent::Characters(data) => {
                                                                    eventwriter.write(
                                                                        XmlEvent::Characters(

                                                                        )
                                                                    )
                                                                }
                                                                XmlEvent::Comment(_)
                                                                | XmlEvent::Whitespace(_) => {}
                                                                unexpected => {
                                                                    return Err(CompilerError::XmlError {
                                                                            why: format!("Unexpected XML: {unexpected:?}"),
                                                                        });
                                                                }
                                                            },
                                                            Err(why) => {
                                                                return Err(
                                                                    CompilerError::XmlError {
                                                                        why: format!(
                                                                            "Bad XML: {why:?}"
                                                                        ),
                                                                    },
                                                                );
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    return Err(CompilerError::XmlError {
                                                        why: format!("Unexpected XML: {name}"),
                                                    });
                                                }
                                            }
                                            XmlEvent::EndElement { name, .. } => {}
                                            XmlEvent::Comment(_) | XmlEvent::Whitespace(_) => {
                                                continue;
                                            }
                                            unexpected => {
                                                return Err(CompilerError::XmlError {
                                                    why: format!("Unexpected XML: {unexpected:?}"),
                                                });
                                            }
                                        },
                                        Err(why) => {
                                            return Err(CompilerError::XmlError {
                                                why: why.to_string(),
                                            });
                                        }
                                    }
                                }
                            }
                        }
                        XmlEvent::EndElement { name, .. } => {}
                        XmlEvent::CData(data) | XmlEvent::Characters(data) => {}
                        XmlEvent::Comment(_) | XmlEvent::Whitespace(_) => {
                            continue;
                        }
                        unexpected => {
                            return Err(CompilerError::XmlError {
                                why: format!("Unexpected XML Event: {unexpected:?}"),
                            })
                        }
                    },
                    Err(why) => {
                        return Err(CompilerError::XmlError {
                            why: why.to_string(),
                        })
                    }
                }
            }
        }
        return Err(CompilerError::XmlError {
            why: "".to_string(),
        });

        // let markdown_opt = Options::all();
        //
        // let dom = Dom::parse(&text).map_err(|x| CompilerError::XmlError { why: x.to_string() })?;
        // if dom.children.len() == 1 {
        //     return Err(CompilerError::XmlError {
        //         why: "There can only be one root child.".to_string(),
        //     });
        // }
        //
        // let mut root_children = Vec::new();
        //
        // if let Node::Element(e) = &dom.children[0] {
        //     if e.name != "CoconutPakText" {
        //         return Err(CompilerError::XmlError {
        //             why: "First root child must be CoconutPakText for Text.".to_string(),
        //         });
        //     }
        //     root_children = e.children.clone();
        // } else {
        //     return Err(CompilerError::XmlError {
        //         why: "First root child must be element.".to_string(),
        //     });
        // }
        //
        // let subnamespace = if let Node::Element(e) = &root_children[0] {
        //     if e.name != "subnamespace" {
        //         return Err(CompilerError::XmlError {
        //             why: "First child must be subnamespace".to_string(),
        //         });
        //     }
        //     if let Node::Text(t) = &e.children[0] {
        //         t.to_string()
        //     } else {
        //         return Err(CompilerError::XmlError {
        //             why: "Subnamespace must be string.".to_string(),
        //         });
        //     }
        // } else {
        //     return Err(CompilerError::XmlError {
        //         why: "No Element".to_string(),
        //     });
        // };
        //
        // let tags = if let Node::Element(e) = &root_children[1] {
        //     if e.name != "tag" {
        //         return Err(CompilerError::XmlError {
        //             why: "First child must be subnamespace".to_string(),
        //         });
        //     }
        //     if let Node::Text(t) = &e.children[0] {
        //         t.to_string()
        //             .parse::<Tags>()
        //             .map_err(|why| CompilerError::BadText {
        //                 why: why.to_string(),
        //             })?
        //     } else {
        //         return Err(CompilerError::XmlError {
        //             why: "Tag must be string.".to_string(),
        //         });
        //     }
        // } else {
        //     return Err(CompilerError::XmlError {
        //         why: "No Element".to_string(),
        //     });
        // };
        //
        // let langcode = if let Node::Element(e) = &root_children[2] {
        //     if e.name != "langcode" {
        //         return Err(CompilerError::XmlError {
        //             why: "Second child must be langcode".to_string(),
        //         });
        //     }
        //     if let Node::Text(t) = &e.children[0] {
        //         LanguageTag::parse(t)
        //     } else {
        //         return Err(CompilerError::XmlError {
        //             why: "langcode must be string.".to_string(),
        //         });
        //     }
        // } else {
        //     return Err(CompilerError::XmlError {
        //         why: "No Element".to_string(),
        //     });
        // }
        // .map_err(|why| CompilerError::BadAttr {
        //     attribute: "langcode".to_string(),
        //     value: "".to_string(),
        //     why: why.to_string(),
        // })?;
        //
        // let description = if let Node::Element(e) = &root_children[3] {
        //     if e.name != "description" {
        //         return Err(CompilerError::XmlError {
        //             why: "Third child must be description".to_string(),
        //         });
        //     }
        //     if let Node::Text(t) = &e.children[0] {
        //         t.to_string()
        //     } else {
        //         return Err(CompilerError::XmlError {
        //             why: "description must be string.".to_string(),
        //         });
        //     }
        // } else {
        //     return Err(CompilerError::XmlError {
        //         why: "No Element".to_string(),
        //     });
        // };
        //
        // let mut texts = HashMap::new();
        //
        // if let Some(Node::Element(resps)) = root_children.get(3) {
        //     if resps.name != "responses" {
        //         return Err(CompilerError::XmlError {
        //             why: "Did not find `responses` where it should be.".to_string(),
        //         });
        //     }
        //     for responses in resps.children {
        //         if let Node::Element(response) = responses {
        //             if response.name != "response" {
        //                 return Err(CompilerError::XmlError {
        //                     why: "Did not find `response` where it should be.".to_string(),
        //                 });
        //             }
        //             let name = response
        //                 .attributes
        //                 .get("name")
        //                 .map(|n| n.as_deref())
        //                 .flatten()
        //                 .ok_or(CompilerError::BadAttr {
        //                     attribute: "name".to_string(),
        //                     value: "".to_string(),
        //                     why: "Failed to parse".to_string(),
        //                 })?
        //                 .to_string();
        //
        //             let welcome = response
        //                 .attributes
        //                 .get("welcome")
        //                 .map(|w| w.as_deref())
        //                 .flatten()
        //                 .map(|x| x.parse::<bool>().ok())
        //                 .flatten()
        //                 .ok_or(CompilerError::BadAttr {
        //                     attribute: "welcome".to_string(),
        //                     value: "".to_string(),
        //                     why: "Failed to parse".to_string(),
        //                 })?;
        //             let probability = response
        //                 .attributes
        //                 .get("probability")
        //                 .map(|p| p.as_deref())
        //                 .flatten()
        //                 .map(|x| x.parse::<f32>().ok())
        //                 .flatten()
        //                 .ok_or(CompilerError::BadAttr {
        //                     attribute: "probability".to_string(),
        //                     value: "".to_string(),
        //                     why: "Failed to parse".to_string(),
        //                 })?;
        //             for msg in response.children {
        //                 if let Node::Element(message) = msg {
        //                     if message.name != "message" {
        //                         return Err(CompilerError::XmlError { why: "Found other nodes that were not message where there are only supposed to be message.".to_string() });
        //                     }
        //                     let wait = message
        //                         .attributes
        //                         .get("wait")
        //                         .map(|w| w.as_deref())
        //                         .flatten()
        //                         .map(|x| x.parse::<f32>().ok())
        //                         .flatten()
        //                         .ok_or(CompilerError::BadAttr {
        //                             attribute: "wait".to_string(),
        //                             value: "".to_string(),
        //                             why: "Failed to parse".to_string(),
        //                         })?;
        //                     if wait > 5.0 {
        //                         return Err(CompilerError::BadAttr {
        //                             attribute: "wait".to_string(),
        //                             value: wait.to_string(),
        //                             why: "wait > 5.0".to_string(),
        //                         });
        //                     }
        //
        //                     let message_contents = ;
        //                 }
        //             }
        //         } else {
        //             return Err(CompilerError::XmlError {
        //                 why: "Found other nodes where there shouldn't be.".to_string(),
        //             });
        //         }
        //     }
        // }
    }
}

// I do not know what the fuck is going on here but it seems correct so here it will stay.
#[derive(Clone, Debug, PartialOrd, PartialEq)]
enum SpecialTags<'a> {
    HtmlDefault(Cow<'a, str>),
    Color,
    Strike,
    Under,
    Inline,
    Super,
    Sub,
    Highlight,
    Quote,
    Wave,
    Shake,
    Spoiler,
}

impl<'a> SpecialTags<'a> {
    pub fn closer(&self) -> String {
        format!("</{}>", self)
    }
}

impl<'a> Display for SpecialTags<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SpecialTags::HtmlDefault(d) => d,
                SpecialTags::Color => "span",
                SpecialTags::Strike => "del",
                SpecialTags::Under => "u",
                SpecialTags::Inline => "code",
                SpecialTags::Super => "sup",
                SpecialTags::Sub => "sub",
                SpecialTags::Highlight => "mark",
                SpecialTags::Quote => "blockquote",
                SpecialTags::Wave => "span",
                SpecialTags::Shake => "span",
                SpecialTags::Spoiler => "span",
            }
        )
    }
}

impl<'a> From<&'a String> for SpecialTags<'a> {
    fn from(s: &'a String) -> Self {
        match s.as_ref() {
            "color" => SpecialTags::Color,
            "strike" => SpecialTags::Strike,
            "under" => SpecialTags::Under,
            "inline" => SpecialTags::Inline,
            "super" => SpecialTags::Super,
            "sub" => SpecialTags::Sub,
            "highlight" => SpecialTags::Highlight,
            "quote" => SpecialTags::Quote,
            "wave" => SpecialTags::Wave,
            "shaky" => SpecialTags::Shake,
            "spoiler" => SpecialTags::Spoiler,
            d => SpecialTags::HtmlDefault(Cow::from(d)),
        }
    }
}

// fn message_element_to_string(message_element: Element) -> Result<String, CompilerError> {
//     // i am cobbling together a thing that turns this thing back into a string at 4am because i was too fucking lazy to use xml-rs and now im suffering for it
// }

fn owned_attribute_vec_to_hashmap(owned_attrs: Vec<OwnedAttribute>) -> HashMap<String, String> {
    let mut attribute_map = HashMap::with_capacity(owned_attrs.len());
    for attribute in owned_attrs {
        attribute_map.insert(attribute.name.to_string(), attribute.value);
    }
    attribute_map
}
