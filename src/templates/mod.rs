pub mod templater;

pub mod html;
pub mod md;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Debug, thiserror::Error)]
pub enum RendererError {
    #[error("I/O error while rendering template: {0}")]
    Io(#[from] std::io::Error),

    #[error("Could not render template: {0}")]
    CouldNotWriteTemplate(String),

    #[error("Unknown error while rendering template")]
    Unknown,
}

pub trait Renderer {
    fn write_to_fs(&self, target_dir: &str) -> Result<(), RendererError>;
}

pub fn render_template(text: &str, params: &HashMap<String, String>) -> String {
    let mut update_me = String::from(text);

    // { is escaped with {{, repeat for both halves
    params.iter().for_each(|(key, value)| {
        update_me = update_me.replace(&format!("{{{{ {key} }}}}"), value);
    });
    update_me
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumIter, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum CLITemplate {
    HTML,
    Markdown,
}
