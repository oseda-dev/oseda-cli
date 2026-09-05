use std::{
    path::Path,
    process::Command,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

use crate::config::{self};

/// More in depth errors that could cause a project not to run
#[derive(Debug)]
pub enum OsedaRunError {
    BuildError(String),
    ServeError(String),
    NotOsedaProjectError(String),
}

impl std::error::Error for OsedaRunError {}
impl std::fmt::Display for OsedaRunError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BuildError(msg) => write!(f, "Oseda Build Error: {}", msg),
            Self::ServeError(msg) => write!(f, "Oseda Serve Error: {}", msg),
            Self::NotOsedaProjectError(msg) => write!(
                f,
                "Current working directory is not an Oseda project: {}",
                msg
            ),
        }
    }
}

/// Runs an Oseda project in the working directory
///
/// This will:
/// - Run `npx vite build`
/// - Start a static file server (`serve dist`)
/// - Gracefully listen for Ctrl+C to shut down the server
///     - This gracefull-ness here is important, this runs on a separate thread, do not attempt to orphan this process
/// # Returns
/// * `Ok(())` if both the build and serve steps succeed
/// * `Err(OsedaRunError)` if any step fails (missing vite isn't installed, or `serve` fails to start)
pub fn run() -> Result<(), OsedaRunError> {
    // todo refactor the other check command to use this
    run_with_shutdown(Arc::new(AtomicBool::new(false)))
}

pub fn is_cwd_oseda_project() -> bool {
    Path::new(config::CONFIG_FILE_NAME)
        .try_exists()
        .is_ok_and(|exists| exists)
}

pub fn run_with_shutdown(shutdown_flag: Arc<AtomicBool>) -> Result<(), OsedaRunError> {
    // command run failure and command status are considered different, handled accordingly
    if !is_cwd_oseda_project() {
        return Err(OsedaRunError::NotOsedaProjectError(
            "oseda-config.json not found".to_string(),
        ));
    }

    match Command::new("npx").arg("vite").arg("build").status() {
        Ok(status) => {
            if !status.success() {
                println!("Error: `npx vite build` exited with a failure.");
                println!("Please ensure that npx and vite are installed properly.");
                return Err(OsedaRunError::BuildError(
                    "could not 'npx vite build'".to_string(),
                ));
            }
        }
        Err(e) => {
            println!("Error: failed to execute `npx vite build`: {e}");
            println!("Please ensure that `npx` and `vite` are installed and in your PATH.");
            return Err(OsedaRunError::BuildError(
                "could not 'npx vite build'".to_string(),
            ));
        }
    }

    let mut child = Command::new("npx")
        .arg("serve")
        .arg("dist")
        .spawn()
        .map_err(|e| {
            println!("Error starting `serve dist`: {e}");
            OsedaRunError::ServeError("failed to start serve".into())
        })?;
    // spawn will leave child running the background. Need to listen for ctrl+c, snatch it. Then kill subprocess

    // https://github.com/Detegr/rust-ctrlc
    // let (tx, rx) = mpsc::channel();
    let ctrlc_flag = shutdown_flag.clone();
    ctrlc::set_handler(move || {
        println!("\nSIGINT received. Attempting graceful shutdown...");
        ctrlc_flag.store(true, Ordering::SeqCst);
    })
    .map_err(|e| {
        println!("Error setting ctrl+c handler: {e}");
        OsedaRunError::ServeError("failed to set handler".into())
    })?;

    // block until ctrl+c or sigkill or flag set otherwise (e.g. via export)
    while !shutdown_flag.load(Ordering::SeqCst) {
        std::thread::sleep(Duration::from_millis(100));
    }

    // attempt to kill the child process
    if let Err(e) = child.kill() {
        println!("Failed to kill `serve`: {e}");
    } else {
        println!("`serve` process terminated.");
    }

    let _ = child.wait();

    Ok(())
}
