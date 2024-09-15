mod lamp;
mod wall;

pub use lamp::*;
pub use wall::*;

use crate::objects::lamp::LampObjectPlugin;
use bevy::app::App;
use bevy::prelude::{Component, Plugin};

pub struct GameObjectPlugin;

#[derive(Component)]
pub struct GameObject;

impl Plugin for GameObjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((WallObjectPlugin, LampObjectPlugin));
    }
}
