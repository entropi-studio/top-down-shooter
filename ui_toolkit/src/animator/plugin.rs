use crate::animator::ToolkitAnimateEnterPlugin;
use bevy::app::App;
use bevy::prelude::Plugin;

pub struct ToolkitAnimatePlugin;

impl Plugin for ToolkitAnimatePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ToolkitAnimateEnterPlugin);
    }
}
