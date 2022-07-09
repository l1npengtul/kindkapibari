use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Debug, Display, Formatter};

#[derive(Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum AsThree {
    Man,
    Woman,
    NonBinary,
}

impl Debug for AsThree {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AsThree::Man => "man",
                AsThree::Woman => "woman",
                AsThree::NonBinary => "nb",
            }
        )
    }
}

impl Display for AsThree {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl From<Gender> for AsThree {
    fn from(gender: Gender) -> Self {
        match gender {
            Gender::Man => AsThree::Man,
            Gender::Woman => AsThree::Woman,
            _ => AsThree::NonBinary,
        }
    }
}

impl Serialize for AsThree {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for AsThree {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let ot_str = String::deserialize(deserializer)?;
        Ok(Self::from(Gender::from(ot_str))) // the laziness kicks in
    }
}

#[cfg_attr(feature = "server", derive(utoipa::Component))]
#[derive(Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum Gender {
    Man,
    Woman,
    NonBinary,
    Custom(String), // upload custom gender. max 10 MB
}

impl Default for Gender {
    fn default() -> Self {
        Gender::Custom("diyhrt.github.io gender :)".to_string())
    }
}

impl Debug for Gender {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Gender::Man => "man",
                Gender::Woman => "woman",
                Gender::NonBinary => "nb",
                Gender::Custom(s) => s.as_str(),
            }
        )
    }
}

impl Display for Gender {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl<S> From<S> for Gender
where
    S: AsRef<str>,
{
    fn from(s: S) -> Self {
        match s.as_ref() {
            "man" => Gender::Man,
            "woman" => Gender::Woman,
            "non-binary" => Gender::NonBinary,
            g => Gender::Custom(g.to_owned()),
        }
    }
}

#[cfg(feature = "server")]
crate::impl_sea_orm!(Gender, AsThree);
#[cfg(feature = "server")]
crate::impl_redis!(Gender, AsThree);
