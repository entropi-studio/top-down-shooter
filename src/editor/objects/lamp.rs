use crate::editor::{
    EditorObject, EditorObjectPosition, EditorObjectPositionSnap, EditorObjectSize,
    EditorObjectSizeSnap,
};
use crate::objects::LampObjectBundle;
use bevy::app::{App, Plugin, Update};
use bevy::color::palettes::basic::GREEN;
use bevy::input::mouse::MouseWheel;
use bevy::input::ButtonInput;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use lighting::prelude::{PointLight2d, PointLight2dBundle};
use std::f32::consts::{PI, TAU};

#[derive(Default, PartialEq)]
enum EditorLampAngleInputMode {
    #[default]
    Angle,
    Rotation,
}

#[derive(Component)]
pub struct EditorLamp {
    current_radius: f32,
    current_intensity: f32,
    current_angle: f32,
    target_angle: f32,
    current_rotation: f32,
    target_rotation: f32,
    angle_input_mode: EditorLampAngleInputMode,
}

impl Default for EditorLamp {
    fn default() -> Self {
        Self {
            current_radius: 0.0,
            current_intensity: 0.0,
            current_angle: 0.0,
            target_angle: TAU,
            current_rotation: 0.0,
            target_rotation: 0.0,
            angle_input_mode: EditorLampAngleInputMode::default(),
        }
    }
}

#[derive(Bundle)]
pub struct EditorLampBundle {
    pub position: EditorObjectPosition,
    pub position_snap: EditorObjectPositionSnap,
    pub size: EditorObjectSize,
    pub size_snap: EditorObjectSizeSnap,
    pub lamp: EditorLamp,
    pub editor_object: EditorObject,
}

impl Default for EditorLampBundle {
    fn default() -> Self {
        Self {
            position: EditorObjectPosition::default(),
            position_snap: EditorObjectPositionSnap(Vec2::new(20.0, 20.0)),
            size: EditorObjectSize::default(),
            size_snap: EditorObjectSizeSnap(Vec2::new(10.0, 1.0)),
            lamp: EditorLamp::default(),
            editor_object: EditorObject,
        }
    }
}

pub struct EditorLampPlugin;
impl Plugin for EditorLampPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, lamp_interface)
            .add_systems(Update, (lamp_init, lamp_update, lamp_handle_place));
    }
}
fn lamp_init(mut commands: Commands, mut query: Query<(Entity), (Added<EditorLamp>)>) {
    for (entity) in query.iter_mut() {
        commands.entity(entity).insert(
            (PointLight2dBundle {
                point_light: PointLight2d {
                    color: Color::WHITE,
                    cast_shadows: true,
                    angles: [0.0, 5.0],
                    ..default()
                },
                ..default()
            }),
        );
    }
}

fn lamp_interface(
    mut query: Query<(&mut EditorLamp, &EditorObjectPosition, &EditorObjectSize)>,
    mut contexts: EguiContexts,
) {
    for (mut lamp, EditorObjectPosition(position), EditorObjectSize(size)) in query.iter_mut() {
        egui::Window::new("[Placing] Lamp").show(contexts.ctx_mut(), |ui| {
            ui.label(format!(
                "Position ({:^10}, {:^10})",
                position.x.round(),
                position.y.round()
            ));
            ui.label(format!("Radius {:^10}", size.x));
            ui.label(format!("Intensity {:^10}", size.y));
            ui.label(format!(
                "Angle    {:^10}   {}",
                lamp.current_angle.to_degrees().round().to_string() + "°",
                if lamp.angle_input_mode == EditorLampAngleInputMode::Angle {
                    "<---"
                } else {
                    ""
                }
            ));
            ui.label(format!(
                "Rotation {:^10}   {}",
                lamp.current_rotation.to_degrees().round().to_string() + "°",
                if lamp.angle_input_mode == EditorLampAngleInputMode::Angle {
                    ""
                } else {
                    "<---"
                }
            ));
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
    mut input_wheel: EventReader<MouseWheel>,
    mut input_keyboard: Res<ButtonInput<KeyCode>>,
    mut contexts: EguiContexts,
    mut gizmos: Gizmos,
) {
    if let Some(ctx_mut) = contexts.try_ctx_mut() {
        if ctx_mut.wants_pointer_input() {
            return;
        }
    }

    for (
        mut lamp,
        mut transform,
        mut light,
        EditorObjectPosition(position),
        EditorObjectSize(size),
    ) in query.iter_mut()
    {
        transform.translation = transform.translation.lerp(
            Vec3::new(position.x, position.y, 0.0),
            time.delta_seconds() * 10.0,
        );
        gizmos.circle_2d(transform.translation.xy(), 10.0, GREEN);

        let mut angle_input = 0.0;
        if input_keyboard.pressed(KeyCode::AltLeft) {
            for event in input_wheel.read() {
                angle_input += event.y * (5.0f32.to_radians());
            }
            if input_keyboard.just_pressed(KeyCode::KeyG) {
                if lamp.angle_input_mode == EditorLampAngleInputMode::Angle {
                    lamp.angle_input_mode = EditorLampAngleInputMode::Rotation;
                } else {
                    lamp.angle_input_mode = EditorLampAngleInputMode::Angle;
                }
            }
        }

        if lamp.angle_input_mode == EditorLampAngleInputMode::Angle {
            lamp.target_angle += angle_input;
            lamp.target_angle = lamp.target_angle.clamp(0.0, PI);
        } else {
            lamp.target_rotation += angle_input;
            lamp.target_rotation = lamp.target_rotation.clamp(0.0, TAU);
        }

        let interpolation = time.delta_seconds() * 10.0;

        lamp.current_radius = lamp.current_radius.lerp(size.x, interpolation);
        lamp.current_intensity = lamp.current_intensity.lerp(size.y, interpolation);
        lamp.current_angle = lamp.current_angle.lerp(lamp.target_angle, interpolation);
        lamp.current_rotation = lamp
            .current_rotation
            .lerp(lamp.target_rotation, interpolation);

        light.angles = [lamp.current_rotation, lamp.current_angle];
        light.radius = lamp.current_radius;
        light.intensity = lamp.current_intensity;
    }
}

fn lamp_handle_place(
    mut commands: Commands,
    query: Query<(&EditorObjectPosition, &EditorObjectSize), With<EditorLamp>>,
    input_mouse: Res<ButtonInput<MouseButton>>,
    mut contexts: EguiContexts,
) {
    if let Some(ctx_mut) = contexts.try_ctx_mut() {
        if ctx_mut.wants_pointer_input() {
            return;
        }
    }
    if input_mouse.just_pressed(MouseButton::Left) {
        for (EditorObjectPosition(position), EditorObjectSize(size)) in query.iter() {
            commands.spawn(LampObjectBundle::new(*position, size.x, size.y));
        }
    }
}
