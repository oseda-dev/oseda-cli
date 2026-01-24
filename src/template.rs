use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumIter, EnumString)]
pub enum Template {
    Markdown,
    HTML,
}
