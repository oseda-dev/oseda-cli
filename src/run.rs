use std::{error::Error, process::Command};

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

    let child = Command::new("serve").arg("dist").spawn().map_err(|e| {
        println!("Error starting `serve dist`: {e}");
        OsedaRunError::ServeError("failed to start serve".into())
    })?;
    // spawn will leave child running the background. Need to listen for ctrl+c, snatch it. Then kill subprocess

    println!("leaving run method");

    Ok(())
}
