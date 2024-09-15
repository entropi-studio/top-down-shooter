use crate::editor::{EditorLampPlugin, EditorWallPlugin};
use bevy::prelude::*;

pub(in super::super) struct EditorObjectsPlugin;

impl Plugin for EditorObjectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((EditorWallPlugin, EditorLampPlugin));
    }
}
