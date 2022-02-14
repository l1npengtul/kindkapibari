#![deny(clippy::pedantic)]
#![warn(clippy::all)]

#[macro_use]
extern crate serde;
#[cfg(feature = "server")]
#[macro_use]
extern crate sea_orm;

pub mod affirmpak;
pub mod languages;
pub mod preferences;
pub mod pronouns;
pub mod responses;
pub mod tags;
pub mod user;
mod version;
