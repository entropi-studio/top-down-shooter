mod button;
mod card;

pub use button::*;
pub use card::*;

use bevy::app::{App, Plugin};

pub struct ToolkitWidgetPlugin;

impl Plugin for ToolkitWidgetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ToolkitButtonWidgetPlugin, ToolkitCardWidgetPlugin));
    }
}
