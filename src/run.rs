use std::{error::Error, process::Command};

pub fn run() -> Result<(), Box<dyn Error>> {
    Command::new("npx").arg("vite").arg("build").status()?;

    Command::new("serve").arg("dist").status()?;

    Ok(())
}
