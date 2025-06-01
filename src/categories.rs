use std::fmt;

use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumIter)]
pub enum Category {
    ComputerScience,
    Engineering,
}

impl Category {
    pub fn to_vec() -> Vec<Category> {
        return Category::iter().collect();
    }
}
