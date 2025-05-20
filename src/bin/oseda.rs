use clap::{Args, Parser, Subcommand};

use oseda_cli::init;

#[derive(Parser)]
#[command(name = "oseda")]
#[command(version = "0.1.0")]
#[command(about = "oseda scafolding", long_about = None)]
#[command(author = "oseda.net")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init(init::InitOptions),
    Run,
    Check,
    Deploy,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init(options) => {
            println!("init ran");
            println!("options: {:?}", options);

            // will need to pass init options at some point
            match init::init(options) {
                Ok(_) => {
                    println!("Sucessfully inited project")
                }
                Err(err) => {
                    println!("could not init project with err {:?}", err)
                }
            }
        }
        Commands::Run => {
            println!("run ran")
        }
        Commands::Check => {
            println!("check ran")
        }
        Commands::Deploy => {
            println!("deploy ran")
        }
    }
}
