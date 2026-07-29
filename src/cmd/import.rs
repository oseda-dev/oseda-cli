use std::{error::Error, fs::{self, File}, path::Path};

use clap::Args;
use gemini_client_api::gemini::{ask::Gemini, types::sessions::Session};

use crate::conversion;

/// Options struct for the import subcommand
#[derive(Args, Debug, Clone)]
pub struct ImportOptions {
    /// name of PDF to import
    #[arg()]
    pub filename: String,
    /// String name of the output project name [default: <filename_without_extension>]
    #[arg(long)]
    pub output: Option<String>,    
    /// Port the Docling Client runs on
    #[arg(long, default_value_t = 8000)]
    pub port: u16,
}

impl ImportOptions {
    /// Helper method to get the explicit output name or the fallback filename
    pub fn get_output_name(&self) -> String {
        self.output.clone().unwrap_or_else(|| {
            // Extracts "document" from "path/to/document.pdf"
            Path::new(&self.filename)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or(&self.filename)
                .to_string()
        })
    }
}


pub fn import(opts: ImportOptions) -> Result<(), Box<dyn Error>> {
    let output_proj_name = opts.get_output_name();
    println!("File name: {:?}", opts.filename.clone());
    println!("Output project name: {:?}", output_proj_name);



    let response = conversion::gemini::get_gemini_response(&opts.filename)?;
    fs::write("sample_output.html", response)?;

    Ok(())
}
