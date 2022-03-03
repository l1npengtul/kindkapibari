use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, Hash, PartialOrd, PartialEq, Serialize, Deserialize)]
pub enum Languages {
    AllNone,
    En,
    Ko,
    Ja,
    ZhTw,
    ZhCn,
    Other(String), // Your language is not real. Wake up.
}

impl Debug for Languages {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let lang = match self {
            Languages::AllNone => "N/A",
            Languages::En => "en",
            Languages::Ko => "ko",
            Languages::Ja => "ja",
            Languages::ZhTw => "zh_tw",
            Languages::ZhCn => "zh_cn",
            Languages::Other(e) => e,
        };

        write!(f, "{lang}")
    }
}

impl Display for Languages {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl<S> From<S> for Languages
where
    S: AsRef<str>,
{
    fn from(s: S) -> Self {
        match s.as_ref() {
            "N/A" => Self::AllNone,
            "en" => Self::En,
            "ko" => Self::Ko,
            "ja" => Self::Ja,
            "zh_tw" => Self::ZhTw,
            "zh_cn" => Self::ZhCn,
            other => Self::Other(other.to_string()),
        }
    }
}
