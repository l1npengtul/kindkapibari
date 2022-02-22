use std::ffi::{OsStr, OsString};
use std::io;
use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum CompilerError {
    #[error("Cannot Find or Open Source Path!")]
    SourcePathInvalid,
    #[error("Cannot find or open file {file}, {why}")]
    FileError { file: String, why: String },
    #[error("Invalid lib.json!")]
    BadLibJson,
    #[error("Invalid Manifest!")]
    BadManifest,
    #[error("Invalid Text File {file}: {why}")]
    BadText { file: String, why: String },
    #[error("Invalid Attribute: attribute {attribute} of value {value}, Error: {why}")]
    BadAttr {
        attribute: String,
        value: String,
        why: String,
    },
    #[error("No Attribute: {attribute}")]
    NoAttr { attribute: String },
    #[error("Failed to generate XML for {file}: {why}")]
    XmlError { file: String, why: String },
    #[error("Error: {0}")]
    CompileError(String),
}

#[derive(Clone, Debug, Error)]
pub enum ConfigError {
    #[error("Config file not found: {0}")]
    ConfigNotFound(OsString),
    #[error("Bad Config File at {path}: {why}")]
    InvalidConfigFile { path: OsString, why: String },
    #[error("Config File Error: {path}: {why}")]
    ConfigFileError { path: OsString, why: String },
}
