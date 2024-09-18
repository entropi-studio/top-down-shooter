use crate::dialog::events::ToolkitDialogSelectOptionEvent;
use crate::dialog::render::markers::ToolkitDialogSelectOptionButton;
use crate::dialog::*;
use crate::prelude::*;
use bevy::color::palettes::basic::WHITE;
use bevy::prelude::*;
use sickle_ui::prelude::*;

pub(super) fn render_select_type(
    modal_entity: Entity,
    container_entity: Entity,
    trigger: &ToolkitDialogOpenTrigger,
    builder: &mut UiBuilder<Entity>,
) {
    let ToolkitDialogOpenTrigger(
        ToolkitDialog {
            title,
            dialog_type:
                ToolkitDialogType::Select {
                    description,
                    options,
                    dismissable,
                },
            ..
        },
        id,
    ) = trigger
    else {
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
        builder.style().row_gap(Val::Px(10.0));

        for (i, option) in options.iter().enumerate() {
            builder
                .toolkit_text_button(option.clone())
                .insert(ToolkitDialogSelectOptionButton(*id, i))
                .insert(ToolkitDialogDiscardButton {
                    modal: modal_entity,
                    container: container_entity,
                });
        }

        if *dismissable {
            builder
                .toolkit_text_button("Close")
                .insert(ToolkitDialogDiscardButton {
                    modal: modal_entity,
                    container: container_entity,
                });
        }
    });
}

pub(in super::super) fn handle_dialog_select_option(
    mut query: Query<(&ToolkitDialogSelectOptionButton, &FluxInteraction)>,
    mut commands: Commands,
) {
    for (ToolkitDialogSelectOptionButton(dialog_id, index), interaction) in query.iter() {
        if !interaction.is_released() {
            continue;
        }

        commands.trigger(ToolkitDialogSelectOptionEvent {
            id: *dialog_id,
            option: *index,
        });
    }
}
