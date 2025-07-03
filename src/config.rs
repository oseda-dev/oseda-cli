use std::error::Error;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::{ffi::OsString, fs};

use chrono::{DateTime, Utc};
use inquire::validator::Validation;
use reqwest::blocking::get;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use crate::categories::Category;
use crate::cmd::check::OsedaCheckError;
use crate::github;

pub fn read_and_validate_config(skip_git: bool) -> Result<OsedaConfig, OsedaCheckError> {
    let config_str = fs::read_to_string("oseda-config.json").map_err(|_| {
        OsedaCheckError::MissingConfig(format!(
            "Could not find config file in {}",
            std::env::current_dir().unwrap().to_str().unwrap()
        ))
    })?;

    let conf: OsedaConfig = serde_json::from_str(&config_str)
        .map_err(|_| OsedaCheckError::BadConfig("Could not parse oseda config file".to_owned()))?;

    if !skip_git {
        println!("Running git checks");
        let gh_name = github::get_config("user.name").ok_or_else(|| {
            OsedaCheckError::BadGitCredentials(
                "Could not get git user.name from git config".to_owned(),
            )
        })?;

        if gh_name != conf.author {
            return Err(OsedaCheckError::BadGitCredentials(
                "Config author does not match git credentials".to_owned(),
            ));
        }
    }

    let path = std::env::current_dir().map_err(|_| {
        OsedaCheckError::DirectoryNameMismatch("Could not get path of working directory".to_owned())
    })?;

    let cwd = path.file_name().ok_or_else(|| {
        OsedaCheckError::DirectoryNameMismatch("Could not resolve path name".to_owned())
    })?;

    if cwd != OsString::from(conf.title.clone()) {
        return Err(OsedaCheckError::DirectoryNameMismatch(
            "Config title does not match directory name".to_owned(),
        ));
    }

    Ok(conf)
}

#[derive(Serialize, Deserialize)]
pub struct OsedaConfig {
    pub title: String,
    pub author: String,
    pub category: Vec<Category>,
    // effectively mutable. Will get updated on each deployment
    pub last_updated: DateTime<Utc>,
}

pub fn create_conf() -> Result<OsedaConfig, Box<dyn Error>> {
    // let mut title = String::new();
    // std::io::stdin().read_line(&mut title)?;

    let validator = |input: &str| {
        if input.chars().count() < 2 {
            Ok(Validation::Invalid(
                ("Title must be longer than two characters").into(),
            ))
        } else {
            Ok(Validation::Valid)
        }
    };

    let mut title = inquire::Text::new("Title: ")
        .with_validator(validator)
        .prompt()?;

    title = title.replace(" ", "-");

    let categories = get_categories()?;

    let user_name = github::get_config("user.name")
        .ok_or_else(|| "Could not get github username. Please ensure you are signed into github")?;

    Ok(OsedaConfig {
        title: title.trim().to_owned(),
        author: user_name,
        category: categories,
        last_updated: get_time(),
    })
}

fn get_categories() -> Result<Vec<Category>, Box<dyn Error>> {
    let options: Vec<Category> = Category::iter().collect();

    let selected_categories =
        inquire::MultiSelect::new("Select categories", options.clone()).prompt()?;

    println!("You selected:");
    for category in selected_categories.iter().copied() {
        println!("- {:?}", category);
    }

    return Ok(selected_categories);
}

pub fn update_time(mut conf: OsedaConfig) -> Result<(), Box<dyn Error>> {
    conf.last_updated = get_time();

    write_config(".", &conf)?;
    Ok(())
}

fn get_time() -> DateTime<Utc> {
    chrono::offset::Utc::now()
}

pub fn write_config(path: &str, conf: &OsedaConfig) -> Result<(), Box<dyn Error>> {
    let file = File::create(format!("{}/oseda-config.json", path))?;
    let writer = BufWriter::new(file);

    serde_json::to_writer_pretty(writer, &conf)?;

    Ok(())
}
