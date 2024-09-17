use crate::dialog::{ToolkitDialog, ToolkitDialogType};
use crate::prelude::ToolkitOpenDialogTrigger;
use crate::tween::lens::{UiSizeLens, ValSize};
use crate::ui::{StyleBuilderExt, TextBuilderExt};
use crate::widgets::UiToolkitButtonWidgetExt;
use bevy::color::palettes::basic::WHITE;
use bevy::color::{Color, Srgba};
use bevy::ecs::system::SystemId;
use bevy::prelude::PositionType::Absolute;
use bevy::prelude::*;
use bevy::text::TextStyle;
use bevy::ui::{Style, Val};
use bevy::utils::default;
use bevy_tweening::lens::{TransformScaleLens, UiBackgroundColorLens};
use bevy_tweening::{Animator, EaseFunction, Tween};
use sickle_ui::prelude::*;
use std::convert::Into;
use std::time::Duration;

#[derive(Resource, Default)]
pub(super) struct ToolkitDialogGlobalState {
    modal_despawn_system: Option<SystemId>,
}

#[derive(Component)]
pub struct ToolkitDialogComponent;

#[derive(Component)]
pub(super) struct ToolkitDialogDiscardButton {
    modal: Entity,
    container: Entity,
}

const DIALOG_BACKGROUND: Srgba = Srgba::rgb(28.0 / 255.0, 28.0 / 255.0, 30.0 / 255.0);
const DIALOG_BORDER: Srgba = Srgba::rgb(40.0 / 255.0, 40.0 / 255.0, 41.0 / 255.0);
const DIALOG_TEXT: Srgba = WHITE;

fn in_modal(mut commands: Commands, content: impl FnOnce(&mut ChildBuilder)) {
    commands
        .spawn((NodeBundle {
            style: Style {
                width: Val::Vw(100.0),
                height: Val::Vh(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        },))
        .with_children(content);
}

pub(super) fn on_open_dialog(trigger: Trigger<ToolkitOpenDialogTrigger>, mut commands: Commands) {
    let ToolkitOpenDialogTrigger(ToolkitDialog { title, dialog_type }) = trigger.event();

    match dialog_type {
        ToolkitDialogType::Alert { content } => {
            commands.ui_builder(UiRoot).column(|builder| {
                let modal_entity = builder.id();

                builder
                    .style()
                    .width(Val::Vw(100.0))
                    .height(Val::Vh(100.0))
                    .justify_content(JustifyContent::Center)
                    .align_items(AlignItems::Center);

                builder.column(|builder| {
                    let container_entity = builder.id();

                    builder.insert(ToolkitDialogComponent);

                    builder
                        .style()
                        .height(Val::Auto)
                        // .min_width(Val::ZERO)
                        .border(UiRect::all(Val::Px(2.0)))
                        .border_color(DIALOG_BORDER.into())
                        .background_color(DIALOG_BACKGROUND.into())
                        .visibility(Visibility::Hidden)
                        .justify_content(JustifyContent::Center)
                        .align_items(AlignItems::Center)
                        .row_gap(Val::Px(8.0))
                        .position_type(Absolute)
                        .overflow(Overflow::clip());

                    builder.column(|builder| {
                        builder.style().margin(UiRect::all(Val::Px(16.0)));

                        builder.spawn(TextBundle::from_section(
                            title,
                            TextStyle {
                                color: DIALOG_TEXT.into(),
                                font_size: 24.0,
                                ..default()
                            },
                        ));
                        builder.spawn(TextBundle::from_section(
                            content,
                            TextStyle {
                                color: DIALOG_TEXT.into(),
                                font_size: 16.0,
                                ..default()
                            },
                        ));
                        builder
                            .spawn((
                                ButtonBundle::default(),
                                ToolkitDialogDiscardButton {
                                    modal: modal_entity,
                                    container: container_entity,
                                },
                            ))
                            .column(|builder| {
                                builder.toolkit_button(|builder| {
                                    builder.text("Confirm", |style| {
                                        style.font_size(8.0);
                                    });
                                });
                            });
                    });
                });
            });
        }
    };
}

pub(super) fn handle_discard(
    mut query: Query<
        (
            Entity,
            &ToolkitDialogDiscardButton,
            &Node,
            Option<&BackgroundColor>,
            &Interaction,
        ),
        (Changed<Interaction>),
    >,
    mut commands: Commands,
    mut global_state: ResMut<ToolkitDialogGlobalState>,
) {
    if global_state.modal_despawn_system == None {
        global_state.modal_despawn_system = Some(commands.register_one_shot_system(modal_despawn));
    }

    for (entity, button, node, color, interaction) in query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        let base_color = if let Some(color) = color {
            color.0
        } else {
            Color::default()
        };

        let easing = EaseFunction::CubicInOut;
        let duration = Duration::from_secs_f32(1.0);

        let tween_size = Tween::new(
            easing,
            duration,
            UiSizeLens {
                start: ValSize::auto().with_width(Val::Px(node.size().x)),
                end: ValSize::auto().with_width(Val::Px(0.0)),
            },
        );
        let tween_color = Tween::new(
            easing,
            duration,
            UiBackgroundColorLens {
                start: base_color,
                end: base_color.with_alpha(0.0),
            },
        )
        .with_completed_system(global_state.modal_despawn_system.unwrap());

        commands
            .entity(button.container)
            .insert((Animator::new(tween_size), Animator::new(tween_color)));
    }
}

fn modal_despawn() {}

pub(super) fn insert_enter_animator(
    mut query: Query<
        (
            Entity,
            &Node,
            Option<&mut BackgroundColor>,
            Option<&mut BorderColor>,
            &mut Visibility,
        ),
        (
            Without<Animator<BackgroundColor>>,
            With<ToolkitDialogComponent>,
        ),
    >,
    mut commands: Commands,
) {
    for (entity, node, mut background_color, mut border_color, mut visibility) in query.iter_mut() {
        if node.size().x == 0.0 && node.size().y == 0.0 {
            *visibility = Visibility::Hidden;
            continue;
        }

        let background_initial = if let Some(mut color) = background_color {
            let cache = color.0.clone();
            *color = BackgroundColor(cache.with_alpha(0.0));
            cache
        } else {
            Color::default()
        };

        // let border_initial = if let Some(mut color) = border_color {
        //     let cache = color.0.clone();
        //     *color = BorderColor(cache.with_alpha(0.0));
        //     cache
        // } else {
        //     Color::default()
        // };

        let easing = EaseFunction::BackOut;
        let duration = Duration::from_secs_f32(1.0);

        let tween_size = Tween::new(
            easing,
            duration,
            UiSizeLens {
                start: ValSize::auto().with_width(Val::Px(0.0)),
                end: ValSize::auto().with_width(Val::Px(node.size().x)),
            },
        );
        let tween_color = Tween::new(
            easing,
            duration,
            UiBackgroundColorLens {
                start: background_initial.with_alpha(0.0),
                end: background_initial,
            },
        );
        let tween_scale = Tween::new(
            easing,
            duration,
            TransformScaleLens {
                start: Vec3::splat(0.8),
                end: Vec3::ONE,
            },
        );

        commands.entity(entity).insert((
            Animator::new(tween_size),
            Animator::new(tween_color),
            Animator::new(tween_scale),
        ));
    }
}

pub(super) fn update_visibility(
    mut query: Query<(&mut Visibility), (With<ToolkitDialogComponent>, Added<Animator<Style>>)>,
) {
    for mut visibility in query.iter_mut() {
        *visibility = Visibility::Inherited;
    }
}
