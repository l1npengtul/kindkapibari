#![deny(clippy::pedantic)]
#![warn(clippy::all)]

#[macro_use]
extern crate serde_derive;
#[cfg(feature = "server")]
#[macro_use]
extern crate diesel;

pub mod affirmpak;
pub mod languages;
pub mod preferences;
pub mod pronouns;
pub mod responses;
pub mod tags;
pub mod user;
mod version;
