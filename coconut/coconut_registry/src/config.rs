use color_eyre::eyre::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerConfig {
    pub postgres: PostgresSQL,
    pub port: u16,
    pub github: GithubLogin,
}

impl ServerConfig {
    pub fn save(&self) -> Result<()> {
        let mut config_file = File::create("Config.toml")?;
        config_file.set_len(0)?;
        let data = toml::to_vec(&self)?;
        config_file.write_all(&data)?;
        Ok(())
    }

    pub fn load() -> Result<Self> {
        let mut config_file = File::open("Config.toml")?;
        let mut bytes = Vec::new();
        config_file.read_exact(&mut bytes)?;
        Ok(toml::from_slice(&bytes)?)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Files {
    pub static_serve_location: String,
    pub compile_target_directory: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GithubLogin {
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostgresSQL {
    pub url: String,
}
