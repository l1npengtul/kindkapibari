#![deny(clippy::pedantic)]
#![warn(clippy::all)]

mod api;
mod config;
mod context;
mod schema;
mod scopes;
mod login;
mod access;

use crate::config::ServerConfig;
use crate::context::ApiContext;
use sea_orm::Database;
use sled::Db;
use std::time::Duration;
use std::{iter::Once, sync::Arc};
use tokio::{fs::File, io::AsyncReadExt, sync::OnceCell};

#[tokio::main]
async fn main() {}
