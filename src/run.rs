use std::{error::Error, process::Command};

pub fn run() -> Result<(), Box<dyn Error>> {
    match Command::new("npx").arg("vite").arg("build").status() {
        Ok(status) => {
            if !status.success() {
                println!("Error: `npx vite build` exited with a failure.");
                println!("Please ensure that npx and vite are installed properly.");
            }
        }
        Err(e) => {
            println!("Error: failed to execute `npx vite build`: {e}");
            println!("Please ensure that `npx` and `vite` are installed and in your PATH.");
        }
    }

    match Command::new("serve").arg("dist").status() {
        Ok(status) => {
            if !status.success() {
                println!("Error: `serve dist` exited with a failure.");
                println!(
                    "Please ensure that `serve` is installed properly (e.g., with `npm install -g serve`)."
                );
            }
        }
        Err(e) => {
            println!("Error: failed to execute `serve dist`: {e}");
            println!("Please ensure that `serve` is installed and in your PATH.");
        }
    }

    Ok(())
}
