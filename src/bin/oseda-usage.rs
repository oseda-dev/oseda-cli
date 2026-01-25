use std::{error::Error, fs};

use clap::Parser;
use oseda_cli::Cli;


fn main () -> Result<(), Box<dyn Error>> {
    let markdown = clap_markdown::help_markdown::<Cli>();

    fs::write("Usage.md", markdown)?;
    Ok(())
}
