use std::fmt::{Display, Formatter};

#[cfg(feature = "backend")]
use rocket::form::FromFormField;

use crate::enums::EnumName;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "backend", derive(FromFormField))]
pub enum TileSet {
    Yellow,
    Red,
    Black,
    MartinPersson,
}

impl EnumName for TileSet {
    fn name(&self) -> &'static str {
        match self {
            TileSet::Yellow => "Yellow",
            TileSet::Red => "Red",
            TileSet::Black => "Black",
            TileSet::MartinPersson => "MartinPersson",
        }
    }
}

impl Default for TileSet {
    fn default() -> Self {
        Self::Yellow
    }
}

impl Display for TileSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TileSet::Yellow => {
                write!(f, "Yellow")
            }
            TileSet::Red => {
                write!(f, "Red")
            }
            TileSet::Black => {
                write!(f, "Black")
            }
            TileSet::MartinPersson => {
                write!(f, "Martin Persson")
            }
        }
    }
}
