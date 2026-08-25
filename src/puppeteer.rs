// utils for puppeteer and chrome

use std::{error::Error, process::Command};

use inquire::Confirm;
/// Checks whether headless Chrome has been installed on the users system via npx puppeteer
///
/// # Returns
///
/// - `bool` - Whether or not it has been installed
pub fn is_puppeteer_chrome_installed() -> bool {
    //puppeteer will install into cache directory
    // this is differnt on different systems
    // afaik, not even specific to just OS -> so I pulled in `dirs`
    let mut path = match dirs::cache_dir() {
        Some(p) => p,
        None => return false,
    };

    // {cache}/puppeteer/chrome
    path.push("puppeteer");
    path.push("chrome");

    if path.is_dir() {
        if let Ok(entries) = std::fs::read_dir(&path) {
            // if any folder present, should be installed
            return entries
                .filter_map(Result::ok)
                .any(|entry| entry.path().is_dir());
        }
    }

    false
}

/// Prompts the user for an installation of headless Chrome via. Puppeteer, installing it if they elect to proceed
///
/// # Returns
///
/// - `Result<(), Box<dyn Error>>`
/// - Ok(()) if installation proceeds as normal
/// - Err() if installation is aborted by user
pub fn prompt_install_puppeteer_chrome() -> Result<(), Box<dyn Error>> {
    println!("`oseda export` uses DeckTape to export your presentation");
    println!("DeckTape needs chromium installed (puppeteer prefered");
    println!("Puppeteer Chrome is not currently detected.");
    let ans = Confirm::new("Would you like to install it and proceed?")
        .with_default(false)
        .prompt();

    // let else my beloved
    // gaurd clause
    let Ok(true) = ans else {
        return Err("DeckTape is required to proceed".into());
    };

    println!("Installing Puppeteer Chrome... (This may take several minutes, you will only need to install this once");

    let chrome_install_output = Command::new("npx")
        .args(["puppeteer", "browsers", "install", "chrome"])
        .output()?;

    if !chrome_install_output.status.success() {
        return Err("Error: could not install puppeteer chrome".into());
    }

    Ok(())
}
