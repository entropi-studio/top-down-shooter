use crate::prelude::ToolkitDialogId;
use bevy::prelude::Event;

#[derive(Event)]
pub struct ToolkitDialogSelectOptionEvent {
    pub id: ToolkitDialogId,
    pub option: usize,
}
