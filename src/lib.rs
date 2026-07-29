use clap::{Parser, Subcommand};

pub mod cmd;
pub mod color;
pub mod config;
pub mod github;
pub mod net;
pub mod tags;
pub mod template;
pub mod conversion;


/// Oseda Project scaffolding CLI
#[derive(Parser)]
#[command(name = "oseda")]
#[command(version)]
#[command(about = "oseda project scaffolding CLI", long_about = None)]
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
    /// Fork the library repository to submit your course
    Fork,
    /// Export the Oseda project to a PDF file
    /// This will install the npm package `decktape`
    /// This relies on a chromium backend, as a result, it may take a while to run
    Export(cmd::export::ExportOptions),

    /// Import a PDF presentation and convert it into an Oseda project
    /// This is highly experimental and relies of generative AI
    /// `import` relies on several dependencies such as [TODO]
    Import(cmd::import::ImportOptions),
}
