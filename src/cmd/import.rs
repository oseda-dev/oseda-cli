use std::{error::Error, fs::{self, File}, path::{self, Path, PathBuf}};

use clap::Args;

use crate::{cmd::init::{InitOptions, create_project_on_fs}, config::create_conf, conversion::{self, image::extract_pdf_images}, template::Template};

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
    // let output_proj_name = opts.get_output_name();

    let conf = create_conf(InitOptions { 
        template: Some("HTML".to_string()),
        ..Default::default()
    })?;

    create_project_on_fs(conf.clone(), Template::HTML)?;

    let project_dir = PathBuf::from(&conf.title);
    let public_dir = Path::join(&project_dir, "public");
    let html_output_path = project_dir.join("slides").join("slides.html");
    let pdf_path = PathBuf::from(&opts.filename);

    // images go into [title]/public/img{num}.ext
    let base_filenames = extract_pdf_images(&pdf_path, &public_dir)?;
    println!("Extracted {} image(s) into {:?}", base_filenames.len(), public_dir);

    //  filenames => ["img1.png", "img2.jpg"], and pray gemini understands what we're talking about
    let response = conversion::gemini::convert_pdf(&opts.filename, &base_filenames)?;

    fs::write(html_output_path, response)?;

    Ok(())
}