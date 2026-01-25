use clap::{Parser, Subcommand};

pub mod cmd;
pub mod color;
pub mod config;
pub mod github;
pub mod net;
pub mod tags;
pub mod template;


/// Oseda Project scafolding CLI
#[derive(Parser)]
#[command(name = "oseda")]
#[command(version = "0.1.0")]
#[command(about = "oseda project scafolding CLI", long_about = None)]
#[command(author = "oseda.net")]
pub struct Cli {
    /// The subcommand to run
    #[command(subcommand)]
    pub command: Commands,
}

/// Oseda subcommand, represents an action to take on your Oseda project
#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new Oseda project in the working directory
    Init(cmd::init::InitOptions),
    /// Run the Oseda project in the working directory
    Run,
    /// Check the Oseda project in the working directory for common errors
    Check(cmd::check::CheckOptions),
    /// Deploy your Oseda project to github to add to oseda.net
    Deploy(cmd::deploy::DeployOptions),
}
