use crate::level::LevelObject;
use std::any::Any;

pub trait DeserializeLevelObject {
    fn deserialize(statement: String, args: Vec<String>) -> Result<Self, String>
    where
        Self: Sized;
}

pub trait SerializeLevelObject {
    fn serialize(&self) -> (String, Vec<String>);
}
