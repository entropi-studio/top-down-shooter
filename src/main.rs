mod editor;
mod level;
mod objects;
mod plugins;
mod screen;
mod state;
mod render;

use crate::editor::EditorPlugin;
use crate::objects::GameObjectPlugin;
use crate::plugins::{BasePlugin, LightingPlugin};
use crate::screen::ScreenMainPlugin;
use bevy::prelude::*;
use rand::Rng;

fn main() {
    App::new()
        .add_plugins(BasePlugin)
        .add_plugins(LightingPlugin)
        .add_plugins(EditorPlugin)
        .add_plugins(GameObjectPlugin)
        .add_plugins(ScreenMainPlugin)
        .run();
}
