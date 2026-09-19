use std::{
    process::Command,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

use clap::Args;

use crate::cmd::run::is_cwd_oseda_project;

/// Options for the `oseda dev` command
#[derive(Args, Debug, Clone)]
pub struct DevOptions {
    /// Port to run the vite dev server on
    #[arg(long, default_value_t = 3000)]
    pub port: u16,
}

#[derive(Debug)]
pub enum OsedaDevError {
    NotOsedaProjectError(String),
    ServeError(String),
}

impl std::error::Error for OsedaDevError {}
impl std::fmt::Display for OsedaDevError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotOsedaProjectError(msg) => write!(
                f,
                "Current working directory is not an Oseda project: {}",
                msg
            ),
            Self::ServeError(msg) => write!(f, "Oseda Dev Server Error: {}", msg),
        }
    }
}


/// Run in dev mode, with auto-reload on save
/// 
/// # Arguments:
/// * `opts` - options for subcommand
/// * `shutdown_flag` - Arc to kill process
/// 
/// # Returns
/// * `Ok()` on success
/// * `Err` on any issue related to running oseda in dev mode
pub fn dev(opts: DevOptions) -> Result<(), OsedaDevError> {
    dev_with_shutdown(opts, Arc::new(AtomicBool::new(false)))
}

/// Run in dev mode, with auto-reload on save, with a shutdown flag
/// 
/// # Arguments:
/// * `opts` - options for subcommand
/// * `shutdown_flag` - Arc to kill process
/// 
/// # Returns
/// * `Ok()` on success
/// * `Err` on any issue related to running oseda in dev mode
pub fn dev_with_shutdown(
    opts: DevOptions,
    shutdown_flag: Arc<AtomicBool>,
) -> Result<(), OsedaDevError> {
    if !is_cwd_oseda_project() {
        return Err(OsedaDevError::NotOsedaProjectError(
            "oseda-config.json not found".to_string(),
        ));
    }

    let mut cmd = Command::new("npx");
    cmd.arg("vite")
        .arg("--port")
        .arg(opts.port.to_string())
        // fail if port not allowed
        .arg("--strictPort");

    let mut child = cmd.spawn().map_err(|e| {
        println!("Error starting `npx vite`: {e}");
        println!("Please ensure that `npx` and `vite` are installed and in your PATH.");
        OsedaDevError::ServeError("failed to start vite dev server".into())
    })?;

    let ctrlc_flag = shutdown_flag.clone();
    ctrlc::set_handler(move || {
        println!("\nSIGINT received. Shutting down dev server...");
        ctrlc_flag.store(true, Ordering::SeqCst);
    })
    .map_err(|e| {
        println!("Error setting ctrl+c handler: {e}");
        OsedaDevError::ServeError("failed to set handler".into())
    })?;

    // block until ctrl+c/shutdown flag OR the vite process dies on its own
    while !shutdown_flag.load(Ordering::SeqCst) {
        if let Ok(Some(status)) = child.try_wait() {
            println!("`vite` exited on its own with status: {status}");
            return Err(OsedaDevError::ServeError(
                "vite dev server exited unexpectedly".into(),
            ));
        }
        std::thread::sleep(Duration::from_millis(100));
    }

    if let Err(e) = child.kill() {
        println!("Failed to kill `vite`: {e}");
    } else {
        println!("`vite` dev server terminated.");
    }

    let _ = child.wait();

    Ok(())
}