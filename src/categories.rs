use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumIter)]
pub enum Category {
    Aerospace,
    Business,
    ComputerScience,
    Economics,
    Education,
    Engineering,
    Geography,
    HealthMedicine,
    History,
    LanguageArts,
    LiberalArts,
    Mathematics,
    Politics,
    Psychology,
    Science,
}

impl Category {
    pub fn to_vec() -> Vec<Category> {
        Category::iter().collect()
    }
}
