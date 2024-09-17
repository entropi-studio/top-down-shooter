mod button;

use bevy::app::{App, Plugin};
pub use button::*;

pub struct ToolkitWidgetPlugin;

impl Plugin for ToolkitWidgetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ToolkitButtonWidgetPlugin);
    }
}
