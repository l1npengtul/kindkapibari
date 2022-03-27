use sea_orm::sea_query::{ColumnType, ValueType, ValueTypeErr};
use sea_orm::Value;
#[cfg(feature = "server")]
use sea_orm::{QueryResult, TryGetError, TryGetable};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Debug, Display, Formatter};

#[derive(Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum OnlyThree {
    Man,
    Woman,
    NonBinary,
}

impl Debug for OnlyThree {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                OnlyThree::Man => "man",
                OnlyThree::Woman => "woman",
                OnlyThree::NonBinary => "nb",
            }
        )
    }
}

impl Display for OnlyThree {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl From<Gender> for OnlyThree {
    fn from(gender: Gender) -> Self {
        match gender {
            Gender::Man => OnlyThree::Man,
            Gender::Woman => OnlyThree::Woman,
            _ => OnlyThree::NonBinary,
        }
    }
}

impl Serialize for OnlyThree {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for OnlyThree {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let ot_str = String::deserialize(deserializer)?;
        Ok(Self::from(Gender::from(ot_str))) // the laziness kicks in
    }
}

// reduction meant for coconutpak
#[derive(Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum Gender {
    Man,
    Woman,
    Nonbinary,
    Custom(String),
}

impl Debug for Gender {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Gender::Man => "man",
                Gender::Woman => "woman",
                Gender::Nonbinary => "nb",
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
            "non-binary" => Gender::Nonbinary,
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
