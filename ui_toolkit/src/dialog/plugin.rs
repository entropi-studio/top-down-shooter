use crate::dialog::render::{insert_enter_animator, on_open_dialog, update_visibility};
use crate::dialog::ToolkitDialogGlobalState;
use bevy::app::{App, Plugin};
use bevy::prelude::Update;

pub(crate) struct ToolkitDialogPlugin;

impl Plugin for ToolkitDialogPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ToolkitDialogGlobalState::default())
            .add_systems(Update, (insert_enter_animator, update_visibility))
            .observe(on_open_dialog);
    }
}
