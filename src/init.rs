/*
npm init -y
npm install --save-dev vite http-server
npm install reveal.js serve vite-plugin-singlefile
touch vite.config.js -> add the plugin, write this by hand

*/

use std::{error::Error, fs, panic::PanicHookInfo, path::Path, process::Command};

use clap::Args;

#[derive(Args, Debug)]
pub struct InitOptions {
    #[arg(required = true)]
    path: String,

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
    // if !Path::new(&opts.path).exists() {
    // npm init does not support a path as an argument for some reason

    // }

    println!("opts path {:?}", &opts.path);
    std::fs::create_dir_all(&opts.path)?;
    // Command::new("cd").arg(&opts.path).spawn()?;

    let _ = Command::new("npm")
        .args(format!("init -y --prefix {}", &opts.path).split(" "))
        .current_dir(&opts.path)
        .spawn()?
        .wait();

    let npm_commands = vec![
        format!("install --save-dev vite http-server"),
        format!("install reveal.js serve vite-plugin-singlefile"),
    ];

    for c in npm_commands {
        match Command::new("npm")
            .args(c.split(' '))
            .current_dir(&opts.path)
            .spawn()
        {
            Ok(mut handle) => {
                println!("Bootstrapping npm {}", c);
                let _ = handle.wait();
            }
            Err(err) => return Err(Box::new(err)),
        }
    }

    fs::write(format!("{}/package.json", &opts.path), PACKAGE_JSON)?;
    fs::write(format!("{}/vite.config.js", &opts.path), VITE_CONFIG_JS)?;
    fs::write(format!("{}/index.html", &opts.path), INDEX_HTML)?;

    std::fs::create_dir_all(format!("{}/src", &opts.path))?;
    fs::write(format!("{}/src/main.js", &opts.path), MAIN_JS)?;

    std::fs::create_dir_all(format!("{}/slides", &opts.path))?;
    fs::write(format!("{}/slides/slides.md", &opts.path), SLIDES_MD)?;

    std::fs::create_dir_all(format!("{}/css", &opts.path))?;
    fs::write(format!("{}/css/custom.css", &opts.path), CUSTOM_CSS)?;

    Ok(())
}
