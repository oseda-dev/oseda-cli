use std::{
    error::Error,
    process::Command,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use clap::Args;
use inquire::Confirm;

use crate::{cmd::run, net::kill_port, puppeteer::{self, is_puppeteer_chrome_installed, prompt_install_puppeteer_chrome}};

/// Options struct for the export subcommand
#[derive(Args, Debug, Clone)]
pub struct ExportOptions {
    /// String name of the output PDF file
    #[arg(long, default_value = "slides.pdf")]
    pub output: String,
    /// Port the project runs on
    #[arg(long, default_value_t = 3000)]
    pub port: u16,
}

/// Export the current Oseda project to a PDF file via `decktape`
pub fn export(opts: ExportOptions) -> Result<(), Box<dyn Error>> {
    println!("Cleaning any existing oseda processing...");
    if kill_port(opts.port).is_err() {
        eprintln!("Warning, could not kill value on desired port")
    }

    let output = Command::new("npm")
        .args(["install", "decktape@3.15.0"])
        .current_dir(".")
        .output()?;

    // prompt for puppeteer and confirm installation
    if !is_puppeteer_chrome_installed(){
        prompt_install_puppeteer_chrome()?;
    }


    if !output.status.success() {
        eprintln!(
            "Decktape installation failure: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        return Err("npm init failed".into());
    }

    // decktape automatic http://localhost:3000/ Desktop/IntroToRust/slides.pdf

    let shutdown_flag = Arc::new(AtomicBool::new(false));
    let run_flag = shutdown_flag.clone();

    let run_handle = std::thread::spawn(move || run::run_with_shutdown(run_flag));

    // wait a moment for the localhost server to spin up
    std::thread::sleep(std::time::Duration::from_millis(10000));

    let addr = format!("http://localhost:{}", opts.port);

    // run decktape, assuming the server has spun up by now
    let export_output = Command::new("npm")
        .args(["exec", "decktape", "automatic", &addr, &opts.output])
        .output()?;

    // send shutdown flag, should signal to run_with_shutdown to kill the process
    shutdown_flag.store(true, Ordering::SeqCst);
    // wait to run to terminate (hopefully gracefully) and join the process to cur. thread
    let _ = run_handle.join();

    if !export_output.status.success() {
        eprintln!(
            "Decktape PDF export failure: {}",
            String::from_utf8_lossy(&export_output.stderr)
        );
        return Err("npm init failed".into());
    }

    Ok(())
}



