#![deny(clippy::pedantic)]
#![warn(clippy::all)]

#[cfg(feature = "server")]
#[macro_use]
extern crate sea_orm;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde;

pub mod affirmpak;
pub mod languages;
pub mod preferences;
pub mod pronouns;
pub mod responses;
pub mod tags;
pub mod user;
