use std::path::Path;

use crate::config;

pub mod check;
pub mod deploy;
pub mod dev;
pub mod export;
pub mod fork;
pub mod init;
pub mod run;
pub mod update;

pub fn is_cwd_oseda_project() -> bool {
    Path::new(config::CONFIG_FILE_NAME)
        .try_exists()
        .is_ok_and(|exists| exists)
}
