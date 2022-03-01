use capybafirmations_commons::{languages::Languages, responses::Response};

#[derive(Clone, Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct TextContainer {
    sub_namespace: String,
    language: Languages,
    description: String,
    responses: Vec<Response>,
}

impl TextContainer {
    pub fn new(
        sub_namespace: String,
        language: Languages,
        description: String,
        responses: Vec<Response>,
    ) -> Self {
        TextContainer {
            sub_namespace,
            language,
            description,
            responses,
        }
    }

    pub fn sub_namespace(&self) -> &str {
        &self.sub_namespace
    }
    pub fn language(&self) -> &Languages {
        &self.language
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn responses(&self) -> &[Response] {
        &self.responses
    }

    pub fn set_sub_namespace(&mut self, sub_namespace: String) {
        self.sub_namespace = sub_namespace;
    }
    pub fn set_language(&mut self, language: Languages) {
        self.language = language;
    }
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }
    pub fn set_responses(&mut self) -> &mut Vec<Response> {
        &mut self.responses
    }
}