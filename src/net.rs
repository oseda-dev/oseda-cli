use std::{error::Error, process::Command};

/// Checks the status of host url from a GET request
///
/// # Arguments
/// * `host` - the full URL to check -> eg.. `"http://localhost:3000"`
///
/// # Returns
/// * `Ok(StatusCode)` if the request was successfully sent, containing the resulting status code
/// * `Err` if the request fails to *send*. This is not reflective of the status code
///
pub fn get_status(host: &str) -> Result<reqwest::StatusCode, Box<dyn Error>> {
    let response = reqwest::blocking::get(host)?;

    Ok(response.status())
}

/// Kills any process listening to a provided port number
///
/// # Platform
/// This function only works on Unix based systems
///
/// # Arguments
/// * `port_num` - the TCP port number to search for and terminate
///
/// # Returns
/// * `Ok(())` if processes were successfully terminated -> even if none were found
/// * `Err` if `lsof` or `kill` fails, or if output cannot be parsed properly
pub fn kill_port(port_num: u16) -> Result<(), Box<dyn Error>> {
    // TIL cargo lets you do # Platform in the doc comments

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
