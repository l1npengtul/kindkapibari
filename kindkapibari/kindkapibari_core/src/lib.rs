#![deny(clippy::pedantic)]
#![warn(clippy::all)]

#[macro_use]
extern crate serde;

#[cfg(feature = "server")]
pub mod dbarray;
#[cfg(feature = "server")]
pub mod dbhasmap;
#[cfg(feature = "server")]
pub mod dbvec;
pub mod error;
pub mod gender;
pub mod language;
pub mod license;
pub mod manifest;
pub mod motd;
pub mod output;
#[cfg(feature = "server")]
pub mod permissions;
pub mod preferences;
pub mod pronouns;
pub mod reseedingrng;
pub mod responses;
#[cfg(feature = "server")]
pub mod secret;
#[cfg(feature = "server")]
pub mod snowflake;
#[cfg(feature = "server")]
pub mod state;
pub mod tags;
pub mod templater;
pub mod text;
#[cfg(feature = "server")]
pub mod throttle;
pub mod user_data;
pub mod version;
#[cfg(feature = "server")]
#[macro_use]
pub mod db_impl;
pub mod reminder;
pub mod sober;
pub use kindkapibari_proc::AttrString;

pub trait AttrErr {
    type ParseError;
}

#[macro_export]
macro_rules! impl_attr_err {
    ($($toimpl:ty),*) => {
        $(
            impl kindkapibari_core::AttrErr for $toimpl {
                type ParseError = kindkapibari_core::ParseEnumError;
            }
        )*
    };
}

#[derive(Debug, thiserror::Error)]
pub enum ParseEnumError {
    #[error("Failed to parse {0}")]
    FailToParse(String),
}

impl From<String> for ParseEnumError {
    fn from(s: String) -> Self {
        ParseEnumError::FailToParse(s)
    }
}
