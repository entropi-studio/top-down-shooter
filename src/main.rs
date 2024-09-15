mod editor;
mod level;
mod objects;
mod screen;
mod state;

use crate::editor::ShootEditorPlugin;
use crate::objects::GameObjectPlugin;
use crate::screen::ScreenMainPlugin;
use crate::state::GameState;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy::DefaultPlugins;
use bevy_egui::EguiPlugin;
use bevy_light_2d::plugin::Light2dPlugin;
use rand::Rng;

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
        .add_plugins(Light2dPlugin)
        .add_plugins(ShootEditorPlugin)
        .add_plugins(GameObjectPlugin)
        .add_systems(Startup, setup_scene)
        .add_plugins(ScreenMainPlugin)
        .insert_state(GameState::Main)
        .run();
}

fn setup_scene(mut commands: Commands) {
    commands.spawn(Camera2dBundle { ..default() });
}
