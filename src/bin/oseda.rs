use clap::{Args, Parser, Subcommand};
use oseda_cli::cmd::{check, init, run};

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
    Check(check::CheckOptions),
    Deploy,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init(options) => match init::init(options) {
            Ok(_) => {
                println!("Sucessfully inited project")
            }
            Err(err) => {
                println!("could not init project with err {:?}", err)
            }
        },
        Commands::Run => match run::run() {
            Ok(_) => {
                println!("sucessfully ran")
            }
            Err(err) => {
                println!("{:?}", err);
            }
        },
        Commands::Check(options) => match check::check(options) {
            Ok(res) => {
                println!("sucessfully ran check")
            }
            Err(err) => {
                println!("{:?}", err);
            }
        },
        Commands::Deploy => {
            println!("deploy ran")
        }
    }
}
