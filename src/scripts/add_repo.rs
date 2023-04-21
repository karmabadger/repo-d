use crate::configuration::{RepodConfig, RepositoryConfig};

pub fn add_repo(
    name: String,
    r#type: Option<String>,
    branch: Option<String>,
    server: Option<String>,
    port: Option<String>,
    root_path: String,
    command: Vec<String>,
    update: Vec<String>,
) {
    let mut config = RepodConfig::from_default_config_file_sync().unwrap();
    config.add_repo(RepositoryConfig::new(
        name, command, update, root_path, r#type, branch, server, port,
    ));

    let res = config.to_default_config_file_sync();
}
