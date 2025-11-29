use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumIter)]
pub enum Template {
    Markdown,
    HTML,
}
