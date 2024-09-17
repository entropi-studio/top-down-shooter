use crate::level::LevelObject;
use bevy::math::Vec3;
use bevy::prelude::*;
use bevy::utils::default;
use lighting::prelude::{PointLight2d, PointLight2dBundle};

pub struct LampObjectPlugin;

impl Plugin for LampObjectPlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Component, Default)]
pub struct LampObject;

#[derive(Bundle)]
pub struct LampObjectBundle {
    light: PointLight2dBundle,
    object: LampObject,
    level_object: LevelObject,
}

impl LampObjectBundle {
    pub fn new(position: Vec2, radius: f32, intensity: f32) -> LampObjectBundle {
        Self {
            light: PointLight2dBundle {
                point_light: PointLight2d {
                    intensity,
                    radius,
                    cast_shadows: true,
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(position.x, position.y, 100.0)),
                ..default()
            },
            object: LampObject,
            level_object: LevelObject::Lamp {
                radius,
                intensity,
                falloff: 0.0,
            },
        }
    }
}
