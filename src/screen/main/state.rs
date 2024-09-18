use bevy::prelude::Resource;
use ui_toolkit::dialog::ToolkitDialogId;

#[derive(Resource, Default)]
pub(super) struct ScreenMainState {
    pub editor_select_dialog_id: Option<ToolkitDialogId>,
}
