/*
npm init -y
npm install --save-dev vite http-server
npm install reveal.js serve vite-plugin-singlefile
touch vite.config.js -> add the plugin, write this by hand

*/

use std::{
    error::Error,
    fs::{self},
    process::Command,
};

use clap::Args;
use strum::IntoEnumIterator;

use crate::{config, template::Template};

/// Options for the `oseda init` command
#[derive(Args, Debug)]
pub struct InitOptions {
    /// Unused for now
    #[arg(long, required = false)]
    sentation_only: bool,
}

// embed all the static markdown template files into binary
const MD_VITE_CONFIG_JS: &str = include_str!("../static/md-templates/vite.config.js");
const MD_INDEX_HTML: &str = include_str!("../static/md-templates/index.html");
const MD_MAIN_JS: &str = include_str!("../static/md-templates/main.js");
const MD_SLIDES: &str = include_str!("../static/md-templates/slides.md");
const MD_CUSTOM_CSS: &str = include_str!("../static/md-templates/custom.css");
const MD_FERRIS: &[u8] = include_bytes!("../static/md-templates/ferris.png");


// do the same with the html templates
const HTML_VITE_CONFIG_JS: &str = include_str!("../static/html-templates/vite.config.js");
const HTML_INDEX_HTML: &str = include_str!("../static/html-templates/index.html");
const HTML_MAIN_JS: &str = include_str!("../static/html-templates/main.js");
const HTML_SLIDES: &str = include_str!("../static/html-templates/slides.html");
const HTML_CUSTOM_CSS: &str = include_str!("../static/html-templates/custom.css");
const HTML_FERRIS: &[u8] = include_bytes!("../static/html-templates/ferris.png");


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
pub fn init(_opts: InitOptions) -> Result<(), Box<dyn Error>> {
    let conf = config::create_conf()?;


    let template: Template = prompt_template()?;

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
        format!("install --save-dev vite http-server"),
        format!("install reveal.js serve vite-plugin-singlefile"),
        format!("install vite@5"),
        format!("install patch-package"),
    ];

    for c in npm_commands {
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
        println!("Bootstrapped npm {}", c);

    }

    println!("Saving config file...");

    config::write_config(&conf.title, &conf)?;

    // 99% sure we'll only ever have to maintain these two template schemas
    match template {
        Template::Markdown => {
            // fs::write(format!("{}/package.json", &conf.title), MD_PACKAGE_JSON)?;
            fs::write(format!("{}/vite.config.js", &conf.title), MD_VITE_CONFIG_JS)?;
            fs::write(format!("{}/index.html", &conf.title), MD_INDEX_HTML)?;

            std::fs::create_dir_all(format!("{}/src", &conf.title))?;
            fs::write(format!("{}/src/main.js", &conf.title), MD_MAIN_JS)?;

            std::fs::create_dir_all(format!("{}/slides", &conf.title))?;
            fs::write(format!("{}/slides/slides.md", &conf.title), MD_SLIDES)?;

            std::fs::create_dir_all(format!("{}/css", &conf.title))?;
            fs::write(format!("{}/css/custom.css", &conf.title), MD_CUSTOM_CSS)?;

            std::fs::create_dir_all(format!("{}/public", &conf.title))?;
            fs::write(format!("{}/public/ferris.png", &conf.title), MD_FERRIS)?;
        }
        Template::HTML => {

            // fs::write(format!("{}/package.json", &conf.title), HTML_PACKAGE_JSON)?;
            fs::write(format!("{}/vite.config.js", &conf.title), HTML_VITE_CONFIG_JS)?;
            fs::write(format!("{}/index.html", &conf.title), HTML_INDEX_HTML)?;

            std::fs::create_dir_all(format!("{}/src", &conf.title))?;
            fs::write(format!("{}/src/main.js", &conf.title), HTML_MAIN_JS)?;

            std::fs::create_dir_all(format!("{}/slides", &conf.title))?;
            fs::write(format!("{}/slides/slides.html", &conf.title), HTML_SLIDES)?;

            std::fs::create_dir_all(format!("{}/css", &conf.title))?;
            fs::write(format!("{}/css/custom.css", &conf.title), HTML_CUSTOM_CSS)?;

            std::fs::create_dir_all(format!("{}/public", &conf.title))?;
            fs::write(format!("{}/public/ferris.png", &conf.title), HTML_FERRIS)?
        },
    }

    Ok(())
}

fn prompt_template() -> Result<Template, Box<dyn Error>> {
    let template_opts: Vec<Template> = Template::iter().collect();

    let chosen_template = inquire::Select::new("Select a template:", template_opts).prompt()?;

    Ok(chosen_template)
}
