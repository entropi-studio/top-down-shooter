use crate::prelude::{ToolkitDialogId, ToolkitDialogOpenTrigger};
use bevy::prelude::Commands;
use bevy::reflect::erased_serde::__private::serde::de::Unexpected::Str;

#[derive(Clone)]
pub struct ToolkitDialog {
    pub title: String,
    pub dialog_type: ToolkitDialogType,
}

impl ToolkitDialog {
    pub fn to_trigger(self) -> ToolkitDialogOpenTrigger {
        ToolkitDialogOpenTrigger(self, ToolkitDialogId::new())
    }

    pub fn open(self, commands: &mut Commands) -> ToolkitDialogId {
        let trigger = self.to_trigger();
        commands.trigger(trigger.clone());
        trigger.1
    }
}

pub struct ToolkitDialogBuilder {
    title: String,
    dialog_type: ToolkitDialogType,
}

impl ToolkitDialogBuilder {
    pub fn new() -> Self {
        Self {
            title: String::new(),
            dialog_type: ToolkitDialogType::Alert {
                content: String::new(),
            },
        }
    }

    pub fn build(self) -> ToolkitDialog {
        ToolkitDialog {
            title: self.title,
            dialog_type: self.dialog_type,
        }
    }

    pub fn open(self, commands: &mut Commands) -> ToolkitDialogId {
        self.build().open(commands)
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn dialog_type(mut self, dialog_type: ToolkitDialogType) -> Self {
        self.dialog_type = dialog_type;
        self
    }
}

#[derive(Clone)]
pub enum ToolkitDialogType {
    Alert { content: String },
}

impl ToolkitDialogType {
    pub fn alert_builder() -> ToolkitDialogTypeAlertBuilder {
        ToolkitDialogTypeAlertBuilder::new()
    }
}

pub struct ToolkitDialogTypeAlertBuilder {
    content: String,
}

impl ToolkitDialogTypeAlertBuilder {
    pub fn new() -> Self {
        Self {
            content: String::new(),
        }
    }

    pub fn build(self) -> ToolkitDialogType {
        ToolkitDialogType::Alert {
            content: self.content,
        }
    }

    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = content.into();
        self
    }
}
