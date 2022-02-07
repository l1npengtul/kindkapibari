#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Serialize, Deserialize)]
pub enum Languages {
    AllNone,
    En,
    Ko,
    Ja,
    ZhTw,
    ZhCn,
    Other(String), // Your language is not real. Wake up.
}
