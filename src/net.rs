use std::error::Error;

pub fn get_status(host: &str) -> Result<reqwest::StatusCode, Box<dyn Error>> {
    let response = reqwest::blocking::get(host)?;

    return Ok(response.status());
}
