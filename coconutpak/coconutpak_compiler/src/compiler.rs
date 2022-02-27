use crate::{
    compiler::SpecialTags::HtmlDefault,
    error::CompilerError
};
use coconutpak_core::{
    libjson::LibJson, manifest::CoconutPakManifest, output::Output, text::TextContainer,
};
use bson::{document::ValueAccessResult, Document};
use capybafirmations_commons::languages::Languages;
use escaper::encode_minimal;
use html_parser::{Dom, Element, Node};
use itertools::Itertools;
use log::{error, warn};
use serde_json::from_str;
use std::{
    borrow::Cow,
    collections::BTreeMap,
    fmt::{
        write,
        Display,
        Formatter
    },
    fs::File,
    io::Read,
    ops::{Deref, DerefMut},
    path::Path,
    num::ParseFloatError
};
use xml_dom::level2::{convert::as_document_mut, get_implementation};
use capybafirmations_commons::responses::{Message, Response};

static ALLOWED_TAGS: [&'static str; 16] = [
    "resp", "response", "message", "i", "b", "color", "strike", "under", "inline", "super", "sub", "highlight", "quote", "wave", "shaky", "spoiler"
];

pub struct Compiler {
    manifest: CoconutPakManifest,
    lib: LibJson,
    source_path: String,
}

impl Compiler {
    pub fn new(manifest: CoconutPakManifest, lib: LibJson, source_path: String) -> Self {
        Compiler {
            manifest,
            lib,
            source_path,
        }
    }

