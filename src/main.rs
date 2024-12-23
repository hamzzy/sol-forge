use clap::Parser;
use commands::{compile::compile_program, init};
use log::{debug, error, info};
use anyhow::Result;
use tokio;
mod config;
mod utils;
mod commands;

 #[derive(Debug, Parser)]
#[command(name = "anchorforge", version, about = "A Solana dev tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[arg(short, long, global = true, default_value = "info")]
    log_level: String,
}

#[derive(Debug, Parser)]
enum Commands {
    #[command(about = "Initializes a new Anchor project")]
    Init {
        #[arg(help = "The name of the project")]
        project_name: String
    },
    #[command(about = "Compiles the Anchor program")]
    Compile,
    #[command(about = "Manages the local Solana devnet")]
    Devnet {
        #[command(subcommand)]
        command: DevnetCommands,
    },
    #[command(about = "Deploys the compiled program to the local DevNet")]
    Deploy,
     #[command(about = "Runs tests inside the Anchor program")]
    Test,
}

 #[derive(Debug, Parser)]
enum DevnetCommands {
    #[command(about = "Starts the local Solana devnet")]
    Start,
    #[command(about = "Stops the local Solana devnet")]
    Stop,
}
#[tokio::main]
async fn main() -> Result<()> {
     let cli = Cli::parse();
     setup_logger(&cli.log_level)?;
     debug!("Parsed command: {:?}", cli);

     match cli.command {
        Commands::Init { project_name } => init::init_project(project_name).await,
        Commands::Compile => compile_program().await,
        Commands::Devnet { command } => match command {
            DevnetCommands::Start => commands::devnet::start_devnet().await,
            DevnetCommands::Stop => commands::devnet::stop_devnet().await,
        },
         Commands::Deploy => commands::deploy::deploy_program().await,
         Commands::Test => commands::test::run_tests().await,
    }
}
fn setup_logger(log_level: &str) -> Result<()> {
    env_logger::Builder::new()
        .parse_filters(log_level)
        .init();
    Ok(())
}