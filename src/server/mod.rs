use std::{
    collections::HashMap,
    process::{Command, Output},
};

use actix_web::{
    get, post,
    web::{self, Data},
    App, HttpResponse, HttpServer,
};
use serde::Deserialize;

use crate::configuration::{
    defaults::{DEFAULT_BRANCH, DEFAULT_PATH, DEFAULT_PORT, DEFAULT_SERVER},
    RepodConfig, RepositoryConfig,
};

#[derive(Deserialize)]
pub struct Payload {
    pub event: String,
    pub repository: Repository,
    pub r#ref: String,
    pub commits: Vec<Commit>,
}

#[derive(Deserialize)]
pub struct Repository {
    pub name: String,
    pub owner: Owner,
}

#[derive(Deserialize)]
pub struct Owner {
    pub login: String,
}

#[derive(Deserialize)]
pub struct Commit {
    pub id: String,
    pub message: String,
    pub author: Author,
}

#[derive(Deserialize)]
pub struct Author {
    pub name: String,
    pub email: String,
}

// handler for index route
// takes a body
#[post("/repod-webhooks")]
async fn index(
    payload: web::Json<Payload>,
    config: Data<RepodConfig>,
    repo_map: Data<HashMap<String, RepositoryConfig>>,
) -> HttpResponse {
    if payload.event == "push" {
        let repo_name = payload.repository.name.clone();
        let branch = payload.r#ref.replace("refs/heads/", "");
        let repo_config = repo_map.get(&repo_name);
        if repo_config.is_none() {
            return HttpResponse::Ok().body(format!("repo {} not found", repo_name));
        }
        let repo_config = repo_config.unwrap();
        let repo_branch = repo_config
            .branch
            .clone()
            .unwrap_or(DEFAULT_BRANCH.to_string());
        if branch != repo_branch {
            return HttpResponse::Ok().body("Hello");
        }

        run_repo_command(
            &branch,
            &repo_config.root_path,
            &repo_config.commands,
            &repo_config.updates,
        );
    }
    HttpResponse::Ok().body("Hello")
}

pub async fn start_server() -> std::io::Result<()> {
    let mut config = RepodConfig::from_default_config_file_sync().unwrap();

    let config2 = config.clone();
    let repos = config.repos.unwrap();
    let mut repo_map = HashMap::new();
    for repo in repos {
        repo_map.insert(repo.name.clone(), repo);
    }

    println!("starting server on {}:{}", DEFAULT_SERVER, DEFAULT_PORT);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(config2.clone()))
            .app_data(Data::new(repo_map.clone()))
            .service(index)
    })
    .bind((DEFAULT_SERVER, DEFAULT_PORT))?
    .run()
    .await
}

pub fn run_repo_command(
    branch: &str,
    root_path: &str,
    commands_str: &Vec<String>,
    updates_str: &Vec<String>,
) {
    // run updates commands
    println!("  running updates:");
    for update_str in updates_str {
        println!("    {}", update_str);
        let output = run_sh_command_in_dir(&update_str, root_path);
        println!("    {}", String::from_utf8_lossy(&output.stdout));
    }

    // run commands
    println!("  running commands:");
    for command_str in commands_str {
        println!("    {}", command_str);
        let command_str = command_str.replace("{branch}", branch);
        let output = run_sh_command_in_dir(&command_str, root_path);
        println!("    {}", String::from_utf8_lossy(&output.stdout));
    }
}

pub fn run_sh_command_in_dir(command_string: &str, root_path: &str) -> Output {
    let mut command = Command::new("sh");
    let mut command = command.arg("-c");
    let mut command = command.arg(format!("cd {} && {}", root_path, command_string));

    let output = command.output().expect("failed to execute process");
    output
}
