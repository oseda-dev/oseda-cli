use clap::{Args, Parser, Subcommand};

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
    Init(InitOptions),
    Run,
    Check,
    Deploy,
}

#[derive(Args, Debug)]
struct InitOptions {
    #[arg(long, required = false)]
    presentation_only: bool,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init(options) => {
            println!("init ran");
            println!("options: {:?}", options)
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
