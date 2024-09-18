use crate::dialog::render::*;
use bevy::app::{App, Plugin};
use bevy::prelude::Update;

pub(crate) struct ToolkitDialogPlugin;

impl Plugin for ToolkitDialogPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ToolkitDialogGlobalState::default())
            .add_systems(
                Update,
                (
                    insert_enter_animator,
                    insert_exit_animator,
                    update_visibility,
                    cleanup_dialog,
                    handle_discard,
                ),
            )
            .observe(on_open_dialog)
            .observe(on_close)
            .observe(on_close_all);
    }
}
