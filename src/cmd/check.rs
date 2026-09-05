use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

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
    MissingDescription(String),
    MissingTags(String),
}

impl std::error::Error for OsedaCheckError {}

/// Display options with more verbose messagess
impl std::fmt::Display for OsedaCheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingConfig(msg) => write!(f, "Missing config file: {}", msg),
            Self::BadConfig(msg) => write!(f, "Bad config file: {}", msg),
            Self::BadGitCredentials(msg) => write!(f, "Missing git credentials: {}", msg),
            Self::DirectoryNameMismatch(msg) => {
                write!(f, "Project name does not match directory: {}", msg)
            }
            Self::CouldNotPingLocalPresentation(msg) => {
                write!(f, "Could not ping localhost after project was ran: {}", msg)
            }
            Self::MissingDescription(msg) => {
                write!(f, "Config file is missing description: {}", msg)
            }
            Self::MissingTags(msg) => {
                write!(f, "No tags detected: {}", msg)
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
    let _conf = match config::read_and_validate_config() {
        Ok(conf) => conf,
        Err(err) => return OsedaProjectStatus::NotDeploymentReady(err),
    };

    let shutdown_flag = Arc::new(AtomicBool::new(false));
    let shutdown_flag_clone = shutdown_flag.clone();

    // use shutdown hook and kill once polled as alive
    let run_handle = std::thread::spawn(move || {
        let _ = run::run_with_shutdown(shutdown_flag_clone);
    });

    let addr = format!("http://localhost:{}", port_num);
    let mut status = None;

    // poll oseda run process at:
    let max_polls = 100;
    let poll_delay = Duration::from_millis(200);

    for i in 0..max_polls {
        println!("polled {}", i);
        if let Ok(res_status) = net::get_status(&addr) {
            if res_status == StatusCode::OK {
                status = Some(res_status);
                break;
            }
        }
        std::thread::sleep(poll_delay);
    }

    let status = match status {
        Some(status) => status,
        None => {
            // if could not get status, ensure process dies 
            shutdown_flag.store(true, Ordering::SeqCst);
            let _ = run_handle.join();
            return OsedaProjectStatus::NotDeploymentReady(
                OsedaCheckError::CouldNotPingLocalPresentation(
                    "Could not ping presentation".to_owned(),
                ),
            );
        }
    };

    if status != StatusCode::OK {
        // send shutdown flag, but happy about it this time
        shutdown_flag.store(true, Ordering::SeqCst);
        let _ = run_handle.join();
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

    // shutdown other process
    shutdown_flag.store(true, Ordering::SeqCst);
    if run_handle.join().is_err() {
        if kill_port(port_num).is_err() {
            println!("Warning: could not kill process on port, project could still be running");
        }
    } else {
        println!("Project process sucessfully terminated");
    }

    OsedaProjectStatus::DeployReady
}