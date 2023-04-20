mod configs;
mod configuration;
mod scripts;
mod utils;

use clap::{Parser, Subcommand};

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
    // #[clap(default_value_t = Commands::Hello, validator = validate_command)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[clap(about = "Start the daemon.")]
    Start {},
    #[clap(about = "Restart the daemon.")]
    Restart {},
    #[clap(about = "Stop the daemon.")]
    Stop {},
    #[clap(about = "Add a new repository to the daemon.")]
    AddRepo {
        #[clap(help = "The name of the repository.", short, long)]
        name: String,
        #[clap(
            help = "The type of the repository host platform (github or gitlab).",
            short,
            long
        )]
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
        #[clap(help = "The root path of the repo locally.", short, long)]
        root_path: Option<String>,
        #[clap(help = "The shell command to run once it is pulled.", short, long)]
        command: String,
    },
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    match args.command {
        Commands::Start {} => println!("Start"),
        Commands::Restart {} => println!("Restart"),
        Commands::Stop {} => println!("Stop"),
        Commands::AddRepo {
            name,
            r#type,
            branch,
            server,
            port,
            root_path,
            command,
        } => {
            println!("AddRepo");
            println!("name: {}", name);
            println!("type: {:?}", r#type);
            println!("branch: {:?}", branch);
            println!("server: {:?}", server);
            println!("port: {:?}", port);
            println!("root_path: {:?}", root_path);
            println!("command: {}", command);
        }
    }

    Ok(())
}
