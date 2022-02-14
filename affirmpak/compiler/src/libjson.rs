#[derive(Clone, Debug, Default, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct LibJson {
    texts: Vec<String>,
    themes: Vec<String>,
}

impl LibJson {
    pub fn texts(&self) -> &Vec<String> {
        &self.texts
    }
    pub fn themes(&self) -> &Vec<String> {
        &self.themes
    }
}
