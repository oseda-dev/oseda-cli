use std::{
    error::Error,
    fs::{self},
    process::Command,
    str::FromStr,
};

use clap::Args;
use spinners::{Spinner, Spinners};
use strum::IntoEnumIterator;

use crate::{config, templates::{self, template::CLITemplate}};

/// Options for the `oseda init` command
#[derive(Args, Debug)]
pub struct InitOptions {
    // claps 'value_name' does not change the argument name, basically just the value in the help menu,
    // e.g  --template <FORMAT>
    /// Project Title
    #[arg(long, value_name = "TITLE")]
    pub title: Option<String>,

    /// Project Tags [e.g: ComputerScience,Engineering,...]
    #[arg(long, value_delimiter = ',', value_name = "TAG1,TAG2,...")]
    pub tags: Option<Vec<String>>,

    /// Project Color [e.g: Red]
    #[arg(long, value_name = "COLOR")]
    pub color: Option<String>,

    /// Project Template Format [HTML | Markdown]
    #[arg(long, value_name = "FORMAT")]
    pub template: Option<String>,

    /// OSI License SPDX identifier [e.g: MIT]
    #[arg(long, value_name = "SPDX_ID")]
    pub license: Option<String>,

    /// Project Description
    #[arg(long, value_name = "TEXT")]
    pub description: Option<String>,
}


/// Initialize an Oseda project with the provided options
///
/// This command will:
/// - Run `npm init`
/// - Install required dependencies (Vite, Reveal.js, etc)
/// - Write config and boilerplate files
///
/// # Arguments
/// * `_opts` - command-line options (this is unused rn, used later I hope)
///
/// # Returns
/// * `Ok(())` if project initialization is suceeded
/// * `Err` if any step (npm, file write, config generation etc) fails
pub fn init(opts: InitOptions) -> Result<(), Box<dyn Error>> {
    let template = match opts.template {
        Some(ref arg_template) => {
            CLITemplate::from_str(arg_template).map_err(|_| "Invalid template".to_string())?
        }
        None => prompt_template()?,
    };

    let conf = config::create_conf(opts)?;

    std::fs::create_dir_all(&conf.title)?;

    let output = Command::new("npm")
        .args(["init", "-y", "--prefix", &conf.title])
        .current_dir(&conf.title)
        .output()?;

    // swapped to explicit check so it doesn't hang after
    if !output.status.success() {
        eprintln!(
            "npm init failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        return Err("npm init failed".into());
    }

    let npm_commands = vec![
        format!("install --save-dev vite@5.4.21 http-server@14.1.1"),
        format!("install reveal.js@5.2.1 serve@14.2.6 highlight.js@11.12.0"),
        format!("install patch-package@8.0.1"),
    ];

    for c in npm_commands {
        let mut spinner = Spinner::new(Spinners::Dots9, "Initializing...".into());

        let args: Vec<&str> = c.split(' ').collect();
        let output = Command::new("npm")
            .args(&args)
            .current_dir(&conf.title)
            .output()?;

        if !output.status.success() {
            eprintln!(
                "npm {} failed: {}",
                c,
                String::from_utf8_lossy(&output.stderr)
            );
            return Err(format!("npm {} failed", c).into());
        }
        spinner.stop();

        println!("Bootstrapped npm {}", c);
    }

    println!("Saving config file...");

    config::write_config(&conf.title, &conf)?;

    // 99% sure we'll only ever have to maintain these two template schemas
    match template {
        CLITemplate::Markdown => {
            // fs::write(format!("{}/package.json", &conf.title), MD_PACKAGE_JSON)?;
            fs::write(format!("{}/vite.config.js", &conf.title), templates::md::MD_VITE_CONFIG_JS)?;
            fs::write(format!("{}/index.html", &conf.title), templates::md::MD_INDEX_HTML)?;
            fs::write(format!("{}/.gitignore", &conf.title), templates::md::MD_GITIGNORE)?;

            std::fs::create_dir_all(format!("{}/src", &conf.title))?;
            fs::write(format!("{}/src/main.js", &conf.title), templates::md::MD_MAIN_JS)?;

            std::fs::create_dir_all(format!("{}/slides", &conf.title))?;
            fs::write(format!("{}/slides/slides.md", &conf.title), templates::md::MD_SLIDES)?;

            std::fs::create_dir_all(format!("{}/css", &conf.title))?;
            fs::write(format!("{}/css/custom.css", &conf.title), templates::md::MD_CUSTOM_CSS)?;

            std::fs::create_dir_all(format!("{}/public", &conf.title))?;
            fs::write(format!("{}/public/ferris.png", &conf.title), templates::md::MD_FERRIS)?;
            fs::write(format!("{}/public/favicon.png", &conf.title), templates::md::MD_FAVICON)?;
        }
        CLITemplate::HTML => {
            // fs::write(format!("{}/package.json", &conf.title), HTML_PACKAGE_JSON)?;
            fs::write(
                format!("{}/vite.config.js", &conf.title),
                templates::html::HTML_VITE_CONFIG_JS,
            )?;
            fs::write(format!("{}/index.html", &conf.title), templates::html::HTML_INDEX_HTML)?;
            fs::write(format!("{}/.gitignore", &conf.title), templates::html::HTML_GITIGNORE)?;

            std::fs::create_dir_all(format!("{}/src", &conf.title))?;
            fs::write(format!("{}/src/main.js", &conf.title), templates::html::HTML_MAIN_JS)?;

            std::fs::create_dir_all(format!("{}/slides", &conf.title))?;
            fs::write(format!("{}/slides/slides.html", &conf.title), templates::html::HTML_SLIDES)?;

            std::fs::create_dir_all(format!("{}/css", &conf.title))?;
            fs::write(format!("{}/css/custom.css", &conf.title), templates::html::HTML_CUSTOM_CSS)?;

            std::fs::create_dir_all(format!("{}/public", &conf.title))?;
            fs::write(format!("{}/public/ferris.png", &conf.title), templates::html::HTML_FERRIS)?;
            fs::write(format!("{}/public/favicon.png", &conf.title), templates::html::HTML_FAVICON)?;
        }
    }

    Ok(())
}

fn prompt_template() -> Result<CLITemplate, Box<dyn Error>> {
    let template_opts: Vec<CLITemplate> = CLITemplate::iter().collect();

    let chosen_template = inquire::Select::new("Select a template:", template_opts).prompt()?;

    Ok(chosen_template)
}
