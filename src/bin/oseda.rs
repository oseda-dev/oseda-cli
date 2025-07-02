use std::{error::Error, process};

use clap::{Args, Parser, Subcommand};
use oseda_cli::cmd::{
    check,
    deploy::{self, deploy},
    init, run,
};

#[derive(Parser)]
#[command(name = "oseda")]
#[command(version = "0.1.0")]
#[command(about = "oseda project scafolding CLI", long_about = None)]
#[command(author = "oseda.net")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init(init::InitOptions),
    Run,
    Check(check::CheckOptions),
    Deploy(deploy::DeployOptions),
}

fn main() {
    let cli = Cli::parse();

    let result: Result<(), Box<dyn Error>> = match cli.command {
        Commands::Init(options) => init::init(options)
            .map(|_| println!("Successfully initialized oseda project"))
            .map_err(|e| e.into()),

        Commands::Run => run::run()
            .map(|_| println!("Successfully ran oseda project"))
            .map_err(|e| e.into()),

        Commands::Check(options) => check::check(options)
            .map(|_| println!("Successfully checked oseda project"))
            .map_err(|e| e.into()),

        Commands::Deploy(options) => deploy::deploy(options)
            .map(|_| {
                println!("Successfully deployed oseda project");
                println!("See deployment instructions...");
            })
            .map_err(|e| e.into()),
    };

    if let Err(err) = result {
        eprintln!("Error: {err}");
        process::exit(1);
    }
}
