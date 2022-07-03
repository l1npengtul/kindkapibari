#[cfg(feature = "server")]
use sea_orm::{
    sea_query::{ColumnType, ValueType, ValueTypeErr},
    DbErr, QueryResult, TryGetError, TryGetable, Value,
};
use semver::{BuildMetadata, Prerelease, Version as SemVer};
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Version {
    version: SemVer,
}

impl Version {
    #[must_use]
    pub const fn new(major: u64, minor: u64, patch: u64) -> Self {
        Version {
            version: SemVer {
                major,
                minor,
                patch,
                pre: Prerelease::EMPTY,
                build: BuildMetadata::EMPTY,
            },
        }
    }

    pub fn parse(text: &str) -> Result<Self, semver::Error> {
        Ok(Version {
            version: SemVer::parse(text)?,
        })
    }
}

impl Deref for Version {
    type Target = SemVer;

    fn deref(&self) -> &Self::Target {
        &self.version
    }
}

impl DerefMut for Version {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.version
    }
}

#[cfg(feature = "server")]
impl TryGetable for Version {
    fn try_get(res: &QueryResult, pre: &str, col: &str) -> Result<Self, TryGetError> {
        SemVer::from_str(&String::try_get(res, pre, col)?)
            .map(|x| Version { version: x })
            .map_err(|why| TryGetError::DbErr(DbErr::Custom(why.to_string())))
    }
}

#[cfg(feature = "server")]
impl From<Version> for sea_orm::Value {
    fn from(ver: Version) -> Self {
        sea_orm::Value::String(Some(Box::new(ver.to_string())))
    }
}

#[cfg(feature = "server")]
impl ValueType for Version {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        match v {
            Value::String(Some(version_str)) => SemVer::from_str(&version_str)
                .map(|x| Version { version: x })
                .map_err(|_| ValueTypeErr),
            _ => Err(ValueTypeErr),
        }
    }

    fn type_name() -> String {
        stringify!(Version).to_string()
    }

    fn column_type() -> ColumnType {
        ColumnType::Text
    }
}
