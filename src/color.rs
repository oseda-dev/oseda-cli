use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumIter, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum Color {
    Black,
    White,
    Red,
    Green,
    Blue,
    Yellow,
    Cyan,
    Magenta,
    Gray,
    Silver,
    Maroon,
    Olive,
    Lime,
    Navy,
    Teal,
    Purple,
    Orange,
    Brown,
    Pink,
    Gold,
}

impl Color {
    pub fn into_hex(&self) -> String {
        match self {
            self::Color::Black => "#000000".to_owned(),
            self::Color::White => "#FFFFFF".to_owned(),
            self::Color::Red => "#FF0000".to_owned(),
            self::Color::Green => "#008000".to_owned(),
            self::Color::Blue => "#0000FF".to_owned(),
            self::Color::Yellow => "#FFFF00".to_owned(),
            self::Color::Cyan => "#00FFFF".to_owned(),
            self::Color::Magenta => "#FF00FF".to_owned(),
            self::Color::Gray => "#808080".to_owned(),
            self::Color::Silver => "#C0C0C0".to_owned(),
            self::Color::Maroon => "#800000".to_owned(),
            self::Color::Olive => "#808000".to_owned(),
            self::Color::Lime => "#00FF00".to_owned(),
            self::Color::Navy => "#000080".to_owned(),
            self::Color::Teal => "#008080".to_owned(),
            self::Color::Purple => "#800080".to_owned(),
            self::Color::Orange => "#FFA500".to_owned(),
            self::Color::Brown => "#A52A2A".to_owned(),
            self::Color::Pink => "#FFC0CB".to_owned(),
            self::Color::Gold => "#FFD700".to_owned(),
        }
    }
}
