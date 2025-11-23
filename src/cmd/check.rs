use std::time::Duration;

use clap::Args;
use reqwest::StatusCode;

use crate::cmd::run;
use crate::config;

use crate::net::{self, kill_port};

/// Options for the `oseda check` command
#[derive(Args, Debug)]
pub struct CheckOptions {
    /// Port to check for the Oseda project on
    /// This is only useful if you have changed the default port that Oseda projects run on my default (3000)
    #[arg(long, default_value_t = 3000)]
    port: u16,
}
/// All common error types that could cause `oseda check` to fail
#[derive(Debug)]
pub enum OsedaCheckError {
    MissingConfig(String),
    BadConfig(String),
    BadGitCredentials(String),
    DirectoryNameMismatch(String),
    CouldNotPingLocalPresentation(String),
}

impl std::error::Error for OsedaCheckError {}

/// Display options with more verbose messagess
impl std::fmt::Display for OsedaCheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
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

/// Checks the Oseda project in the working directory for common oseda errors
///
/// # Arguments
/// * `opts` - options parsed from CLI flags
///
/// # Returns
/// * `Ok(())` if the project passes all checks and is considered as "deployabl"e
/// * `Err(OsedaCheckError)` a problem was detected that prevents the user from doing a deployment
pub fn check(opts: CheckOptions) -> Result<(), OsedaCheckError> {
    // separate abstraction layer here, want the primary subcommand to call this
    // verify can also be called from deploy (in theory)
    match verify_project(opts.port) {
        OsedaProjectStatus::DeployReady => Ok(()),
        OsedaProjectStatus::NotDeploymentReady(err) => Err(err),
    }
}

/// Status of Oseda project, plan to make this more verbose later
pub enum OsedaProjectStatus {
    DeployReady,
    NotDeploymentReady(OsedaCheckError),
}

/// Verifies a project passes all common checks
///
/// # Arguments
/// * `skip_git` - skips git authorship validation
/// * `port_num` - the port to check for the running project (defaults to 3000)
///
/// # Returns
/// * `OsedaProjectStatus::DeployReady` if the project passes all checks
/// * `OsedaProjectStatus::NotDeploymentReady(err)` if something fails that is commonly seen
fn verify_project(port_num: u16) -> OsedaProjectStatus {
    // TODO: document me -> assumes working directory is the project folder

    let _conf = match config::read_and_validate_config() {
        Ok(conf) => conf,
        Err(err) => return OsedaProjectStatus::NotDeploymentReady(err),
    };

    let _run_handle = std::thread::spawn(run::run);

    std::thread::sleep(Duration::from_millis(10000));

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

    println!("Project returned status code {:?}", status);

    // due to memory issues, no nice way to kill run_handle
    // eg -> no run_handle.kill();
    // so we'll go through the OS instead.
    // This can also be solved with an atomic boolean in run, this
    // would also get rid of the mpsc stuff going on in run(), but honestly
    // im just not that familiar with the mpsc pattern and rust api

    if kill_port(port_num).is_err() {
        println!("Warning: could not kill process on port, project could still be running");
    } else {
        println!("Project process sucessfully terminated");
    }

    OsedaProjectStatus::DeployReady
}
