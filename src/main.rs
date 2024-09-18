mod editor;
mod level;
mod objects;
mod plugins;
mod render;
mod screen;
mod state;

use crate::editor::EditorPlugin;
use crate::objects::GameObjectPlugin;
use crate::plugins::{BasePlugin, LightingPlugin};
use crate::screen::ScreenMainPlugin;
use bevy::prelude::*;
use rand::Rng;
use std::panic::catch_unwind;

fn main() {
    App::new()
        .add_plugins(BasePlugin)
        .add_plugins(LightingPlugin)
        .add_plugins(EditorPlugin)
        .add_plugins(GameObjectPlugin)
        .add_plugins(ScreenMainPlugin)
        .run();
}