    pub fn compile(self) -> Result<Output, CompilerError> {
        let mut source_path = Path::new(&self.source_path);
        if !source_path.is_dir() {
            return Err(CompilerError::SourcePathInvalid);
        }

        // open libjson and see what's inside
        let mut texts = vec![];
        for text_to_compile in self.lib.texts() {
            let text_path = Path::new(text_to_compile);
            let total_path = source_path.join(text_path);
            let container = match File::open(total_path) {
                Ok(f) => match Document::from_reader(f) {
                    Ok(doc) => {
                        let sub_namespace = match doc.get_str("subnamespace") {
                            Ok(sns) => sns.to_owned(),
                            Err(why) => {
                                return Err(CompilerError::BadText {
                                    file: total_path.as_os_str().to_string_lossy().to_string(),
                                    why: why.to_string(),
                                })
                            }
                        };
                        let language_code = match doc.get_str("langcode") {
                            Ok(lang_str) => Languages::from(lang_str),
                            Err(why) => {
                                return Err(CompilerError::BadText {
                                    file: total_path.as_os_str().to_string_lossy().to_string(),
                                    why: why.to_string(),
                                })
                            }
                        };
                        let description = match doc.get_str("description") {
                            Ok(desc) => desc.to_owned(),
                            Err(why) => {
                                return Err(CompilerError::BadText {
                                    file: total_path.as_os_str().to_string_lossy().to_string(),
                                    why: why.to_string(),
                                })
                            }
                        };
                        let responses = match doc.get_str("responses") {
                            Ok(resp_arr) => {
                                let dom_parse = match Dom::parse(resp_arr) {
                                    Ok(d) => {
                                        // ignore all root tags that arnt <resp>
                                        let mut iter = d.children.into_iter().filter_map(|n| match n {
                                            Node::Element(mut e) => {
                                                let name = &e.name;
                                                if name != "resp" {
                                                    warn!("Ignoring element \"{name}\" in file {total_path:?}, ignored element type.");
                                                    None
                                                } else {
                                                    e.children.retain(|e| {
                                                        match e {
                                                            Node::Element(e) => {
                                                                if e.name != "response" {
                                                                    false
                                                                }
                                                                else {
                                                                    true
                                                                }
                                                            }
                                                            _ => false
                                                        }
                                                    });
                                                    Some(e)
                                                }
                                            }
                                            Node::Text(t) => {
                                                warn!("Ignoring text \"{t}\" in file {total_path:?}");
                                                None
                                            }
                                            _ => None,
                                        });

                                        let counts = iter.count();
                                        let filename =
                                            total_path.as_os_str().to_string_lossy().to_string();
                                        if iter.count() != 1 {
                                            error!("In file {filename}: More than one <resp> root tags. ({counts}).");
                                            return Err(CompilerError::BadText {
                                                file: filename,
                                                why: "More than one <resp>".to_string(),
                                            });
                                        }

                                        iter.nth(0).unwrap()
                                    }
                                    Err(why) => {
                                        return Err(CompilerError::BadText {
                                            file: total_path
                                                .as_os_str()
                                                .to_string_lossy()
                                                .to_string(),
                                            why: why.to_string(),
                                        })
                                    }
                                };

                                dom_parse
                                    .children
                                    .into_iter()
                                    .filter_map(|x| -> Option<Result<Response, CompilerError>> match x {
                                        Node::Element(e) => {
                                            if e.name != "response" {
                                                warn!("Ignoring {e:?} in file {filename}");
                                                return None;
                                            }

                                            let probability = match e.attributes.get("probability") {
                                                Some(p) => {
                                                    match p {
                                                        Some(chance) => {
                                                            match str::parse::<f32>(chance) {
                                                                Ok(f) => f,
                                                                Err(why) => {
                                                                    return Some(Err(
                                                                        CompilerError::BadAttr {
                                                                            attribute: "probability".to_string(),
                                                                            value: chance.to_string(),
                                                                            why: why.to_string()
                                                                        }
                                                                    ))
                                                                }
                                                            }
                                                        }
                                                        None => {
                                                            1.0
                                                        }
                                                    }
                                                }
                                                None => {
                                                    1.0
                                                }
                                            };
                                            let welcome = match e.attributes.get("welcome") {
                                                Some(p) => {
                                                    match p {
                                                        Some(welcomable) => {
                                                            match str::parse::<bool>(welcomable) {
                                                                Ok(f) => f,
                                                                Err(why) => {
                                                                    return Some(Err(
                                                                        CompilerError::BadAttr {
                                                                            attribute: "welcome".to_string(),
                                                                            value: welcomable.to_string(),
                                                                            why: why.to_string()
                                                                        }
                                                                    ))
                                                                }
                                                            }
                                                        }
                                                        None => {
                                                            false
                                                        }
                                                    }
                                                }
                                                None => {
                                                    false
                                                }
                                            };

                                            let messages = e.children.into_iter()
                                                .filter_map(|x| -> Option<Result<Message, CompilerError>> match x {
                                                    Node::Element(e) => {
                                                        if e.name != "message" {
                                                            warn!("Ignoring {e:?} in file {filename}");
                                                            return None
                                                        }

                                                        let wait = match e.attributes.get("wait") {
                                                            Some(w) => {
                                                                match w {
                                                                    Some(w_str) => {
                                                                        match str::parse::<f32>(w_str) {
                                                                            Ok(wait_time) => wait_time,
                                                                            Err(why) => {
                                                                                return Some(Err(
                                                                                    CompilerError::BadAttr {
                                                                                        attribute: "probability".to_string(),
                                                                                        value: chance.to_string(),
                                                                                        why: why.to_string()
                                                                                    }
                                                                                ))
                                                                            }
                                                                        }
                                                                    }
                                                                    None => {
                                                                        0.0
                                                                    }
                                                                }
                                                            }
                                                            None => 0.0,
                                                        };

                                                        let message_str = HtmlNodeWrapper { inner: Node::Element(e) }.to_string()?;

                                                        Some(Ok(Message {
                                                            message: message_str,
                                                            wait_after: wait,
                                                        }))
                                                    }
                                                    i => {
                                                        warn!("Ignoring {i:?} in file {filename}");
                                                        None
                                                    }
                                                }).collect::<Result<Vec<Message>, CompilerError>>()?;
                                            
                                            Some(
                                                Ok(
                                                    Response {
                                                        messages,
                                                        probability,
                                                        usable_for_welcome: welcome,
                                                    }
                                                )
                                            )

                                        }
                                        i => {
                                            warn!("Ignoring {i:?} in file {filename}");
                                            None
                                        }
                                    })
                                    .collect::<Result<Vec<Response>, CompilerError>>()?
                            }
                            Err(why) => {
                                return Err(CompilerError::BadText {
                                    file: total_path.as_os_str().to_string_lossy().to_string(),
                                    why: why.to_string(),
                                })
                            }
                        };

                        TextContainer::new(
                            sub_namespace,
                            language_code,
                            description,
                            responses
                        )
                    }
                    Err(why) => {
                        return Err(CompilerError::FileError {
                            file: total_path.as_os_str().to_string_lossy().to_string(),
                            why: why.to_string(),
                        });
                    }
                },
                Err(why) => {
                    return Err(CompilerError::FileError {
                        file: total_path.as_os_str().to_string_lossy().to_string(),
                        why: why.to_string(),
                    });
                }
            };
            texts.push(container);
        }


        Ok(
            Output {
                author: self.manifest.author,
                name: self.manifest.name,
                version: self.manifest.version,
                compatibility: self.manifest.compatibility,
                source: self.manifest.source,
                description: self.manifest.description,
                tags: self.manifest.tags,
                docs: self.manifest.docs,
                homepage: self.manifest.homepage,
                categories: self.manifest.categories,
                register_text_containers: {
                    if texts.len() > 0 {
                        Some(texts)
                    } else {
                        None
                    }
                },
            }
        )
    }
}

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
    pub fn open(&self) -> String {
        format!("<{}>", self)
    }

    pub fn open_no_ctag(&self) -> String {
        format!("<{}", self)
    }

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

