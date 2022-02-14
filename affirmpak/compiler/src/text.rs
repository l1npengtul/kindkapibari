#[derive(Clone, Debug, Default, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct TextContainer {
    sub_namespace: String,
    language: String,
    description: String,
    messages: Vec<String>,
}

impl TextContainer {
    pub fn sub_namespace(&self) -> &str {
        &self.sub_namespace
    }
    pub fn language(&self) -> &str {
        &self.language
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn messages(&self) -> &Vec<String> {
        &self.messages
    }

    pub fn set_sub_namespace(&mut self, sub_namespace: String) {
        self.sub_namespace = sub_namespace;
    }
    pub fn set_language(&mut self, language: String) {
        self.language = language;
    }
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }
    pub fn set_messages(&mut self, messages: Vec<String>) {
        self.messages = messages;
    }
}
