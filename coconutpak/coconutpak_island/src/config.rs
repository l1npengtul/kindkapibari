use color_eyre::eyre::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub machine_id: u8,
    #[serde(default = "default_port")]
    pub port: u16,
    pub compiler: Compiler,
    pub database: Database,
    pub other_coconutpak: OtherCoconutPak,
    pub host_url: String,
    pub oauth: KindKapiBariOAuth,
    pub signing_key: String,
    #[serde(default = "default_kkb_login")]
    pub support_official_kkb_login: bool,
}

impl Config {
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
    #[serde(default = "default_static_serve_location")]
    pub static_serve_location: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Compiler {
    #[serde(default = "default_core_threads")]
    pub core_threads: usize,
    #[serde(default = "default_max_threads")]
    pub max_threads: usize,
    #[serde(default = "default_wtsa")]
    pub worker_thread_stay_alive: usize,
    #[serde(default = "default_mpts")]
    pub max_pak_time_s: usize,
    #[serde(default = "default_compiler_location")]
    pub compiler_location: String,
    #[serde(default = "default_compile_target_directory")]
    pub compile_target_directory: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OtherCoconutPak {
    pub statistics_service_address: String,
    pub statistics_service_secret: String,
    pub build_service_address: String,
    pub build_service_secret: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KindKapiBariOAuth {
    pub client_id: String,
    pub client_secret: String,
    pub authorize_url: String,
    pub token_url: String,
}

const fn default_port() -> u16 {
    3160
}

const fn default_kkb_login() -> bool {
    false
}

fn default_static_serve_location() -> String {
    "static".to_string()
}

fn default_compile_target_directory() -> String {
    "compile".to_string()
}

const fn default_core_threads() -> usize {
    2
}
const fn default_max_threads() -> usize {
    4
}
const fn default_wtsa() -> usize {
    1000
}
const fn default_mpts() -> usize {
    60
}

fn default_compiler_location() -> String {
    "coconutpak".to_string()
}

fn default_sled_store_path() -> String {
    "sled".to_string()
}
