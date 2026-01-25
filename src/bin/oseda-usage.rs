use std::{error::Error, fs};

use oseda_cli::Cli;


/// Generates the Usage.md documentation from clap help menu 
fn main () -> Result<(), Box<dyn Error>> {
    let markdown = clap_markdown::help_markdown::<Cli>();

    fs::write("Usage.md", markdown)?;
    Ok(())
}
