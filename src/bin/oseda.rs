use std::{error::Error, process};

use clap::Parser;
use oseda_cli::{Cli, Commands, cmd::{
    check,
    deploy::{self},
    init, run,
}};

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
