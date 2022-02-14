use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum CompilerError {
    #[error("Cannot Find or Open Source Path!")]
    SourcePathInvalid,
    #[error("Cannot find or open file {file}, {why}!")]
    FileError {
        file: String,
        why: String,
    },
    #[error("Invalid lib.json!")]
    BadLibJson,
    #[error("Invalid Manifest!")]
    BadManifest,
    #[error("Invalid Text File: {0}")]
    BadText(String),
    #[error("Error: {0}")]
    CompileError(String),
}
