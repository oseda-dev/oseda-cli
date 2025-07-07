use std::{error::Error, process};

use clap::{Parser, Subcommand};
use oseda_cli::cmd::{
    check,
    deploy::{self},
    init, run,
};

/// Oseda Project scafolding CLI
#[derive(Parser)]
#[command(name = "oseda")]
#[command(version = "0.1.0")]
#[command(about = "oseda project scafolding CLI", long_about = None)]
#[command(author = "oseda.net")]
struct Cli {
    /// The subcommand to run
    #[command(subcommand)]
    command: Commands,
}

/// Oseda subcommand, represents an action to take on your Oseda project
#[derive(Subcommand)]
enum Commands {
    /// Initialize a new Oseda project in the working directory
    Init(init::InitOptions),
    /// Run the Oseda project in the working directory
    Run,
    /// Check the Oseda project in the working directory for common errors
    Check(check::CheckOptions),
    /// Deploy your Oseda project to github to add to oseda.net
    Deploy(deploy::DeployOptions),
}
/// CLI entry point
fn main() {
    let cli = Cli::parse();

    // match on every subcommand result
    let result: Result<(), Box<dyn Error>> = match cli.command {
        Commands::Init(options) => {
            init::init(options).map(|_| println!("Successfully initialized oseda project"))
        }
        Commands::Run => run::run()
            .map(|_| println!("Successfully ran oseda project"))
            .map_err(|e| e.into()),
        Commands::Check(options) => check::check(options)
            .map(|_| println!("Successfully checked oseda project"))
            .map_err(|e| e.into()),
        Commands::Deploy(options) => deploy::deploy(options).map(|_| {
            println!("Successfully deployed oseda project");
            println!("See deployment instructions...");
        }),
    };

    // little annoying, but makes the exit code match what users would expect
    // this is also necessary for the check to run properly in CI
    if let Err(err) = result {
        eprintln!("Error: {err}");
        process::exit(1);
    }
}
