#![deny(clippy::pedantic)]
#![warn(clippy::all)]

#[macro_use]
extern crate sea_orm;
#[macro_use]
extern crate serde_derive;

mod affirmpak;
mod languages;
mod preferences;
mod pronouns;
mod responses;
mod tags;
mod user;
