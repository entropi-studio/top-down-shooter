mod lamp;
mod wall;

pub use wall::*;
pub use lamp::*;

use crate::objects::lamp::LampObjectPlugin;
use bevy::app::App;
use bevy::prelude::Plugin;

pub struct GameObjectPlugin;

impl Plugin for GameObjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((WallObjectPlugin, LampObjectPlugin));
    }
}
