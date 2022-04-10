use std::ffi::OsString;
use thiserror::Error;
use toml::ser::Error;

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd, Error)]
pub enum CompilerError {
    #[error("Disallowed Character in {field}: {bad_char}")]
    InvalidCharacters { field: String, bad_char: String },
    #[error("Bad {field}: {why}")]
    BadField { field: String, why: String },
    #[error("Cannot Find or Open Source Path!")]
    SourcePathInvalid,
    #[error("Cannot find or open file {file}, {why}")]
    FileError { file: String, why: String },
    #[error("Invalid Manifest: {}")]
    BadManifest(String),
    #[error("Invalid Text: {why}")]
    BadText { why: String },
    #[error("Invalid Attribute: attribute {attribute} of value {value}, Error: {why}")]
    BadAttr {
        attribute: String,
        value: String,
        why: String,
    },
    #[error("No Attribute: {attribute}")]
    NoAttr { attribute: String },
    #[error("Failed to generate XML: {why}")]
    XmlError { why: String },
    #[error("Error: {0}")]
    CompileError(String),
}

#[derive(Clone, Debug, Error)]
pub enum ConfigError {
    #[error("Config file not found: {0:?}")]
    ConfigNotFound(OsString),
    #[error("Bad Config File: {why}")]
    InvalidConfigFile { why: String },
    #[error("Config File Error: {why}")]
    ConfigFileError { why: String },
}

impl From<toml::ser::Error> for ConfigError {
    fn from(toml_err: Error) -> Self {
        ConfigError::InvalidConfigFile {
            why: toml_err.to_string(),
        }
    }
}

impl From<std::io::Error> for ConfigError {
    fn from(io_error: std::io::Error) -> Self {
        ConfigError::ConfigFileError {
            why: io_error.to_string(),
        }
    }
}
