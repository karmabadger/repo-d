use std::path::Path;

use crate::configs::default_configs::DEFAULT_CONFIG_FILE_PATH;
use std::fs;

pub async fn add_repo(
    name: String,
    r#type: Option<String>,
    branch: Option<String>,
    server: Option<String>,
    port: Option<String>,
    root_path: Option<String>,
    command: String,
) {
}
