#[derive(Clone, Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct MessageOfTheDay {
    pub color: String,
    pub text: String,
    pub button_link: Option<String>,
}
