use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use std::{ffi::OsString, fs};

use chrono::{DateTime, Utc};
use inquire::validator::Validation;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use crate::categories::Category;
use crate::cmd::check::OsedaCheckError;
use crate::github;

/// Reads and validates an oseda-config.json file in the working directory
///
/// This checks a few things:
/// - the file exists and parses correctly
/// - the git `user.name` matches the config author (unless --skip-git is passed)
/// - the config `title` matches the name of the working directory
///
/// # Arguments
/// * `skip_git` - skips the git author validation, primarily used for CI, not by the end user hopefully lol
///
/// # Returns
/// * `Ok(OsedaConfig)` if the file is valid and all checks pass
/// * `Err(OsedaCheckError)` if any check fails
pub fn read_and_validate_config() -> Result<OsedaConfig, OsedaCheckError> {
    let config_str = fs::read_to_string("oseda-config.json").map_err(|_| {
        OsedaCheckError::MissingConfig(format!(
            "Could not find config file in {}",
            std::env::current_dir().unwrap().to_str().unwrap()
        ))
    })?;

    let conf: OsedaConfig = serde_json::from_str(&config_str)
        .map_err(|_| OsedaCheckError::BadConfig("Could not parse oseda config file".to_owned()))?;

    //https://stackoverflow.com/questions/73973332/check-if-were-in-a-github-action-travis-ci-circle-ci-etc-testing-environme
    let is_in_ci = std::env::var("GITHUB_ACTIONS").map_or(false, |v| v == "true");
    let skip_git = is_in_ci;

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

/// Structure for an oseda-config.json
#[derive(Serialize, Deserialize)]
pub struct OsedaConfig {
    pub title: String,
    pub author: String,
    pub category: Vec<Category>,
    // effectively mutable. Will get updated on each deployment
    pub last_updated: DateTime<Utc>,
}

/// Prompts the user for everything needed to generate a new OsedaConfig
///
/// # Returns
/// * `Ok(OsedaConfig)` containing validated project config options
/// * `Err` if a required input conf is invalid
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
        .ok_or("Could not get github username. Please ensure you are signed into github")?;

    Ok(OsedaConfig {
        title: title.trim().to_owned(),
        author: user_name,
        category: categories,
        last_updated: get_time(),
    })
}

/// Prompts user for categories associated with their Oseda project
///
/// # Returns
/// * `Ok(Vec<Category>)` with selected categories
/// * `Err` if the prompting went wrong somewhere
fn get_categories() -> Result<Vec<Category>, Box<dyn Error>> {
    let options: Vec<Category> = Category::iter().collect();

    let selected_categories =
        inquire::MultiSelect::new("Select categories", options.clone()).prompt()?;

    println!("You selected:");
    for category in selected_categories.iter() {
        println!("- {:?}", category);
    }

    Ok(selected_categories)
}

/// Updates the configs last-updated
/// Currently this is used on creation only, TODO fix this
///
/// # Arguments
/// * `conf` - a previously loaded or generated OsedaConfig
///
/// # Returns
/// * `Ok(())` if the file is successfully updated
/// * `Err` if file writing fails
pub fn update_time(mut conf: OsedaConfig) -> Result<(), Box<dyn Error>> {
    conf.last_updated = get_time();

    write_config(".", &conf)?;
    Ok(())
}

/// Gets the current system time in UTC
///
/// # Returns
/// * a `DateTime<Utc>` representing the current time
fn get_time() -> DateTime<Utc> {
    chrono::offset::Utc::now()
}

/// Write an OsedaConfig to the provided directory
///
/// # Arguments
/// * `path` - the directory path to write into
/// * `conf` - the `OsedaConfig` instance to serialize via serde
///
/// # Returns
/// * `Ok(())` if the file is written successfully
/// * `Err` if file creation or serialization fails
pub fn write_config(path: &str, conf: &OsedaConfig) -> Result<(), Box<dyn Error>> {
    let file = File::create(format!("{}/oseda-config.json", path))?;
    let writer = BufWriter::new(file);

    serde_json::to_writer_pretty(writer, &conf)?;

    Ok(())
}
