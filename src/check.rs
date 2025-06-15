use std::{error::Error, ffi::OsString, fs, io::stdout, path, time::Duration};

use clap::Args;
use reqwest::StatusCode;

use crate::{
    categories::Category,
    github, init,
    net::{self, kill_port},
    run::run,
};

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
    DirectoryNameMismatch(String),
    CouldNotPingLocalPresentation(String),
}

impl std::error::Error for OsedaCheckError {}

impl std::fmt::Display for OsedaCheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DummyError(msg) => write!(f, "dummy error fixme {}", msg),
            Self::MissingConfig(msg) => write!(f, "Missing config file {}", msg),
            Self::BadConfig(msg) => write!(f, "Bad config file {}", msg),
            Self::BadGitCredentials(msg) => write!(f, "Missing git credentials {}", msg),
            Self::DirectoryNameMismatch(msg) => {
                write!(f, "Project name does not match directory {}", msg)
            }
            Self::CouldNotPingLocalPresentation(msg) => {
                write!(f, "Could not ping localhost after project was ran {}", msg)
            }
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
    // - [x] mjthub name
    // - [x] verify categories -> auto checked when config is parsed
    // - [x] Title doesnt have spaces
    // - [x] Title matches directory name
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

    let path = match std::env::current_dir() {
        Ok(path) => path,
        Err(_) => {
            return OsedaProjectStatus::NotDeploymentReady(OsedaCheckError::DirectoryNameMismatch(
                "Could not get path of working directory".to_owned(),
            ));
        }
    };

    let cwd = if let Some(cwd) = path.file_name() {
        cwd
    } else {
        return OsedaProjectStatus::NotDeploymentReady(OsedaCheckError::DirectoryNameMismatch(
            "Could not resolve path name".to_owned(),
        ));
    };

    if cwd != OsString::from(conf.title) {
        return OsedaProjectStatus::NotDeploymentReady(OsedaCheckError::DirectoryNameMismatch(
            "Config title does not match directory name".to_owned(),
        ));
    }

    let run_handle = std::thread::spawn(move || run());

    std::thread::sleep(Duration::from_millis(2500));

    let addr = format!("http://localhost:{}", port_num);
    let status = match net::get_status(&addr) {
        Ok(status) => status,
        Err(_) => {
            return OsedaProjectStatus::NotDeploymentReady(
                OsedaCheckError::CouldNotPingLocalPresentation(
                    "Could not ping presentation".to_owned(),
                ),
            );
        }
    };

    if status != StatusCode::OK {
        return OsedaProjectStatus::NotDeploymentReady(
            OsedaCheckError::CouldNotPingLocalPresentation(
                "Presentation returned non 200 error status code".to_owned(),
            ),
        );
    }

    println!("STATUS WAS: {:?}", status);

    // due to memory issues, no nice way to kill run_handle
    // run_handle.kill();
    // so we'll go through the OS instead.
    // This can also be solved with an atomic boolean in run, this
    // would also get rid of the mpsc stuff going on in run(), but honestly
    // im just not that familiar with the mpsc pattern and rust api

    if kill_port(port_num).is_err() {
        println!("Warning: could not kill process on port, project could still be running");
    } else {
        println!("killed that proc");
    }

    println!("cwd is {:?}", cwd);

    // do ping shit
    return OsedaProjectStatus::DeployReady;
}
