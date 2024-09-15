use bevy::color::color_difference::EuclideanDistance;
use bevy::ecs::observer::TriggerTargets;
use bevy::math::{Quat, Vec3, VectorSpace};
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::utils::default;
use bevy_light_2d::light::{PointLight2d, PointLight2dBundle};
use bevy_light_2d::occluder::LightOccluder2dShape;
use bevy_light_2d::prelude::LightOccluder2d;
use rand::Rng;

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
}

impl LampObjectBundle {
    pub fn new(position: Vec2, radius: f32, intensity: f32) -> LampObjectBundle {
        Self {
            light: PointLight2dBundle {
                point_light: PointLight2d {
                    radius,
                    intensity,
                    cast_shadows: true,
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(position.x, position.y, 100.0)),
                ..default()
            },
            object: LampObject,
        }
    }
}
