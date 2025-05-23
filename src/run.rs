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

    match Command::new("serve").arg("dist").status() {
        Ok(status) => {
            if !status.success() {
                println!("Error: `serve dist` exited with a failure.");
                println!(
                    "Please ensure that `serve` is installed properly (e.g., with `npm install -g serve`)."
                );
                return Err(OsedaRunError::ServeError(
                    "could not 'serve dist'".to_string(),
                ));
            }
        }
        Err(e) => {
            println!("Error: failed to execute `serve dist`: {e}");
            println!("Please ensure that `serve` is installed and in your PATH.");
            return Err(OsedaRunError::ServeError(
                "could not 'serve dist'".to_string(),
            ));
        }
    }

    Ok(())
}
