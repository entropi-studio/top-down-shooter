use bevy::{
    prelude::*,
    render::{render_resource::ShaderType, Extract},
};

use crate::{
    light::{AmbientLight2d, PointLight2d},
    occluder::{LightOccluder2d, LightOccluder2dShape},
};

#[derive(Component, Default, Clone, ShaderType)]
pub struct ExtractedPointLight2d {
    pub transform: Vec2,
    pub radius: f32,
    pub color: LinearRgba,
    pub intensity: f32,
    pub falloff: f32,
    pub angles: [f32; 2],
    pub cast_shadows: u32,
}

#[derive(Component, Default, Clone, ShaderType)]
pub struct ExtractedLightOccluder2d {
    pub center: Vec2,
    pub rotation: Vec4,
    pub half_size: Vec2,
}

#[derive(Component, Default, Clone, ShaderType)]
pub struct ExtractedAmbientLight2d {
    pub color: LinearRgba,
}

pub fn extract_point_lights(
    mut commands: Commands,
    point_light_query: Extract<Query<(Entity, &PointLight2d, &GlobalTransform, &ViewVisibility)>>,
) {
    for (entity, point_light, global_transform, view_visibility) in &point_light_query {
        if !view_visibility.get() {
            continue;
        }
        commands.get_or_spawn(entity).insert(ExtractedPointLight2d {
            color: point_light.color.to_linear(),
            transform: global_transform.translation().xy(),
            radius: point_light.radius,
            intensity: point_light.intensity,
            falloff: point_light.falloff,
            angles: point_light.angles,
            cast_shadows: if point_light.cast_shadows { 1 } else { 0 },
        });
    }
}

pub fn extract_light_occluders(
    mut commands: Commands,
    light_occluders_query: Extract<
        Query<(
            Entity,
            &LightOccluder2d,
            &GlobalTransform,
            &Transform,
            &ViewVisibility,
        )>,
    >,
) {
    for (entity, light_occluder, global_transform, transform, view_visibility) in
        &light_occluders_query
    {
        if !view_visibility.get() {
            continue;
        }

        let extracted_occluder = match light_occluder.shape {
            LightOccluder2dShape::Rectangle { half_size } => ExtractedLightOccluder2d {
                half_size,
                rotation: transform.rotation.inverse().into(),
                center: global_transform.translation().xy(),
            },
        };

        commands.get_or_spawn(entity).insert(extracted_occluder);
    }
}

pub fn extract_ambient_lights(
    mut commands: Commands,
    ambient_light_query: Extract<Query<(Entity, &AmbientLight2d)>>,
    camera_query: Extract<Query<Entity, (With<Camera2d>, Without<AmbientLight2d>)>>,
) {
    for (entity, ambient_light) in &ambient_light_query {
        commands
            .get_or_spawn(entity)
            .insert(ExtractedAmbientLight2d {
                color: ambient_light.color.to_linear() * ambient_light.brightness,
            });
    }

    // Our lighting pass only runs on views with an ambient light component,
    // so let's add a no-op ambient light to any 2d cameras don't have one.
    for entity in &camera_query {
        commands
            .get_or_spawn(entity)
            .insert(ExtractedAmbientLight2d {
                color: Color::WHITE.into(),
            });
    }
}
