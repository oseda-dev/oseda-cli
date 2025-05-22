/*
npm init -y
npm install --save-dev vite http-server
npm install reveal.js serve vite-plugin-singlefile
touch vite.config.js -> add the plugin, write this by hand

*/

use std::{
    error::Error,
    fs::{self, File},
    io::{self, BufWriter, Write, stdin},
    panic::PanicHookInfo,
    path::{Path, PathBuf},
    process::Command,
};

use chrono::{DateTime, Utc};
use clap::Args;
use serde::Serialize;

use crate::github;

#[derive(Args, Debug)]
pub struct InitOptions {
    #[arg(long, required = false)]
    presentation_only: bool,
}

const PACKAGE_JSON: &str = include_str!("static/package.json");
const VITE_CONFIG_JS: &str = include_str!("static/vite.config.js");
const INDEX_HTML: &str = include_str!("static/index.html");
const MAIN_JS: &str = include_str!("static/main.js");
const SLIDES_MD: &str = include_str!("static/slides.md");
const CUSTOM_CSS: &str = include_str!("static/custom.css");

pub fn init(opts: InitOptions) -> Result<(), Box<dyn Error>> {
    // path/[conf.title]

    let conf = create_conf()?;

    // println!("opts path {:?}", &opts.path);
    std::fs::create_dir_all(&conf.title)?;
    // Command::new("cd").arg(&opts.path).spawn()?;

    let output = Command::new("npm")
        .args(["init", "-y", "--prefix", &conf.title])
        .current_dir(&conf.title)
        .output()?;

    // swapped to explicit check so it doesnt hang after
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

        println!("Saving config file...");

        let file = File::create(format!("{}/oseda-config.json", conf.title))?;
        let writer = BufWriter::new(file);

        // Serialize to JSON and write to file
        serde_json::to_writer_pretty(writer, &conf)?;
    }

    fs::write(format!("{}/package.json", &conf.title), PACKAGE_JSON)?;
    fs::write(format!("{}/vite.config.js", &conf.title), VITE_CONFIG_JS)?;
    fs::write(format!("{}/index.html", &conf.title), INDEX_HTML)?;

    std::fs::create_dir_all(format!("{}/src", &conf.title))?;
    fs::write(format!("{}/src/main.js", &conf.title), MAIN_JS)?;

    std::fs::create_dir_all(format!("{}/slides", &conf.title))?;
    fs::write(format!("{}/slides/slides.md", &conf.title), SLIDES_MD)?;

    std::fs::create_dir_all(format!("{}/css", &conf.title))?;
    fs::write(format!("{}/css/custom.css", &conf.title), CUSTOM_CSS)?;

    Ok(())
}

#[derive(Serialize)]
struct OsedaConfig {
    title: String,
    author: String,
    category: Vec<Category>,
    // effectively mutable. Will get updated on each deployment
    last_updated: DateTime<Utc>,
}

fn create_conf() -> Result<OsedaConfig, Box<dyn Error>> {
    print!("Title: ");
    io::stdout().flush()?;

    let mut title = String::new();
    std::io::stdin().read_line(&mut title)?;

    let categories = get_categories()?;

    let user_name = github::get_config("user.name")
        .ok_or_else(|| "Could not get github username. Please ensure you are signed into github")?;

    Ok(OsedaConfig {
        title: title.trim().to_owned(),
        author: user_name,
        category: categories,
        last_updated: chrono::offset::Utc::now(),
    })
}

#[derive(Serialize, Debug, Clone, Copy)]
enum Category {
    ComputerScience,
    Engineering,
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn get_categories() -> Result<Vec<Category>, Box<dyn Error>> {
    // really ugly but idk a better way to do this for now w/o a macro
    let options = vec![Category::ComputerScience, Category::Engineering];

    let selected_categories =
        inquire::MultiSelect::new("Select categories", options.clone()).prompt()?;

    println!("You selected:");
    for category in selected_categories.iter().copied() {
        println!("- {:?}", category);
    }

    return Ok(selected_categories);
}
