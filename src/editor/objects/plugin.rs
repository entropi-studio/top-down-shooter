use crate::editor::objects::wall::{wall_object_init, wall_object_update};
use bevy::prelude::*;

pub(in super::super) struct EditorObjectsPlugin;

impl Plugin for EditorObjectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (wall_object_init, wall_object_update));
    }
}
