use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum KKBCoreError {
    #[error("Error creating the template: {0}")]
    TemplateInit(String),
}
