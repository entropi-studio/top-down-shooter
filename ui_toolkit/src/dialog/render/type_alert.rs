use crate::dialog::*;
use crate::prelude::*;
use bevy::color::palettes::basic::WHITE;
use bevy::prelude::*;
use sickle_ui::prelude::*;

pub(super) fn render_alert_type(
    modal_entity: Entity,
    container_entity: Entity,
    trigger: &ToolkitDialogOpenTrigger,
    builder: &mut UiBuilder<Entity>,
) {
    let ToolkitDialogOpenTrigger(
        ToolkitDialog {
            title,
            dialog_type: ToolkitDialogType::Alert { description },
            timeout,
        },
        id,
    ) = trigger else {
        return;
    };

    builder.spawn(TextBundle::from_section(
        title,
        TextStyle {
            color: WHITE.into(),
            font_size: 36.0,
            ..default()
        },
    ));
    builder.spawn(TextBundle::from_section(
        description,
        TextStyle {
            color: WHITE.into(),
            font_size: 24.0,
            ..default()
        },
    ));

    builder.column(|builder| {
        builder
            .toolkit_text_button("Close")
            .insert(ToolkitDialogDiscardButton {
                modal: modal_entity,
                container: container_entity,
            });
    });
}
