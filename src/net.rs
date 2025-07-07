use std::{error::Error, process::Command};

pub fn get_status(host: &str) -> Result<reqwest::StatusCode, Box<dyn Error>> {
    let response = reqwest::blocking::get(host)?;

    Ok(response.status())
}

// this will only work on linux sadly
pub fn kill_port(port_num: u16) -> Result<(), Box<dyn Error>> {
    // kill $(lsof -t -i:PORT_NUM)

    let lsof_out = Command::new("lsof")
        .arg("-t")
        .arg(format!("-i:{}", port_num))
        .output()?;

    let procs_on_port = String::from_utf8(lsof_out.stdout)?;

    for pid in procs_on_port.lines() {
        Command::new("kill").arg(pid).output()?;
    }

    Ok(())
}
