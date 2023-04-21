pub mod defaults;

use crate::configs::default_configs::default_config_file_path;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RepodConfig {
    pub defaults: Option<Defaults>,
    pub repos: Option<Vec<RepositoryConfig>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Defaults {
    pub r#type: Option<String>,
    pub branch: Option<String>,
    pub server: Option<String>,
    pub port: Option<String>,
    pub root_path: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RepositoryConfig {
    pub name: String,
    pub commands: Vec<String>,
    pub updates: Vec<String>,
    pub root_path: String,
    pub r#type: Option<String>,
    pub branch: Option<String>,
    pub server: Option<String>,
    pub port: Option<String>,
}

impl RepodConfig {
    pub fn new() -> Self {
        RepodConfig {
            defaults: Some(Defaults {
                r#type: None,
                branch: None,
                server: None,
                port: None,
                root_path: None,
            }),
            repos: None,
        }
    }

    pub fn from_config_file_sync(
        config_file_path: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let config = if Path::new(config_file_path).exists() {
            let configs_str = fs::read_to_string(config_file_path).expect(
                format!(
                    "Something went wrong reading the config file {}",
                    config_file_path
                )
                .as_str(),
            );
            toml::from_str(configs_str.as_str())?
        } else {
            RepodConfig::new()
        };
        Ok(config)
    }

    pub fn from_default_config_file_sync() -> Result<Self, Box<dyn std::error::Error>> {
        Self::from_config_file_sync(default_config_file_path().as_str())
    }

    pub fn to_config_file_sync(
        &self,
        config_file_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        fs::write(config_file_path, toml::to_string(&self)?)?;
        Ok(())
    }

    pub fn to_default_config_file_sync(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.to_config_file_sync(default_config_file_path().as_str())
    }

    pub fn add_repo(&mut self, repo: RepositoryConfig) {
        if let Some(repos) = &mut self.repos {
            repos.push(repo);
        } else {
            self.repos = Some(vec![repo]);
        }
    }

    pub fn has_repo(&self, repo_name: &str) -> bool {
        if let Some(repos) = &self.repos {
            for repo in repos {
                if repo.name == repo_name {
                    return true;
                }
            }
        }
        false
    }

    pub fn remove_repo(&mut self, repo_name: &str) {
        if let Some(repos) = &mut self.repos {
            let mut index = 0;
            for repo in repos.iter_mut() {
                if repo.name == repo_name {
                    repos.remove(index);
                    break;
                }
                index += 1;
            }
        }
    }
}

impl RepositoryConfig {
    pub fn new(
        name: String,
        commands: Vec<String>,
        updates: Vec<String>,
        root_path: String,
        r#type: Option<String>,
        branch: Option<String>,
        server: Option<String>,
        port: Option<String>,
    ) -> Self {
        RepositoryConfig {
            name,
            commands,
            updates,
            r#type,
            branch,
            server,
            port,
            root_path,
        }
    }
}
