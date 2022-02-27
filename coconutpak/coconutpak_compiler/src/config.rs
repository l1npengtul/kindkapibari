use crate::error::ConfigError;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::File,
    io::{self, Read, Write},
    str::FromStr,
};
use toml::{de::Error, to_string};
use url::Url;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    logins: HashMap<String, String>,
}

impl Config {
    pub fn load() -> Result<Config, ConfigError> {
        let mut config_path = dirs::config_dir().unwrap();
        config_path.push("config.toml");
        let mut data = String::new();

        match File::open(config_path) {
            Ok(mut handle) => {
                if let Err(why) = handle.read_to_string(&mut data) {
                    return Err(ConfigError::ConfigFileError {
                        path: config_path.into_os_string(),
                        why: why.to_string(),
                    });
                }
            }
            Err(_) => return Err(ConfigError::ConfigNotFound(config_path.into_os_string())),
        }
        toml::from_str::<Self>(&data).map_err(|err| ConfigError::InvalidConfigFile {
            path: config_path.into_os_string(),
            why: err.to_string(),
        })
    }

    pub fn save(&self) -> Result<(), ConfigError> {
        let mut config_path = dirs::config_dir().unwrap();
        config_path.push("config.toml");
        let data = toml::to_string_pretty(&self)?.into_bytes();
        File::create(config_path)?.write(&data)?;
        Ok(())
    }

    pub fn get_login_for_url(&self, url: impl AsRef<str>) -> Option<&String> {
        self.logins.get(url.as_ref())
    }

    pub fn add_login_for_url(
        &mut self,
        url: impl AsRef<str>,
        apikey: impl AsRef<str>,
    ) -> Option<String> {
        self.logins
            .insert(url.as_ref().to_string(), apikey.as_ref().to_string())
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            logins: HashMap::new(),
        }
    }
}
