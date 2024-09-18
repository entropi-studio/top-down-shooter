use crate::screen::main::marker::ScreenMainComponent;
use crate::screen::main::state::ScreenMainState;
use crate::state::GameState;
use bevy::prelude::{
    AlignItems, Changed, Commands, Component, DespawnRecursiveExt, Entity, JustifyContent,
    NextState, Query, Res, ResMut, Trigger, With,
};
use bevy::ui::Val;
use sickle_ui::prelude::*;
use std::time::Duration;
use ui_toolkit::prelude::*;

#[derive(Component)]
pub(super) enum ButtonType {
    Play,
    Editor,
    Settings,
    Quit,
}

pub(super) fn on_enter(mut commands: Commands) {
    commands.ui_builder(UiRoot).column(|builder| {
        builder
            .style()
            .width(Val::Vw(100.0))
            .height(Val::Vh(100.0))
            .justify_content(JustifyContent::Center)
            .align_items(AlignItems::Center);

        builder.insert(ScreenMainComponent);

        builder.card(|builder| {
            builder
                .style()
                .height(Val::Auto)
                .min_width(Val::Percent(30.0))
                .row_gap(Val::Px(10.0));

            builder.column(|builder| {
                builder
                    .style()
                    .width(Val::Percent(100.0))
                    .row_gap(Val::Px(10.0));

                builder.toolkit_text_button("Play").insert(ButtonType::Play);
                builder
                    .toolkit_text_button("Editor")
                    .insert(ButtonType::Editor);

                builder.row(|builder| {
                    builder
                        .style()
                        .min_width(Val::Percent(100.0))
                        .column_gap(Val::Px(10.0));

                    builder
                        .toolkit_text_button("Settings")
                        .insert(ButtonType::Settings)
                        .style()
                        .flex_grow(1.0);
                    builder
                        .toolkit_text_button("Quit")
                        .insert(ButtonType::Quit)
                        .style()
                        .flex_grow(1.0);
                });
            });
        });
    });
}

pub(super) fn on_exit(mut commands: Commands, query: Query<Entity, With<ScreenMainComponent>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    commands.remove_resource::<ScreenMainState>();
}

pub(super) fn on_press(
    mut screen_state: ResMut<ScreenMainState>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    mut query: Query<(&ButtonType, &FluxInteraction), Changed<FluxInteraction>>,
) {
    for (button_type, interaction) in query.iter() {
        if *interaction != FluxInteraction::Released {
            continue;
        }

        match button_type {
            ButtonType::Play => {}
            ButtonType::Editor => {
                screen_state.editor_select_dialog_id = Some(
                    ToolkitDialogBuilder::new()
                        .title("Editor")
                        .dialog_type(
                            ToolkitDialogType::select_builder()
                                .option("Load existing project")
                                .option("Start new project")
                                .option("Cancel")
                                .build(),
                        )
                        // .timeout(Duration::from_secs(2))
                        .open(&mut commands),
                );
            }
            ButtonType::Settings => {}
            ButtonType::Quit => {}
        }
    }
}

pub(super) fn on_select_editor(
    trigger: Trigger<ToolkitDialogSelectOptionEvent>,
    screen_state: Option<Res<ScreenMainState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Some(screen_state) = screen_state else {
        return;
    };

    let Some(dialog_id) = screen_state.editor_select_dialog_id else {
        return;
    };

    let event = trigger.event();
    if event.id != dialog_id {
        return;
    }

    match event.option {
        0 => {}
        1 => next_state.set(GameState::Editor),
        _ => {}
    }
}
