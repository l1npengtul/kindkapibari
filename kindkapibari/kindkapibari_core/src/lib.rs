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
pub mod secret_gen;
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
