use crate::editor::{EditorLampPlugin, EditorWallPlugin};
use bevy::prelude::*;
use bevy_infinite_grid::InfiniteGridPlugin;

pub(in super::super) struct EditorObjectsPlugin;

impl Plugin for EditorObjectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InfiniteGridPlugin)
            .add_plugins((EditorWallPlugin, EditorLampPlugin));
    }
}
