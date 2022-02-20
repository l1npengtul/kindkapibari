use crate::error::CompilerError;
use affirmpak_core::{
    libjson::LibJson, manifest::AffirmPakManifest, output::Output, text::TextContainer,
};
use bson::{document::ValueAccessResult, Document};
use capybafirmations_commons::languages::Languages;
use html_parser::{Dom, Element, Node};
use itertools::Itertools;
use log::warn;
use serde_json::from_str;
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::fmt::write;
use std::{
    fmt::{Display, Formatter},
    fs::File,
    io::Read,
    ops::{Deref, DerefMut},
    path::Path,
};
use xml_dom::level2::{convert::as_document_mut, get_implementation};

pub struct Compiler {
    manifest: AffirmPakManifest,
    lib: LibJson,
    source_path: String,
}

impl Compiler {
    pub fn new(manifest: AffirmPakManifest, lib: LibJson, source_path: String) -> Self {
        Compiler {
            manifest,
            lib,
            source_path,
        }
    }

    pub fn compiler(&self) -> Result<Output, CompilerError> {
        let mut source_path = Path::new(&self.source_path);
        if !source_path.is_dir() {
            return Err(CompilerError::SourcePathInvalid);
        }

        // open libjson and see what's inside
        for text_to_compile in self.lib.texts() {
            let text_path = Path::new(text_to_compile);
            let total_path = source_path.join(text_path);
            let mut container = match File::open(total_path) {
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
                                let mut xml_output_builder = match get_implementation()
                                    .create_document(None, Some("resp"), None)
                                {
                                    Ok(doc) => doc,
                                    Err(why) => {
                                        return Err(CompilerError::XmlError {
                                            file: total_path
                                                .as_os_str()
                                                .to_string_lossy()
                                                .to_string(),
                                            why: why.to_string(),
                                        })
                                    }
                                };
                                let document = as_document_mut(&mut xml_output_builder).unwrap();
                                let mut root_node = document.document_element().unwrap();

                                let dom_parse = match Dom::parse(resp_arr) {
                                    Ok(d) => {
                                        // ignore all root tags that arnt <response>
                                        d.children.into_iter().filter_map(|n| match n {
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
                                        })
                                            .collect::<Vec<Element>>()
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

                                let mut stack = vec![dom_parse[0]];
                                loop {
                                    let element = match stack.pop() {
                                        Some(e) => e,
                                        None => break,
                                    };
                                }
                            }
                            Err(why) => {
                                return Err(CompilerError::BadText {
                                    file: total_path.as_os_str().to_string_lossy().to_string(),
                                    why: why.to_string(),
                                })
                            }
                        };
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

            container.messages().into_iter()
        }

        Err("".to_string())
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
    Link,
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
                SpecialTags::Link => "a",
            }
        )
    }
}

#[derive(Clone, PartialEq)]
struct HtmlNodeWrapper {
    inner: Node,
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

impl Display for HtmlNodeWrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let write_str = match &self.inner {
            Node::Text(t) => t.clone(),
            Node::Element(e) => {
                let mut tag_strings = Vec::with_capacity(5);

                let open_tag_name = match e.name.as_str() {
                    "color" => "span",
                    t => t,
                };
                tag_strings.push(format!("<{}>", e.name));
            }
            Node::Comment(_) => "".to_string(),
        };
    }
}
