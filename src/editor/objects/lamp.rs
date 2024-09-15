use crate::editor::{
    EditorObject, EditorObjectPosition, EditorObjectPositionSnap, EditorObjectRotation,
    EditorObjectSize, EditorWall,
};
use crate::objects::{LampObjectBundle, WallObject, WallObjectBundle};
use bevy::app::{App, Plugin, Update};
use bevy::asset::Assets;
use bevy::color::Color;
use bevy::input::ButtonInput;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{
    default, Added, Bundle, ColorMaterial, Commands, Component, Entity, FloatExt, Mesh,
    MouseButton, Query, Rectangle, Res, ResMut, Time, Transform, With,
};
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy_egui::{egui, EguiContexts};
use bevy_light_2d::prelude::{PointLight2d, PointLight2dBundle};

#[derive(Component, Default)]
pub struct EditorLamp {
    current_radius: f32,
    current_intensity: f32,
}

#[derive(Bundle, Default)]
pub struct EditorLampBundle {
    pub position: EditorObjectPosition,
    pub position_snap: EditorObjectPositionSnap,
    pub size: EditorObjectSize,
    pub lamp: EditorLamp,
    pub editor_object: EditorObject,
}

pub struct EditorLampPlugin;
impl Plugin for EditorLampPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (lamp_init, lamp_update, lamp_handle_place));
    }
}
fn lamp_init(mut commands: Commands, mut query: Query<(Entity), (Added<EditorLamp>)>) {
    for (entity) in query.iter_mut() {
        commands.entity(entity).insert(PointLight2dBundle {
            point_light: PointLight2d {
                radius: 100.0,
                intensity: 3.0,
                cast_shadows: true,
                ..default()
            },
            ..default()
        });
    }
}

fn lamp_update(
    mut query: Query<(
        &mut EditorLamp,
        &mut Transform,
        &mut PointLight2d,
        &EditorObjectPosition,
        &EditorObjectSize,
    )>,
    time: Res<Time>,
    mut contexts: EguiContexts,
) {
    for (
        mut wall,
        mut transform,
        mut light,
        EditorObjectPosition(position),
        EditorObjectSize(size),
    ) in query.iter_mut()
    {
        egui::Window::new("[Placing] Lamp").show(contexts.ctx_mut(), |ui| {
            ui.label(format!(
                "Position ({:^10}, {:^10})",
                position.x.round(),
                position.y.round()
            ));
            ui.label(format!("Radius {:^10}", size.x));
            ui.label(format!("Intensity {:^10}", size.y));
        });

        transform.translation = transform.translation.lerp(
            Vec3::new(position.x, position.y, 0.0),
            time.delta_seconds() * 10.0,
        );

        wall.current_radius = wall
            .current_radius
            .lerp(size.x, time.delta_seconds() * 10.0);
        wall.current_intensity = wall
            .current_intensity
            .lerp(size.y, time.delta_seconds() * 10.0);

        light.radius = wall.current_radius;
        light.intensity = wall.current_intensity;
    }
}

fn lamp_handle_place(
    mut commands: Commands,
    query: Query<(&EditorObjectPosition, &EditorObjectSize), With<EditorLamp>>,
    input_mouse: Res<ButtonInput<MouseButton>>,
) {
    if input_mouse.just_pressed(MouseButton::Left) {
        for (EditorObjectPosition(position), EditorObjectSize(size)) in query.iter() {
            commands.spawn(LampObjectBundle::new(*position, size.x, size.y));
        }
    }
}
