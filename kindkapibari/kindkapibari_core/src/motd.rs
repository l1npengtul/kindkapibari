#[derive(Clone, Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct MessageOfTheDay {
    pub color: String,
    pub text: String,
    pub has_button: bool,
    pub button_label: Option<String>,
    pub button_link: Option<String>,
}
