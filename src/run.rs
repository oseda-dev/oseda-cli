use std::{error::Error, process::Command};

pub fn run() -> Result<(), Box<dyn Error>> {
    let build_status = Command::new("npx").arg("vite").arg("build").status()?;

    if !build_status.success() {
        println!("Error: could not run `npx vite build`");
        println!("Please ensure that npx and vite are both installed properly")
    }

    let serve_status = Command::new("serve").arg("dist").status()?;

    if !serve_status.success() {
        println!("Error: could not run `serve dist`");
        println!("Please ensure that npx and vite are both installed properly")
        println!("with npm install -g serve")
    }
    Ok(())
}
