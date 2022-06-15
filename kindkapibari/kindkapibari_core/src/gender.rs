#[cfg(feature = "server")]
use sea_orm::{
    sea_query::{ColumnType, ValueType, ValueTypeErr},
    QueryResult, TryGetError, TryGetable, Value,
};
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

// reduction meant for coconutpak
#[derive(Clone, Default, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum Gender {
    Man,
    Woman,
    #[default]
    NonBinary,
    Custom(String), // upload custom gender. max 10 MB /s
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

impl Serialize for Gender {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Gender {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let gender_str = String::deserialize(deserializer)?;
        Ok(Self::from(gender_str))
    }
}

#[cfg(feature = "server")]
impl TryGetable for Gender {
    fn try_get(res: &QueryResult, pre: &str, col: &str) -> Result<Self, TryGetError> {
        Ok(Gender::from(String::try_get(res, pre, col)?))
    }
}

#[cfg(feature = "server")]
impl From<Gender> for sea_orm::Value {
    fn from(g: Gender) -> Self {
        sea_orm::Value::String(Some(Box::new(g.to_string())))
    }
}

#[cfg(feature = "server")]
impl ValueType for Gender {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        match v {
            Value::String(Some(gender_str)) => Ok(Gender::from(gender_str.as_ref().as_str())),
            _ => Err(ValueTypeErr),
        }
    }

    fn type_name() -> String {
        stringify!(Gender).to_string()
    }

    fn column_type() -> ColumnType {
        ColumnType::Text
    }
}
