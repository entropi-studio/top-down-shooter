use crate::prelude::{ToolkitDialogId, ToolkitDialogOpenTrigger};
use bevy::prelude::Commands;
use std::time::Duration;

#[derive(Clone)]
pub struct ToolkitDialog {
    pub title: String,
    pub dialog_type: ToolkitDialogType,
    pub timeout: Option<Duration>,
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
    timeout: Option<Duration>,
}

impl ToolkitDialogBuilder {
    pub fn new() -> Self {
        Self {
            title: String::new(),
            dialog_type: ToolkitDialogType::Alert {
                description: String::new(),
            },
            timeout: None,
        }
    }

    pub fn build(self) -> ToolkitDialog {
        ToolkitDialog {
            title: self.title,
            dialog_type: self.dialog_type,
            timeout: self.timeout,
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

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn timeout_none(mut self) -> ToolkitDialogBuilder {
        self.timeout = None;
        self
    }
}

#[derive(Clone)]
pub enum ToolkitDialogType {
    Alert { description: String },
    Select {
        description: String,
        options: Vec<String>,
        dismissable: bool,
    },
}

impl ToolkitDialogType {
    pub fn alert_builder() -> ToolkitDialogTypeAlertBuilder {
        ToolkitDialogTypeAlertBuilder::new()
    }

    pub fn select_builder() -> ToolkitDialogTypeSelectBuilder {
        ToolkitDialogTypeSelectBuilder::new()
    }
}

pub struct ToolkitDialogTypeAlertBuilder {
    description: String,
}
impl ToolkitDialogTypeAlertBuilder {
    pub fn new() -> Self {
        Self {
            description: String::new(),
        }
    }

    pub fn build(self) -> ToolkitDialogType {
        ToolkitDialogType::Alert {
            description: self.description,
        }
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }
}

pub struct ToolkitDialogTypeSelectBuilder {
    description: String,
    options: Vec<String>,
    dismissable: bool,
}

impl ToolkitDialogTypeSelectBuilder {
    pub fn new() -> Self {
        Self {
            description: String::new(),
            options: vec![],
            dismissable: false,
        }
    }

    pub fn build(self) -> ToolkitDialogType {
        ToolkitDialogType::Select {
            description: self.description,
            options: self.options,
            dismissable: self.dismissable,
        }
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    pub fn dismissable(mut self) -> Self {
        self.dismissable = true;
        self
    }

    pub fn not_dismissable(mut self) -> ToolkitDialogTypeSelectBuilder {
        self.dismissable = false;
        self
    }

    pub fn option(mut self, text: impl Into<String>) -> ToolkitDialogTypeSelectBuilder {
        self.options.push(text.into());
        self
    }
}
