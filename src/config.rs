use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use std::{ffi::OsString, fs};

use chrono::{DateTime, Utc};
use inquire::validator::Validation;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use crate::categories::Tag;
use crate::cmd::check::OsedaCheckError;
use crate::color::Color;
use crate::github;

pub fn read_config_file<P: AsRef<std::path::Path>>(
    path: P,
) -> Result<OsedaConfig, OsedaCheckError> {
    let config_str = fs::read_to_string(path.as_ref()).map_err(|_| {
        OsedaCheckError::MissingConfig(format!(
            "Could not find config file in {}",
            path.as_ref().display()
        ))
    })?;

    let conf: OsedaConfig = serde_json::from_str(&config_str)
        .map_err(|_| OsedaCheckError::BadConfig("Could not parse oseda config file".to_owned()))?;

    Ok(conf)
}

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
    let path = std::env::current_dir().map_err(|_| {
        OsedaCheckError::DirectoryNameMismatch("Could not get path of working directory".to_owned())
    })?;

    let config_path = path.join("oseda-config.json");

    let conf = read_config_file(config_path)?;

    let in_ci = std::env::var("GITHUB_ACTIONS").is_ok_and(|v| v == "true");
    let skip_git = in_ci;

    validate_config(&conf, &path, skip_git, || {
        github::get_config_from_user_git("user.name")
    })?;

    Ok(conf)
}

pub fn validate_config(
    conf: &OsedaConfig,
    current_dir: &std::path::Path,
    skip_git: bool,
    // very cool pass in a lambda, swap that lambda out in the tests
    // https://danielbunte.medium.com/a-guide-to-testing-and-mocking-in-rust-a73d022b4075
    get_git_user: impl Fn() -> Option<String>,
) -> Result<(), OsedaCheckError> {
    if !skip_git {
        let gh_name = get_git_user().ok_or_else(|| {
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

    let cwd = current_dir.file_name().ok_or_else(|| {
        OsedaCheckError::DirectoryNameMismatch("Could not resolve path name".to_owned())
    })?;

    if cwd != OsString::from(conf.title.clone()) {
        return Err(OsedaCheckError::DirectoryNameMismatch(
            "Config title does not match directory name".to_owned(),
        ));
    }

    Ok(())
}

/// Structure for an oseda-config.json
#[derive(Serialize, Deserialize)]
pub struct OsedaConfig {
    pub title: String,
    pub author: String,
    pub category: Vec<Tag>,
    // effectively mutable. Will get updated on each deployment
    pub last_updated: DateTime<Utc>,
    pub color: String,
}

/// Prompts the user for everything needed to generate a new OsedaConfig
///
/// # Returns
/// * `Ok(OsedaConfig)` containing validated project config options
/// * `Err` if a required input conf is invalid
pub fn create_conf() -> Result<OsedaConfig, Box<dyn Error>> {
    // let mut title = String::new();
    // std::io::std        in().read_line(&mut title)?;

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
    let color = get_color()?;

    let user_name = github::get_config_from_user_git("user.name")
        .ok_or("Could not get github username. Please ensure you are signed into github")?;

    Ok(OsedaConfig {
        title: title.trim().to_owned(),
        author: user_name,
        category: categories,
        last_updated: get_time(),
        color: color.into_hex(),
    })
}

/// Prompts user for categories associated with their Oseda project
///
/// # Returns
/// * `Ok(Vec<Category>)` with selected categories
/// * `Err` if the prompting went wrong somewhere
fn get_categories() -> Result<Vec<Tag>, Box<dyn Error>> {
    let options: Vec<Tag> = Tag::iter().collect();

    let selected_categories =
        inquire::MultiSelect::new("Select categories (type to search):", options.clone())
            .prompt()?;

    println!("You selected:");
    for category in selected_categories.iter() {
        println!("- {:?}", category);
    }

    Ok(selected_categories)
}

fn get_color() -> Result<Color, Box<dyn Error>> {
    let options: Vec<Color> = Color::iter().collect();

    let selected_color = inquire::Select::new(
        "Select the color for your course (type to search):",
        options.clone(),
    )
    .prompt()?;

    println!("You selected: {:?}", selected_color);

    Ok(selected_color)
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
/// # Returns            color: Color::Black
/// * `Ok(())` if the file is written successfully
/// * `Err` if file creation or serialization fails
pub fn write_config(path: &str, conf: &OsedaConfig) -> Result<(), Box<dyn Error>> {
    let file = File::create(format!("{}/oseda-config.json", path))?;
    let writer = BufWriter::new(file);

    serde_json::to_writer_pretty(writer, &conf)?;

    Ok(())
}

#[cfg(test)]
mod test {
    use std::path::Path;
    use tempfile::tempdir;

    use super::*;

    #[allow(dead_code)]
    fn mock_config_json() -> String {
        r#"
           {
               "title": "TestableRust",
               "author": "JaneDoe",
               "category": ["ComputerScience"],
               "last_updated": "2024-07-10T12:34:56Z"
           }
           "#
        .trim()
        .to_string()
    }

    #[test]
    fn test_read_config_file_missing() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("oseda-config.json");

        let result = read_config_file(&config_path);
        assert!(matches!(result, Err(OsedaCheckError::MissingConfig(_))));
    }

    #[test]
    fn test_validate_config_success() {
        let conf = OsedaConfig {
            title: "my-project".to_string(),
            author: "JaneDoe".to_string(),
            category: vec![Tag::ComputerScience],
            last_updated: chrono::Utc::now(),
            color: Color::Black.into_hex(),
        };

        let fake_dir = Path::new("/tmp/my-project");
        // can mock the git credentials easier
        let result = validate_config(&conf, fake_dir, false, || Some("JaneDoe".to_string()));

        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_config_bad_git_user() {
        let conf = OsedaConfig {
            title: "my-project".to_string(),
            author: "JaneDoe".to_string(),
            category: vec![Tag::ComputerScience],
            last_updated: chrono::Utc::now(),
            color: Color::Black.into_hex(),
        };

        let fake_dir = Path::new("/tmp/oseda");

        let result = validate_config(&conf, fake_dir, false, || Some("NotJane".to_string()));

        assert!(matches!(result, Err(OsedaCheckError::BadGitCredentials(_))));
    }

    #[test]
    fn test_validate_config_bad_dir_name() {
        let conf = OsedaConfig {
            title: "correct-name".to_string(),
            author: "JaneDoe".to_string(),
            category: vec![Tag::ComputerScience],
            last_updated: chrono::Utc::now(),
            color: Color::Black.into_hex(),
        };

        let fake_dir = Path::new("/tmp/wrong-name");

        let result = validate_config(&conf, fake_dir, false, || Some("JaneDoe".to_string()));
        assert!(matches!(
            result,
            Err(OsedaCheckError::DirectoryNameMismatch(_))
        ));
    }

    #[test]
    fn test_validate_config_skip_git() {
        let conf = OsedaConfig {
            title: "oseda".to_string(),
            author: "JaneDoe".to_string(),
            category: vec![Tag::ComputerScience],
            last_updated: chrono::Utc::now(),
            color: Color::Black.into_hex(),
        };

        let fake_dir = Path::new("/tmp/oseda");

        let result = validate_config(&conf, fake_dir, true, || None);
        assert!(result.is_ok());
    }
}
