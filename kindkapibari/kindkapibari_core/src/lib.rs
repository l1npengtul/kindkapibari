#![deny(clippy::pedantic)]
#![warn(clippy::all)]

#[macro_use]
extern crate serde;

#[cfg(feature = "server")]
pub mod dbarray;
#[cfg(feature = "server")]
pub mod dbvec;
pub mod error;
pub mod gender;
pub mod language;
pub mod license;
pub mod manifest;
pub mod motd;
pub mod output;
pub mod preferences;
pub mod pronouns;
pub mod reseedingrng;
pub mod responses;
pub mod templater;
pub mod text;
pub mod user_data;
pub mod version;
