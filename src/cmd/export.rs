use std::{error::Error, process::Command, sync::{Arc, atomic::AtomicBool}};

use clap::Args;

use crate::{cmd::run, net::kill_port};


#[derive(Args, Debug, Clone)]
pub struct ExportOptions {
    #[arg(long, default_value = "slides.pdf")]
    pub output: String,
    #[arg(long, default_value_t = 3000)]
    pub port: u16,

}
pub fn export(opts: ExportOptions) -> Result<(), Box<dyn Error>> {


    let output = Command::new("npm")
            .args(["install", "decktape@3.15.0"])
            .current_dir(".")
            .output()?;

    if !output.status.success() {
        eprintln!(
            "Decktape installation failure: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        return Err("npm init failed".into());
    }

    // decktape automatic http://localhost:3000/ Desktop/IntroToRust/slides.pdf

    // let 


    let _run_handle = std::thread::spawn(run::run);


    let addr = format!("http://localhost:{}", opts.port);

    let export_output = Command::new("decktape")
        .args(["automatic", &addr, &opts.output])
        .output()?;

    if kill_port(opts.port).is_err() {
        println!("Warning: could not kill process on port, project could still be running");
    }
    if !export_output.status.success() {
        eprintln!(
            "Decktape PDF export failure: {}",
            String::from_utf8_lossy(&export_output.stderr)
        );
        return Err("npm init failed".into());
    }




    Ok(())
}
