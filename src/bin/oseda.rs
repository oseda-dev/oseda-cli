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

    match cli.command {
        Commands::Init(options) => match init::init(options) {
            Ok(_) => {
                println!("Sucessfully initialized oseda project")
            }
            Err(err) => {
                println!("Could not initialize project with error: {:?}", err)
            }
        },
        Commands::Run => match run::run() {
            Ok(_) => {
                println!("Sucessfully ran oseda project")
            }
            Err(err) => {
                println!("Could not run project with error: {:?}", err);
            }
        },
        Commands::Check(options) => match check::check(options) {
            Ok(_) => {
                println!("Sucessfully checked oseda project")
            }
            Err(err) => {
                println!("Project did not pass check with error: {:?}", err);
            }
        },
        Commands::Deploy(options) => match deploy::deploy(options) {
            Ok(_) => {
                println!("Sucessfully deployed oseda project");
                println!("See deployment instructions...")
            }
            Err(err) => {
                println!("Could not deploy project with error: {:?}", err);
            }
        },
    }
}
