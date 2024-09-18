use crate::dialog::types::ToolkitDialogType;
use crate::dialog::ToolkitDialog;
use bevy::prelude::Event;

#[derive(Copy, Clone, PartialEq)]
pub struct ToolkitDialogId(pub i32);

impl ToolkitDialogId {
    pub fn new() -> ToolkitDialogId {
        Self(rand::random())
    }
}

#[derive(Event, Clone)]
pub struct ToolkitDialogOpenTrigger(pub ToolkitDialog, pub ToolkitDialogId);
#[derive(Event, Clone)]
pub struct ToolkitDialogCloseTrigger(pub ToolkitDialogId);
#[derive(Event, Clone)]
pub struct ToolkitDialogCloseAllTrigger;

impl From<ToolkitDialog> for ToolkitDialogOpenTrigger {
    fn from(dialog: ToolkitDialog) -> Self {
        Self(dialog, ToolkitDialogId::new())
    }
}
