use std::fmt::{Display, Formatter};

#[cfg(feature = "backend")]
use rocket::form::FromFormField;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "backend", derive(FromFormField))]
pub enum TileSet {
    Yellow,
    Red,
    Black,
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
        }
    }
}
