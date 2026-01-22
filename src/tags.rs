use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumIter)]
pub enum Tag {
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
    Custom(String),
}

impl Tag {
    pub fn to_vec() -> Vec<Tag> {
        Tag::iter().collect()
    }
}
