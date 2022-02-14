use crate::error::CompilerError;
use crate::libjson::LibJson;
use crate::manifest::AffirmPakManifest;
use crate::text::TextContainer;
use bson::Document;
use serde_json::from_str;
use std::fs::File;
use std::io::Read;
use std::path::Path;

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

    pub fn compiler(&self) -> Result<Document, CompilerError> {
        let mut source_path = Path::new(&self.source_path);
        if !source_path.is_dir() {
            return Err(CompilerError::SourcePathInvalid);
        }

        // open libjson and see what's inside
        for text_to_compile in self.lib.texts() {
            let text_path = Path::new(text_to_compile);
            let total_path = source_path.join(text_path);
            let mut container = match File::open(total_path) {
                Ok(mut f) => {
                    let mut read_str = String::new();
                    if let Err(why) = f.read_to_string(&mut read_str) {
                        return Err(CompilerError::FileError {
                            file: total_path.as_os_str().to_string_lossy().to_string(),
                            why: why.to_string(),
                        });
                    }
                    match from_str::<TextContainer>(&read_str) {
                        Ok(t) => t,
                        Err(why) => return Err(CompilerError::BadText(why.to_string())),
                    }
                }
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
