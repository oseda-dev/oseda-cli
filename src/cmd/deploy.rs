use std::{env, error::Error, fs, path::Path};

use clap::Args;

use crate::github::{self, git};

#[derive(Args, Debug)]
pub struct DeployOptions {
    fork_url: String,
}

// FIXME: not the fork url. its the git@github.com:ReeseHatfield/oseda-lib-testing.git
pub fn deploy(opts: DeployOptions) -> Result<(), Box<dyn Error>> {
    let tmp_dir = tempfile::tempdir()?;
    let repo_path = tmp_dir.path();

    git(repo_path, &["clone", "--no-checkout", &opts.fork_url, "."])?;

    git(repo_path, &["sparse-checkout", "init", "--cone"])?;
    git(repo_path, &["sparse-checkout", "set", "courses"])?;
    git(repo_path, &["checkout"])?;

    let course_name = get_current_dir_name()?;
    let new_course_dir = repo_path.join("courses").join(&course_name);

    copy_dir_all(env::current_dir()?, &new_course_dir)?;

    git(repo_path, &["add", "."])?;
    git(repo_path, &["commit", "-m", "Add new course"])?;
    git(repo_path, &["push"])?;

    println!("Successfully deployed! You can now submit a PR.");

    Ok(())
}

// this is like really stupid to have this, since
// this logic is basically already used in `check`
// but really most of that logic should be moved to a config.rs file
// but until then, I am just reading the cwd with this
fn get_current_dir_name() -> Result<String, Box<dyn Error>> {
    let cwd = env::current_dir()?;
    let name = cwd
        .file_name()
        .ok_or("couldn't get directory name")?
        .to_string_lossy()
        .to_string();
    Ok(name)
}

// https://stackoverflow.com/questions/26958489/how-to-copy-a-folder-recursively-in-rust
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
