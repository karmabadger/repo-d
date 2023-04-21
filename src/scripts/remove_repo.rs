use crate::configuration::RepodConfig;

pub fn remove_repo(name: String) {
    let mut config = RepodConfig::from_default_config_file_sync().unwrap();
    config.remove_repo(name.as_str());
    let res = config.to_default_config_file_sync();
}
