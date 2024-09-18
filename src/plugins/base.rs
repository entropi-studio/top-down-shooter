use crate::render::MainCamera;
use crate::state::GameState;
use bevy::app::App;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy::DefaultPlugins;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use ui_toolkit::plugin::UiToolkitPlugin;

pub struct BasePlugin;

/// Setup base plugins and initialize camera, state
impl Plugin for BasePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::Immediate,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin)
        .add_plugins(UiToolkitPlugin)
        .add_plugins(WorldInspectorPlugin::new())
        // .add_plugins((
        //     FrameTimeDiagnosticsPlugin::default(),
        //     LogDiagnosticsPlugin::default(),
        // ))
        .insert_state(GameState::Main)
        // .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, init_camera);
    }
}

fn init_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera, IsDefaultUiCamera));
}
