use crate::dialog::ToolkitDialogPlugin;
use bevy::app::App;
use bevy::prelude::Plugin;
use bevy_tweening::TweeningPlugin;
use sickle_ui::SickleUiPlugin;
use crate::widgets::ToolkitWidgetPlugin;

pub struct UiToolkitPlugin;

impl Plugin for UiToolkitPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<TweeningPlugin>() {
            app.add_plugins(TweeningPlugin);
        }
        if !app.is_plugin_added::<SickleUiPlugin>() {
            app.add_plugins(SickleUiPlugin);
        }
        app.add_plugins(ToolkitWidgetPlugin);
        app.add_plugins(ToolkitDialogPlugin);
    }
}
