use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumIter, EnumString)]
#[strum(ascii_case_insensitive)]
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
    // Custom(String),
}
// TODO document me
// Custom tags must be added by hand to the oseda-config.json

impl Tag {
    pub fn to_vec() -> Vec<Tag> {
        Tag::iter().collect()
    }
}
