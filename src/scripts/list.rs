use crate::configuration::RepodConfig;

pub fn list() {
    let mut config = RepodConfig::from_default_config_file_sync().unwrap();
    let repos = config.repos;
    if let Some(repos) = repos {
        println!("repos:");
        for repo in repos {
            println!("  {:?}", repo);
        }
    } else {
        println!("  no repos found");
    }
}
