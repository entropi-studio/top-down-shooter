use crate::dialog::types::ToolkitDialogType;
use bevy::prelude::Event;
use crate::dialog::ToolkitDialog;

#[derive(Event)]
pub struct ToolkitOpenDialogTrigger(pub ToolkitDialog);

impl From<ToolkitDialog> for ToolkitOpenDialogTrigger {
    fn from(dialog: ToolkitDialog) -> Self {
        Self(dialog)
    }
}