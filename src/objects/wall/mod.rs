use crate::level::LevelObject;
use bevy::math::{Quat, Vec3, VectorSpace};
use bevy::prelude::*;
use bevy::utils::default;
use lighting::prelude::{LightOccluder2d, LightOccluder2dShape};

pub struct WallObjectPlugin;

impl Plugin for WallObjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate);
    }
}

#[derive(Component, Default)]
pub struct WallObject;
#[derive(Component, Default)]
struct WallObjectAnimated {
    current_color: Color,
    current_scale: f32,
    to_color: Color,
}

#[derive(Bundle)]
pub struct WallObjectBundle {
    mesh: ColorMesh2dBundle,
    occluder: LightOccluder2d,
    animated: WallObjectAnimated,
    object: WallObject,
    level_object: LevelObject,
}

impl WallObjectBundle {
    pub fn new(
        position: Vec2,
        size: Vec2,
        rotation: f32,
        z: f32,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) -> WallObjectBundle {
        let translation = Vec3::new(position.x, position.y, z);
        let color_initial = Color::srgba(1.0, 1.0, 1.0, 0.0);
        let scale_initial = 1.5;
        Self {
            mesh: ColorMesh2dBundle {
                mesh: meshes.add(Rectangle::new(size.x, size.y)).into(),
                material: materials.add(color_initial),
                transform: Transform {
                    translation,
                    rotation: Quat::from_rotation_z(rotation),
                    scale: Vec3::splat(scale_initial),
                },
                ..default()
            },
            occluder: LightOccluder2d {
                shape: LightOccluder2dShape::Rectangle {
                    half_size: size / 2.0,
                },
            },
            animated: WallObjectAnimated {
                current_color: color_initial,
                current_scale: scale_initial,
                to_color: Color::WHITE,
            },
            object: WallObject,
            level_object: LevelObject::Wall {
                size,
                position,
                rotation,
            },
        }
    }
}

fn animate(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut Transform,
        &mut WallObjectAnimated,
        &Handle<ColorMaterial>,
    )>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
) {
    for (entity, mut transform, mut state, mut material) in query.iter_mut() {
        if let Some(material) = materials.get_mut(material.id()) {
            state.current_color = Color::from(
                state
                    .current_color
                    .to_srgba()
                    .lerp(state.to_color.to_srgba(), time.delta_seconds() * 5.0),
            );
            state.current_scale = state.current_scale.lerp(1.0, time.delta_seconds() * 5.0);
            transform.scale = Vec3::splat(state.current_scale);
            *material = ColorMaterial::from_color(state.current_color);

            if (1.0 - state.current_scale).abs() < 0.000001 {
                transform.scale = Vec3::ONE;
                *material = ColorMaterial::from_color(state.to_color);
                commands.entity(entity).remove::<WallObjectAnimated>();
            }
        }
    }
}
