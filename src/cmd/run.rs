use std::{process::Command, sync::mpsc};

/// More in depth errors that could cause a project not to run
#[derive(Debug)]
pub enum OsedaRunError {
    BuildError(String),
    ServeError(String),
}

impl std::error::Error for OsedaRunError {}
impl std::fmt::Display for OsedaRunError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BuildError(msg) => write!(f, "Oseda Build Error: {}", msg),
            Self::ServeError(msg) => write!(f, "Oseda Serve Error: {}", msg),
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
    // command run failure and command status are considered different, handled accordingly
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

    let mut child = Command::new("serve")
        .arg("dist")
        .arg("index.html")
        .spawn()
        .map_err(|e| {
            println!("Error starting `serve dist`: {e}");
            OsedaRunError::ServeError("failed to start serve".into())
        })?;
    // spawn will leave child running the background. Need to listen for ctrl+c, snatch it. Then kill subprocess

    // https://github.com/Detegr/rust-ctrlc
    let (tx, rx) = mpsc::channel();
    ctrlc::set_handler(move || {
        println!("\nSIGINT received. Attempting graceful shutdown...");
        let _ = tx.send(());
    })
    .expect("Error setting Ctrl+C handler");

    // block until ctrl+c
    rx.recv().unwrap();

    // attempt to kill the child process
    if let Err(e) = child.kill() {
        println!("Failed to kill `serve`: {e}");
    } else {
        println!("`serve` process terminated.");
    }

    let _ = child.wait();

    Ok(())
}
