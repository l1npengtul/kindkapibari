#![deny(clippy::pedantic)]
#![warn(clippy::all)]

mod api;
mod coconutpak_compiler;
mod config;
mod schema;

#[macro_use]
extern crate sea_orm;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
}
