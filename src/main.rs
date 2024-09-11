mod player;

use bevy::app::Startup;
use bevy::asset::Assets;
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy::DefaultPlugins;
use bevy_egui::EguiPlugin;
use rand::Rng;

#[derive(Component)]
struct PlayerCamera;

#[derive(Default, Resource)]
struct PlayerState {
    zoom_target: f32,
    zoom: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::Immediate,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin)
        .add_plugins((
            FrameTimeDiagnosticsPlugin::default(),
            LogDiagnosticsPlugin::default(),
        ))
        .insert_resource(PlayerState::default())
        .add_systems(Startup, (setup_scene, player::spawn_player))
        .add_systems(Update, (update_camera))
        .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                // hdr: true, // 1. HDR is required for bloom
                ..default()
            },
            // tonemapping: Tonemapping::TonyMcMapface, // 2. Using a tonemapper that desaturates to white is recommended
            ..default()
        },
        // BloomSettings::default(), // 3. Enable bloom for the camera,
        PlayerCamera,
    ));
}

fn update_camera(
    mut state: ResMut<PlayerState>,
    mut query: Query<(&mut OrthographicProjection), With<PlayerCamera>>,
    mut mouse_scroll: EventReader<MouseWheel>,
    time: Res<Time>,
) {
    for e in mouse_scroll.read() {
        state.zoom_target -= e.y * 1.0;
    }

    state.zoom_target = state.zoom_target.clamp(2.0, 100.0);

    state.zoom = state
        .zoom
        .lerp(state.zoom_target, time.delta_seconds() * 10.0);

    for mut projection in query.iter_mut() {
        projection.scale = state.zoom;
    }
}
