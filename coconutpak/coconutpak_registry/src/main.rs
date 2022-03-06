#![deny(clippy::pedantic)]
#![warn(clippy::all)]
#![feature(thread_is_running)]

mod api;
mod coconutpak_cleanup;
mod coconutpak_compiler;
mod config;
mod schema;

#[macro_use]
extern crate sea_orm;
extern crate core;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
}
