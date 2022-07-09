use color_eyre::eyre::Result;
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{Read, Write},
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    pub machine_id: u8,
    #[serde(default = "default_port")]
    pub port: u16,
    pub database: Database,
    pub host_url: String,
    pub signing_keys: SigningKeys,
    pub oauth: OAuthProviders,
    pub others: Others,
}

impl Config {
    pub fn save(&self) -> Result<()> {
        let mut config_file = File::create("config.toml")?;
        config_file.set_len(0)?;
        let data = toml::to_vec(&self)?;
        config_file.write_all(&data)?;
        Ok(())
    }

    pub fn load() -> Result<Self> {
        let mut config_file = File::open("config.toml")?;
        let mut bytes = Vec::new();
        config_file.read_to_end(&mut bytes)?;
        Ok(toml::from_slice(&bytes)?)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Files {
    #[serde(default = "default_static_serve_location")]
    pub static_serve_location: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Database {
    pub postgres_url: String,
    #[serde(default = "default_max_threads")]
    pub postgres_pool: u32,
    #[serde(default = "default_sled_store_path")]
    pub sled_store_path: String,
    pub meilisearch_url: String,
    pub meilisearch_passwd: String,
    pub redis_url: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Eq, Deserialize)]
pub struct OAuthProviders {
    pub default_time: u64,
    pub redirect_url: String,
    pub twitter: OAuth,
    pub github: OAuth,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OAuth {
    pub client_id: String,
    pub secret: String,
    pub authorize_url: String,
    pub token_url: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SigningKeys {
    pub oauth_key: String,
    pub oauth_thirdparty_key: String,
    pub login_key: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Others {
    pub auth_url: String,
}

const fn default_port() -> u16 {
    3160
}

fn default_static_serve_location() -> String {
    "static".to_string()
}

const fn default_max_threads() -> u32 {
    4
}

fn default_sled_store_path() -> String {
    "sled".to_string()
}
