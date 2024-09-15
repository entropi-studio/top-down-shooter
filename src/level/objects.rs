use crate::level::serializers::wall::serialize_wall;
use crate::level::{DeserializeLevelObject, SerializeLevelObject};
use bevy::math::Vec2;

pub enum LevelObject {
    Wall {
        size: Vec2,
        position: Vec2,
        rotation: f32,
    },
}

impl SerializeLevelObject for LevelObject {
    fn serialize(&self) -> (String, Vec<String>) {
        match self {
            LevelObject::Wall {
                size,
                position,
                rotation,
            } => (
                "Wall".into(),
                vec![
                    size.x.to_string(),
                    size.y.to_string(),
                    position.x.to_string(),
                    position.y.to_string(),
                    rotation.to_string(),
                ],
            ),
        }
    }
}

impl DeserializeLevelObject for LevelObject {
    fn deserialize(statement: String, args: Vec<String>) -> Result<Self, String> {
        match statement.as_str() {
            "Wall" => serialize_wall(args),
            _ => Err(format!(
                "[#LevelObject::deserialize] unknown statement: {}",
                statement
            )),
        }
    }
}
