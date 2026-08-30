use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumIter, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum DefinedTag {
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

impl From<DefinedTag> for String {
    fn from(tag: DefinedTag) -> Self {
        tag.to_string()
    }
}

impl DefinedTag {
    pub fn to_vec() -> Vec<DefinedTag> {
        DefinedTag::iter().collect()
    }
}
