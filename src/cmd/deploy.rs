use std::{env, error::Error, fs, path::Path};

use clap::Args;

use crate::{config, github::git};

/// Options for the `oseda deploy` command
#[derive(Args, Debug)]
pub struct DeployOptions {
    fork_url: String,
}

struct SshUrl(String);

/// string deref
impl std::ops::Deref for SshUrl {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Convert a standard HTTPS GitHub URL to SSH format
///
/// # Arguments
/// * `value` - a String starting with `https://github.com/...`
///
/// # Returns
/// * `Ok(SshUrl)` if parsing succeeds
/// * `Err` if the format is not recognized
impl TryFrom<String> for SshUrl {
    type Error = Box<dyn Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        // https://github.com/ReeseHatfield/oseda-lib-testing/
        // into
        // git@github.com:ReeseHatfield/oseda-lib-testing.git
        let suffix = value
            .strip_prefix("https://github.com/")
            .ok_or("Could not get SSH URL")?;

        Ok(SshUrl(format!(
            "git@github.com:{}.git",
            suffix.trim_end_matches('/')
        )))
    }
}

/// Deploys an Oseda project to the provided fork URL
///
/// # Arguments
/// * `opts` - options with the `fork_url` for the deployment target
///
/// # Returns
/// * `Ok(())` on success
/// * `Err` if any git, file, or config step fails, including a check failsure
pub fn deploy(opts: DeployOptions) -> Result<(), Box<dyn Error>> {
    let tmp_dir = tempfile::tempdir()?;
    let repo_path = tmp_dir.path();

    let ssh_url: SshUrl = opts.fork_url.try_into()?;

    git(
        repo_path,
        &["clone", "--no-checkout", ssh_url.0.as_str(), "."],
    )?;

    println!("Running git with sparse checkout");
    git(repo_path, &["sparse-checkout", "init", "--cone"])?;
    git(repo_path, &["sparse-checkout", "set", "courses"])?;
    git(repo_path, &["checkout"])?;

    let course_name = get_current_dir_name()?;
    let new_course_dir = repo_path.join("courses").join(&course_name);

    copy_dir_all(env::current_dir()?, &new_course_dir)?;

    // bails if config is bad
    //
    // force a no-skip-git
    let conf = config::read_and_validate_config()?;

    config::update_time(conf)?;
    println!("Committing files to remote...");
    git(repo_path, &["add", "."])?;
    git(repo_path, &["commit", "-m", "Add new course"])?;
    git(repo_path, &["push"])?;

    println!("Project successfully pushed to remote.");

    Ok(())
}

/// Util fn to get the current working directory name
///
/// # Returns
/// * `Ok(String)` with the directory name
/// * `Err` if the name failed to be extracted
fn get_current_dir_name() -> Result<String, Box<dyn Error>> {
    // this is like really stupid to have this, since
    // this logic is basically already used in `check`
    // but really most of that logic should be moved to a config.rs file
    // but until then, I am just reading the cwd with this
    let cwd = env::current_dir()?;
    let name = cwd
        .file_name()
        .ok_or("couldn't get directory name")?
        .to_string_lossy()
        .to_string();
    Ok(name)
}

/// Recursively copy a directory
/// https://stackoverflow.com/questions/26958489/how-to-copy-a-folder-recursively-in-rust
fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
