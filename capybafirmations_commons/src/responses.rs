use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, Default, PartialOrd, Serialize, Deserialize)]
pub struct Response {
    contents: Vec<String>,
    timings: Vec<f32>,
}

impl PartialEq for Response {
    fn eq(&self, other: &Self) -> bool {
        self.contents.eq(&other.contents)
    }
}

impl Hash for Response {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.contents.hash(state);
    }
}
