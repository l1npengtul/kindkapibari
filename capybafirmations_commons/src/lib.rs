#![deny(clippy::pedantic)]
#![warn(clippy::all)]

#[macro_use]
extern crate sqlx;
#[macro_use]
extern crate serde_derive;

pub mod affirmpak;
pub mod languages;
pub mod preferences;
pub mod pronouns;
pub mod responses;
pub mod tags;
pub mod user;
