pub mod defaults;

use crate::configs::default_configs::DEFAULT_CONFIG_FILE_PATH;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Serialize, Deserialize, Debug)]
pub struct RepodConfig {
    pub defaults: Defaults,
    pub repos: Vec<RepositoryConfig>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Defaults {
    pub r#type: Option<String>,
    pub branch: Option<String>,
    pub server: Option<String>,
    pub port: Option<String>,
    pub root_path: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RepositoryConfig {
    pub name: String,
    pub command: String,
    pub r#type: Option<String>,
    pub branch: Option<String>,
    pub server: Option<String>,
    pub port: Option<String>,
    pub root_path: Option<String>,
}

impl RepodConfig {
    pub fn new() -> Self {
        RepodConfig {
            defaults: Defaults {
                r#type: None,
                branch: None,
                server: None,
                port: None,
                root_path: None,
            },
            repos: Vec::new(),
        }
    }

    pub fn from_config_file_sync(
        config_file_path: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let configs_str = if Path::new(config_file_path).exists() {
            fs::read_to_string(config_file_path).expect(
                format!(
                    "Something went wrong reading the config file {}",
                    config_file_path
                )
                .as_str(),
            )
        } else {
            String::from("")
        };
        let config = toml::from_str(configs_str.as_str())?;
        Ok(config)
    }

    pub fn from_default_config_file_sync() -> Result<Self, Box<dyn std::error::Error>> {
        Self::from_config_file_sync(DEFAULT_CONFIG_FILE_PATH)
    }

    pub fn to_config_file_sync(
        &self,
        config_file_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let config_str = toml::to_string(&self)?;
        fs::write(config_file_path, config_str)?;
        Ok(())
    }

    pub fn to_default_config_file_sync(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.to_config_file_sync(DEFAULT_CONFIG_FILE_PATH)
    }

    pub fn add_repo(&mut self, repo: RepositoryConfig) {
        self.repos.push(repo);
    }
}

impl RepositoryConfig {
    pub fn new(
        name: String,
        command: String,
        r#type: Option<String>,
        branch: Option<String>,
        server: Option<String>,
        port: Option<String>,
        root_path: Option<String>,
    ) -> Self {
        RepositoryConfig {
            name,
            command,
            r#type,
            branch,
            server,
            port,
            root_path,
        }
    }
}
