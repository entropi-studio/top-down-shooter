use crate::level::LevelObject;
use std::fmt::{write, Display, Formatter};
use strum_macros::EnumString;

pub enum LevelTokens {
    Version(String),
    Object(LevelObject),

    Unknown,
}

pub enum LevelTokenParseError {
    UnknownToken(String),
    InvalidArgsSize(usize, usize),
    InvalidLine
}

#[derive(EnumString)]
pub enum LevelStatements {
    Version,
}
