#[derive(Clone, Debug, Default, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Message {
    pub message: String,
    pub wait_after: f32,
}

#[derive(Clone, Debug, Default, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Response {
    pub name: String,
    pub messages: Vec<Message>,
    pub probability: f32,
    pub usable_for_welcome: bool,
}
