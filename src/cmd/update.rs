use std::{error::Error, process::{Command, Stdio}};

pub fn update() -> Result<(), Box<dyn Error>>{
    let status = Command::new("cargo")
        .args(["install", "oseda-cli"])
        .stdout(Stdio::inherit())
        .stdin(Stdio::inherit())
        .status()?;

    if !status.success() {
        return Err(format!("Error: exit status {}", status).into());
    }

    Ok(())
}