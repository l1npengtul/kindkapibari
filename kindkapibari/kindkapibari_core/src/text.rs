use crate::responses::Response;
use crate::tags::Tags;
use language_tags::LanguageTag;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TextContainer {
    sub_namespace: String,
    tags: Tags,
    language: LanguageTag,
    description: String,
    responses: Vec<Response>,
}

impl TextContainer {
    #[must_use]
    pub fn new(
        sub_namespace: String,
        tags: Tags,
        description: String,
        language: LanguageTag,
        responses: Vec<Response>,
    ) -> Self {
        TextContainer {
            sub_namespace,
            tags,
            language,
            description,
            responses,
        }
    }

    #[must_use]
    pub fn sub_namespace(&self) -> &str {
        &self.sub_namespace
    }
    #[must_use]
    pub fn language(&self) -> &LanguageTag {
        &self.language
    }
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }
    #[must_use]
    pub fn responses(&self) -> &[Response] {
        &self.responses
    }

    pub fn set_sub_namespace(&mut self, sub_namespace: String) {
        self.sub_namespace = sub_namespace;
    }
    pub fn set_language(&mut self, language: LanguageTag) {
        self.language = language;
    }
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }
    pub fn set_responses(&mut self) -> &mut Vec<Response> {
        &mut self.responses
    }
}
