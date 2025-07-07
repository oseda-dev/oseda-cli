use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

/// All possible course categoes
/// TODO make this better
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumIter)]
pub enum Category {
    ComputerScience,
    Engineering,
}

impl Category {
    pub fn to_vec() -> Vec<Category> {
        Category::iter().collect()
    }
}
