mod configs;
mod configuration;
mod scripts;
mod server;
mod utils;

use clap::{Parser, Subcommand};
use scripts::{
    add_repo::{self, add_repo},
    list::{self, list},
    remove_repo::{self, remove_repo},
    run::run,
    start::start,
};

#[derive(Parser, Debug)]
#[clap(
    author = "karmabadger",
    version,
    about = "A simple daemon to manage your git repositories and run commands on them on change events.",
    long_about = "A simple daemon to manage your git repositories and run commands on them on change events."
)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[clap(about = "Start the daemon.")]
    Start,
    #[clap(about = "Restart the daemon.")]
    Restart,
    #[clap(about = "Stop the daemon.")]
    Stop,
    #[clap(about = "Add a new repository to the daemon's configs.")]
    AddRepo {
        #[clap(help = "The name of the repository.", short, long)]
        name: String,
        #[clap(
            help = "The type of the repository host platform (github or gitlab).",
            short,
            long
        )]
        #[clap(help = "The root path of the repo locally.", short, long)]
        root_path: String,
        r#type: Option<String>,
        #[clap(help = "The branch to watch for changes.", short, long)]
        branch: Option<String>,
        #[clap(
            help = "The url at which the daemon needs to watch for events.",
            short,
            long
        )]
        server: Option<String>,
        #[clap(
            help = "The port at which the daemon needs to watch for events.",
            short,
            long
        )]
        port: Option<String>,
        #[clap(
            help = "The shell command to run once it is pulled.",
            short,
            long,
            value_parser,
            required = true
        )]
        commands: Vec<String>,
        #[clap(
            help = "The shell command to run to update/pull repo.",
            short,
            long,
            value_parser,
            required = true
        )]
        updates: Vec<String>,
    },
    #[clap(about = "Remove a repository from the daemon's configs.")]
    RemoveRepo {
        #[clap(help = "The name of the repository.", short, long)]
        name: String,
    },
    UpdateRepo,
    #[clap(about = "Run the daemon in the foreground.")]
    Run,
    #[clap(about = "List all the repositories in the daemon's configs.")]
    List,
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let args = Cli::parse();

    match args.command {
        Commands::Start {} => {
            start();
        }
        Commands::Restart {} => println!("Restart"),
        Commands::Stop {} => println!("Stop"),
        Commands::Run {} => {
            run().await?;
        }
        Commands::AddRepo {
            name,
            r#type,
            branch,
            server,
            port,
            root_path,
            updates,
            commands,
        } => {
            add_repo(
                name, r#type, branch, server, port, root_path, commands, updates,
            );
        }
        Commands::RemoveRepo { name } => {
            remove_repo(name);
        }
        Commands::UpdateRepo {} => println!("Update"),
        Commands::List {} => {
            list();
        }
    }

    Ok(())
}
