use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumIter, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum Template {
    Markdown,
    HTML,
}