impl<'a> From<&String> for SpecialTags<'a> {
    fn from(s: &String) -> Self {
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

#[derive(Clone, PartialEq)]
struct HtmlNodeWrapper {
    pub inner: Node,
}

impl HtmlNodeWrapper {
    pub fn to_string(&self) -> Result<String, CompilerError> {
        let write_str = match &self.inner {
            Node::Text(t) => encode_minimal(t),
            Node::Element(e) => {
                if ALLOWED_TAGS.contains(&(&e).name.as_ref()) { // wtf???? lmao
                    let mut tag_strings = Vec::with_capacity(5);

                    let open_tag_name = SpecialTags::from(&(e).name);
                    tag_strings.push(format!("<{}", open_tag_name));

                    let class = match open_tag_name {
                        SpecialTags::Wave => "text-wave",
                        SpecialTags::Shake => "text-shake",
                        SpecialTags::Spoiler => "text-spoiler",
                        _ => "",
                    };
                    tag_strings.push(format!("class=\"{class}\""));

                    if SpecialTags::Color == open_tag_name {
                        let c = match e.attributes.get("color") {
                            Some(pcc) => match pcc {
                                Some(color) => encode_minimal(color),
                                None => {
                                    return Err(CompilerError::BadAttr {
                                        attribute: "color".to_string(),
                                        value: "None".to_string(),
                                        why: "Expected".to_string()
                                    })
                                }
                            },
                            None => {
                                return Err(CompilerError::NoAttr {
                                    attribute: "color".to_string(),
                                })
                            }
                        };
                        tag_strings.push(format!("style=\"color:{c};\""));
                    }

                    tag_strings.push(">".to_string());

                    for child in e.children {
                        let stringed = HtmlNodeWrapper { inner: child }.to_string()?;

                        tag_strings.push(stringed);
                    }

                    tag_strings.push(open_tag_name.closer());

                    tag_strings
                        .into_iter()
                        .map(|x| {
                            if x.starts_with("class")
                                || x.starts_with("href")
                                || x.starts_with("style")
                            {
                                return format!(" {x} ");
                            }
                            x
                        })
                        .join("")
                } else if e.name == "message" {
                    let mut tag_strings = Vec::with_capacity(e.children.len());
                    for child in e.children {
                        let stringed = HtmlNodeWrapper { inner: child }.to_string()?;

                        tag_strings.push(stringed);
                    }

                    tag_strings
                        .into_iter()
                        .map(|x| {
                            if x.starts_with("class")
                                || x.starts_with("href")
                                || x.starts_with("style")
                            {
                                return format!(" {x} ");
                            }
                            x
                        })
                        .join("")
                } else if e.name == "br" {
                    "<br>".to_string()
                } else {
                    warn!("Ignored Tag {}!", e.name);
                    "".to_string()
                }
            }
            Node::Comment(_) => "".to_string(),
        };

        Ok(write_str)
    }
}

impl From<Node> for HtmlNodeWrapper {
    fn from(n: Node) -> Self {
        HtmlNodeWrapper { inner: n }
    }
}

impl Deref for HtmlNodeWrapper {
    type Target = Node;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for HtmlNodeWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
