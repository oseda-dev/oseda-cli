use std::{error::Error, fs::{self, File}, path::{self, Path, PathBuf}};

use clap::Args;

use crate::{cmd::init::{InitOptions, create_project_on_fs}, config::create_conf, conversion, template::Template};

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

    // this will always prompt the user for everything, which may be annoying
    // from a scripting perspective, but I can compromise for now
    let conf = create_conf(InitOptions { 
        template: Some("HTML".to_string()),
        ..Default::default()
    })?;

    create_project_on_fs(conf.clone(), Template::HTML)?;

    let path_parts = [conf.title.as_str(), "slides", "slides.html"];
    let output_path = path_parts.iter().collect::<PathBuf>();

    let response = conversion::gemini::get_gemini_response(&opts.filename)?;


    fs::write(output_path, response)?;

    Ok(())
}
