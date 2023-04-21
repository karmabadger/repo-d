use crate::{
    configuration::{defaults::DEFAULT_BRANCH, RepodConfig},
    server::{run_repo_command, start_server},
};

pub async fn run() -> std::io::Result<()> {
    println!("running repod in foreground. use ctrl+c to stop.");

    start_server().await
}
