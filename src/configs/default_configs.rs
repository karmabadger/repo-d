pub const DEFAULT_CONFIG_FILE_NAME: &str = ".repodrc.toml";

pub fn default_config_file_path() -> String {
    let mut path = dirs::home_dir().expect("Could not find home directory");
    path.push(DEFAULT_CONFIG_FILE_NAME);
    path.to_str().unwrap().to_string()
}
