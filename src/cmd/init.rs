use std::{collections::HashMap, error::Error, process::Command, str::FromStr};

use clap::Args;
use spinners::{Spinner, Spinners};
use strum::IntoEnumIterator;

use crate::{
    config,
    templates::{templater::Templater, CLITemplate, Renderer},
};

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

    // empty hashmap for now
    let mut params: HashMap<String, String> = HashMap::new();
    params.insert(String::from("TITLE"), conf.title.clone());

    let templater = Templater::new(template, params);
    templater.write_to_fs(&conf.title)?;

    Ok(())
}

fn prompt_template() -> Result<CLITemplate, Box<dyn Error>> {
    let template_opts: Vec<CLITemplate> = CLITemplate::iter().collect();

    let chosen_template = inquire::Select::new("Select a template:", template_opts).prompt()?;

    Ok(chosen_template)
}
