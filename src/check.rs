use std::{error::Error, fs};

use clap::Args;

use crate::{github, init};

#[derive(Args, Debug)]
pub struct CheckOptions {
    #[arg(long, default_value_t = 3000)]
    port: u16,
}

#[derive(Debug)]
pub enum OsedaCheckError {
    DummyError(String),
    MissingConfig(String),
    BadConfig(String),
    BadGitCredentials(String),
}

impl std::error::Error for OsedaCheckError {}

impl std::fmt::Display for OsedaCheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DummyError(msg) => write!(f, "dummy error fixme {}", msg),
            Self::MissingConfig(msg) => write!(f, "Missing config file {}", msg),
            Self::BadConfig(msg) => write!(f, "Bad config file {}", msg),
            Self::BadGitCredentials(msg) => write!(f, "Missing git credentials {}", msg),
        }
    }
}

pub fn check(opts: CheckOptions) -> Result<(), OsedaCheckError> {
    // separate abstraction layer here, want the primary subcommand to call this
    // verify can also be called from deploy (in theory)
    match verify_project(opts.port) {
        OsedaProjectStatus::DeployReady => return Ok(()),
        OsedaProjectStatus::NotDeploymentReady(err) => return Err(err),
    }
}

pub enum OsedaProjectStatus {
    DeployReady,
    NotDeploymentReady(OsedaCheckError),
}

pub fn verify_project(port_num: u16) -> OsedaProjectStatus {
    println!("port was {}", port_num);
    // need to check
    // config
    // - [x] Verify it exists
    //  - [x] Verify valid json
    // - [ ] match author name to github name
    // - [ ] verify categories
    // - [ ] Title doesnt have spaces
    //  - [ ] (maybe an additional filter here?, scunthorpe filtering lol?)
    //
    // run
    // - [ ] should be able to sucessfully run project
    // - [ ] `oseda run &` and then like ping the host?
    // - [ ] but if we are pinging the host, we may need to make that configurable
    // - [ ] because the end user may wanna test on a diff port

    // assumes working directory is the project folder
    let config_str = match fs::read_to_string("oseda-config.json") {
        Ok(s) => s,
        // map_err?
        Err(_) => {
            return OsedaProjectStatus::NotDeploymentReady(OsedaCheckError::MissingConfig(
                format!(
                    "Could not find config file in {}",
                    // horrific, I think there is a better way to do this
                    // TODO fix this
                    std::env::current_dir().unwrap().to_str().unwrap()
                ),
            ));
        }
    };
    // now know the config exists
    //
    let conf: init::OsedaConfig = match serde_json::from_str(&config_str) {
        Ok(conf) => conf,
        Err(_) => {
            return OsedaProjectStatus::NotDeploymentReady(OsedaCheckError::BadConfig(
                "Could not parse oseda config file".to_owned(),
            ));
        }
    };

    // config is valid json

    let gh_name = match github::get_config("user.name") {
        Some(name) => name,
        None => {
            return OsedaProjectStatus::NotDeploymentReady(OsedaCheckError::BadGitCredentials(
                "Could not get git user.name from git config".to_owned(),
            ));
        }
    };

    if gh_name != conf.author {
        return OsedaProjectStatus::NotDeploymentReady(OsedaCheckError::BadGitCredentials(
            "Config author does not match git credentials".to_owned(),
        ));
    }

    return OsedaProjectStatus::DeployReady;
}
