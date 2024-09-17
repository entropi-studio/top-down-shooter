use crate::level::serializers::wall::deserialize_wall;
use crate::level::{DeserializeLevelObject, SerializeLevelObject};
use bevy::math::Vec2;
use bevy::prelude::Component;
use crate::level::serializers::lamp::deserialize_lamp;

#[derive(Component, Copy, Clone)]
pub enum LevelObject {
    Wall {
        size: Vec2,
        position: Vec2,
        rotation: f32,
    },
    Lamp {
        radius: f32,
        intensity: f32,
        falloff: f32,
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
            LevelObject::Lamp {
                radius,
                intensity,
                falloff,
            } => (
                "Lamp".to_string(),
                vec![
                    radius.to_string(),
                    intensity.to_string(),
                    falloff.to_string(),
                ],
            ),
        }
    }
}

impl DeserializeLevelObject for LevelObject {
    fn deserialize(statement: String, args: Vec<String>) -> Result<Self, String> {
        match statement.as_str() {
            "Wall" => deserialize_wall(args),
            "Lamp" => deserialize_lamp(args),
            _ => Err(format!(
                "[#LevelObject::deserialize] unknown statement: {}",
                statement
            )),
        }
    }
}
