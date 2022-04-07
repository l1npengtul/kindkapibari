use crate::error::CompilerError;
use escaper::encode_minimal;
use html_parser::{Dom, Node};
use itertools::Itertools;
use log::warn;
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
use kindkapibari_core::output::CoconutPakOutput;
use kindkapibari_core::text::TextContainer;

const ALLOWED_TAGS: [&str; 20] = [
    "CoconutPakAsset",
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

        let mut manifest = match toml::from_str::<CoconutPakM
            anifest>(&read_string_manifest) {
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

        // open libjson and see what's inside
        let mut texts = vec![];
        for text_to_compile in self.lib.texts() {
            let text_path = Path::new(text_to_compile);
            let total_path = source_path.join(text_path);
            let file_path = total_path.as_os_str().to_string_lossy().to_string();
            texts.push(container);
        }

        Ok(CoconutPakOutput {
            edition: self.manifest.version.clone(),
            manifest: self.manifest,
            register_text_containers: {
                if !texts.is_empty() {
                    Some(texts)
                } else {
                    None
                }
            },
        })
    }

    fn compile_text(&self, file: std::path::PathBuf) -> Result<TextContainer, CompilerError> {
        let file_path = file.clone().to_string_lossy().to_string();

        // lint

        let dom = match File::open(file) {
            Ok(mut f) => {
                let mut file_str = String::new();
                if let Err(why) = f.read_to_string(&mut file_str) {
                    return Err(CompilerError::FileError {
                        file: file_path.to_string(),
                        why: why.to_string(),
                    });
                }
                match Dom::parse(&file_str) {
                    Ok(d) => d,
                    Err(why) => {
                        return Err(CompilerError::BadText {
                            file: file_str.to_string(),
                            why: why.to_string(),
                        })
                    }
                }
            }
            Err(why) => {
                return Err(CompilerError::FileError {
                    file: file_path.to_string(),
                    why: why.to_string(),
                })
            }
        };



        // match File::open(file) {
        //     Ok(f) => match Document::from_reader(f) {
        //         Ok(doc) => {
        //             let sub_namespace = match doc.get_str("subnamespace") {
        //                 Ok(sns) => sns.to_owned(),
        //                 Err(why) => {
        //                     return Err(CompilerError::BadText {
        //                         file: file_path,
        //                         why: why.to_string(),
        //                     })
        //                 }
        //             };
        //             let language_code = match doc.get_str("langcode") {
        //                 Ok(lang_str) => match LanguageTag::parse(lang_str) {
        //                     Ok(lc) => lc,
        //                     Err(why) => {
        //                         return Err(CompilerError::BadText {
        //                             file: file_path,
        //                             why: why.to_string(),
        //                         })
        //                     }
        //                 },
        //                 Err(why) => {
        //                     return Err(CompilerError::BadText {
        //                         file: file_path,
        //                         why: why.to_string(),
        //                     })
        //                 }
        //             };
        //             let description = match doc.get_str("description") {
        //                 Ok(desc) => desc.to_owned(),
        //                 Err(why) => {
        //                     return Err(CompilerError::BadText {
        //                         file: file_path,
        //                         why: why.to_string(),
        //                     })
        //                 }
        //             };
        //             let responses = match doc.get_str("responses") {
        //                 Ok(resp_arr) => {
        //                     let dom_parse = match Dom::parse(resp_arr) {
        //                         Ok(d) => {
        //                             // ignore all root tags that arnt <resp>
        //                             let element_vec = d.children.into_iter().filter_map(|n| match n {
        //                                 Node::Element(mut e) => {
        //                                     let name = &e.name;
        //                                     if name != "resp" {
        //                                         warn!("Ignoring element \"{name}\" in file {file_path:?}, ignored element type.");
        //                                         None
        //                                     } else {
        //                                         e.children.retain(|e| {
        //                                             match e {
        //                                                 Node::Element(e) => {
        //                                                     e.name == "response"
        //                                                 }
        //                                                 _ => false
        //                                             }
        //                                         });
        //                                         Some(e)
        //                                     }
        //                                 }
        //                                 Node::Text(t) => {
        //                                     warn!("Ignoring text \"{t}\" in file {file_path:?}");
        //                                     None
        //                                 }
        //                                 _ => None,
        //                             }).collect::<Vec<Element>>();
        //
        //                             let counts = element_vec.len();
        //                             if counts != 1 {
        //                                 error!("In file {file_path}: More than one <resp> root tags. ({counts}).");
        //                                 return Err(CompilerError::BadText {
        //                                     file: file_path,
        //                                     why: "More than one <resp>".to_string(),
        //                                 });
        //                             }
        //
        //                             element_vec.get(0).unwrap().clone()
        //                         }
        //                         Err(why) => {
        //                             return Err(CompilerError::BadText {
        //                                 file: file_path,
        //                                 why: why.to_string(),
        //                             })
        //                         }
        //                     };
        //
        //                     dom_parse
        //                         .children
        //                         .into_iter()
        //                         .filter_map(|x| -> Option<Result<Response, CompilerError>> { match x {
        //                             Node::Element(e) => {
        //                                 if e.name != "response" {
        //                                     warn!("Ignoring {e:?} in file {file_path}");
        //                                     return None;
        //                                 }
        //
        //
        //                                 let probability = match e.attributes.get("probability").cloned().unwrap_or_else(|| Some("1.0".to_string())).as_deref().map(f32::from_str) {
        //                                     None => 1.0,
        //                                     Some(Ok(p)) => p,
        //                                     Some(Err(why)) => {
        //                                         return Some(Err(
        //                                             CompilerError::BadAttr {
        //                                                 attribute: "probability".to_string(),
        //                                                 value: format!("{:?}", e.attributes.get("probability")),
        //                                                 why: why.to_string()
        //                                             }
        //                                         ))
        //                                     }
        //                                 };
        //                                 let welcome = match e.attributes.get("welcome").cloned().unwrap_or_else(|| Some("false".to_string())).as_deref().map(str::parse::<bool>) {
        //                                     Some(Ok(b)) => b,
        //                                     Some(Err(why)) => return Some(Err(
        //                                         CompilerError::BadAttr {
        //                                             attribute: "welcome".to_string(),
        //                                             value: format!("{:?}", e.attributes.get("welcome")),
        //                                             why: why.to_string()
        //                                         }
        //                                     )),
        //                                     None => false,
        //                                 };
        //
        //                                 let messages = e.children.into_iter()
        //                                     .filter_map(|x| -> Option<Result<Message, CompilerError>> { match x {
        //                                         Node::Element(e) => {
        //                                             if e.name != "message" {
        //                                                 warn!("Ignoring {e:?} in file {file_path}");
        //                                                 return None
        //                                             }
        //
        //                                             let wait = match e.attributes.get("wait").cloned().unwrap_or_else(|| Some("0.0".to_string())).as_deref().map(f32::from_str) {
        //                                                 None => 1.0,
        //                                                 Some(Ok(p)) => p,
        //                                                 Some(Err(why)) => {
        //                                                     return Some(Err(
        //                                                         CompilerError::BadAttr {
        //                                                             attribute: "wait".to_string(),
        //                                                             value: format!("{:?}", e.attributes.get("wait")),
        //                                                             why: why.to_string()
        //                                                         }
        //                                                     ))
        //                                                 }
        //                                             };
        //
        //                                             let message_str = HtmlNodeWrapper { inner: Node::Element(e) }.to_string().ok()?;
        //
        //                                             Some(Ok(Message {
        //                                                 message: message_str,
        //                                                 wait_after: wait,
        //                                             }))
        //                                         }
        //                                         i => {
        //                                             warn!("Ignoring {i:?} in file {file_path}");
        //                                             None
        //                                         }
        //                                     } }).collect::<Result<Vec<Message>, CompilerError>>().ok()?;
        //
        //                                 Some(
        //                                     Ok(
        //                                         Response {
        //                                             messages,
        //                                             probability,
        //                                             usable_for_welcome: welcome,
        //                                         }
        //                                     )
        //                                 )
        //
        //                             }
        //                             i => {
        //                                 warn!("Ignoring {i:?} in file {file_path}");
        //                                 None
        //                             }
        //                         } })
        //                         .collect::<Result<Vec<Response>, CompilerError>>()?
        //                 }
        //                 Err(why) => {
        //                     return Err(CompilerError::BadText {
        //                         file: file_path,
        //                         why: why.to_string(),
        //                     })
        //                 }
        //             };
        //
        //             Ok(TextContainer::new(
        //                 sub_namespace,
        //                 description,
        //                 language_code,
        //                 responses,
        //             ))
        //         }
        //         Err(why) => {
        //             return Err(CompilerError::FileError {
        //                 file: file_path,
        //                 why: why.to_string(),
        //             });
        //         }
        //     },
        //     Err(why) => {
        //         return Err(CompilerError::FileError {
        //             file: file_path,
        //             why: why.to_string(),
        //         });
        //     }
        // }
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

#[derive(Clone, PartialEq)]
struct HtmlNodeWrapper {
    pub inner: Node,
}

impl HtmlNodeWrapper {
    pub fn to_string(&self) -> Result<String, CompilerError> {
        let write_str = match &self.inner {
            Node::Text(t) => encode_minimal(t),
            Node::Element(e) => {
                if ALLOWED_TAGS.contains(&e.name.as_ref()) {
                    // wtf???? lmao
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
                                        why: "Expected".to_string(),
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

                    for child in e.children.clone() {
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
                    for child in e.children.clone() {
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

fn check_name_good<S: AsRef<str>>(string: S) -> bool {
    if !string.replace(ALLOWED_CHARS, "").is_empty() || !string.starts_with(ALLOWED_START_CHARS) {
        false
    } else {
        true
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
