/*
user.email=hatfield.69@wright.edu
user.name=ReeseHatfield
core.editor=/usr/bin/vim
*/

use std::{error::Error, path::Path, process::Command};

/// Gets a value from the users local git configuration, see example
///
/// # Arguments
/// * `key` - the config key to look for -> e.g core.editor
///
/// # Returns
/// * `Some(String)` with the value if the key is found
/// * `None` on key retrieval failure
///
/// # Example
/// ```
/// use oseda_cli::github::get_config_from_user_git;
///
/// let name = get_config_from_user_git("user.name");
/// ```
pub fn get_config_from_user_git(key: &str) -> Option<String> {
    let handle = Command::new("git")
        .arg("config")
        .arg("list")
        .output()
        .ok()?;

    let conf_out = String::from_utf8(handle.stdout).ok()?;

    get_key_from_conf(key, &conf_out)
}

fn get_key_from_conf(key: &str, conf: &String) -> Option<String> {
    conf.split("\n").map(|s| s.trim()).find_map(|line| {
        let (cur_key, value) = line.split_once('=')?;
        if cur_key == key {
            Some(value.to_string())
        } else {
            None
        }
    })
}

/// Super generic run git func for general usecases git commands
///
/// # Arguments
/// * `dir` - the directory to run the git command in
/// * `args` - the list of arguments to pass to git -> e.g. `["clone", "[URL]" ]`
///
/// # Returns (based on git exit code)
/// * `Ok(())` if the git command succeeds
/// * `Err` if the command fails
pub fn git(dir: &Path, args: &[&str]) -> Result<(), Box<dyn Error>> {
    let status = Command::new("git").current_dir(dir).args(args).status()?;

    if !status.success() {
        return Err(format!("git {:?} failed", args).into());
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_git_config_kv_pair() {
        let config = r#"
            user.email=john.doe@ucla.edu
            user.name=JohnathanD
            core.editor=/usr/bin/vim
            core.filemode=true
            core.bare=false
            core.logallrefupdates=true
        "#
        .to_string();

        let email = get_key_from_conf("user.email", &config);

        assert!(email.is_some());

        assert_eq!(email.unwrap(), "john.doe@ucla.edu");
    }
}
