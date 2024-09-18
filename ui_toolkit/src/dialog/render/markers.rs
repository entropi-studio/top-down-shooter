use bevy::prelude::Component;
use crate::dialog::ToolkitDialogId;

#[derive(Component)]
pub(in super::super) struct ToolkitDialogSelectOptionButton(pub ToolkitDialogId, pub usize);