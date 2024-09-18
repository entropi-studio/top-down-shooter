use crate::dialog::{ToolkitDialog, ToolkitDialogCloseTrigger, ToolkitDialogType};
use crate::prelude::{ToolkitDialogCloseAllTrigger, ToolkitDialogId, ToolkitDialogOpenTrigger};
use crate::tween::lens::{UiBorderLens, UiSizeLens, ValSize};
use crate::ui::{StyleBuilderExt, TextBuilderExt, TextStyleExt};
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
use bevy_tweening::{Animator, EaseFunction, Tracks, Tween};
use sickle_ui::prelude::*;
use std::convert::Into;
use std::time::Duration;

#[derive(Resource, Default)]
pub(super) struct ToolkitDialogGlobalState {
    modal_despawn_system: Option<SystemId>,
}

#[derive(Component)]
pub struct ToolkitDialogComponent(pub ToolkitDialogId);

#[derive(Component)]
pub(super) struct ToolkitDialogClosing {
    modal: Entity,
    container: Entity,
}

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

pub(super) fn on_open_dialog(trigger: Trigger<ToolkitDialogOpenTrigger>, mut commands: Commands) {
    let ToolkitDialogOpenTrigger(ToolkitDialog { title, dialog_type }, id) = trigger.event();

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

                    builder.insert(ToolkitDialogComponent(*id));

                    builder
                        .style()
                        .height(Val::Auto)
                        .border(UiRect::all(Val::Px(2.0)))
                        .border_color(DIALOG_BORDER.into())
                        .border_radius(BorderRadius::all(Val::Px(16.0)))
                        .background_color(DIALOG_BACKGROUND.into())
                        .visibility(Visibility::Hidden)
                        .justify_content(JustifyContent::Center)
                        .align_items(AlignItems::Center)
                        .position_type(Absolute)
                        .overflow(Overflow::clip());

                    builder.column(|builder| {
                        builder
                            .style()
                            .min_width(Val::Px(200.0))
                            .margin(UiRect::axes(Val::Px(32.0), Val::Px(24.0)))
                            .row_gap(Val::Px(8.0));

                        builder.spawn(TextBundle::from_section(
                            title,
                            TextStyle {
                                color: DIALOG_TEXT.into(),
                                font_size: 36.0,
                                ..default()
                            },
                        ));
                        builder.spawn(TextBundle::from_section(
                            content,
                            TextStyle {
                                color: DIALOG_TEXT.into(),
                                font_size: 24.0,
                                ..default()
                            },
                        ));

                        builder.column(|builder| {
                            builder.toolkit_text_button("Close").insert(
                                ToolkitDialogDiscardButton {
                                    modal: modal_entity,
                                    container: container_entity,
                                },
                            );
                        });
                    });
                });
            });
        }
    };
}

fn trigger_close(commands: &mut Commands, modal: Entity, container: Entity) {
    commands
        .entity(container)
        .insert(ToolkitDialogClosing { modal, container });
}

pub(super) fn handle_discard(
    mut query: Query<
        (Entity, &ToolkitDialogDiscardButton, &FluxInteraction),
        (Without<ToolkitDialogClosing>),
    >,
    mut commands: Commands,
) {
    for (entity, button, interaction) in query.iter_mut() {
        if interaction.is_released() {
            commands
                .entity(entity)
                .remove::<ToolkitDialogDiscardButton>();
            trigger_close(&mut commands, button.modal, button.container);
        }
    }
}

pub(super) fn on_close(
    trigger: Trigger<ToolkitDialogCloseTrigger>,
    query: Query<(Entity, &Parent, &ToolkitDialogComponent)>,
    mut commands: Commands,
) {
    let ToolkitDialogCloseTrigger(target) = trigger.event();
    for (entity, parent, ToolkitDialogComponent(id)) in query.iter() {
        if *target == *id {
            trigger_close(&mut commands, parent.get(), entity);
        }
    }
}

pub(super) fn on_close_all(
    trigger: Trigger<ToolkitDialogCloseAllTrigger>,
    query: Query<(Entity, &Parent, &ToolkitDialogComponent)>,
    mut commands: Commands,
) {
    for (entity, parent, ToolkitDialogComponent(id)) in query.iter() {
        trigger_close(&mut commands, parent.get(), entity);
    }
}

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

        let easing = EaseFunction::QuadraticOut;
        let duration = Duration::from_secs_f32(0.5);

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

pub(super) fn insert_exit_animator(
    mut query: Query<
        (
            Entity,
            &Node,
            Option<&mut BackgroundColor>,
            &mut Style,
            &mut Visibility,
        ),
        (Added<ToolkitDialogClosing>,),
    >,
    mut commands: Commands,
) {
    for (entity, node, mut background_color, mut style, mut visibility) in query.iter_mut() {
        let background_initial = if let Some(mut color) = background_color {
            color.0.clone()
        } else {
            Color::default()
        };

        let easing = EaseFunction::QuadraticIn;
        let duration = Duration::from_secs_f32(0.5);

        let tween_style = Tracks::new([
            Tween::new(
                easing,
                duration,
                UiSizeLens {
                    start: ValSize::auto().with_width(Val::Px(node.size().x)),
                    end: ValSize::auto().with_width(Val::Px(0.0)),
                },
            ),
            Tween::new(
                easing,
                duration,
                UiBorderLens {
                    start: style.border,
                    end: UiRect::ZERO,
                },
            ),
        ]);
        let tween_color = Tween::new(
            easing,
            duration,
            UiBackgroundColorLens {
                start: background_initial,
                end: background_initial.with_alpha(0.5),
            },
        );
        let tween_transform = Tween::new(
            easing,
            duration,
            TransformScaleLens {
                start: Vec3::ONE,
                end: Vec3::splat(0.8),
            },
        );

        commands.entity(entity).insert((
            Animator::new(tween_style),
            Animator::new(tween_color),
            Animator::new(tween_transform),
        ));
    }
}

pub(super) fn cleanup_dialog(
    mut removed: RemovedComponents<Animator<BackgroundColor>>,
    query: Query<(&ToolkitDialogClosing)>,
    mut commands: Commands,
) {
    for removed in removed.read() {
        if let Ok(target) = query.get(removed) {
            commands.entity(target.container).despawn();
            commands.entity(target.modal).despawn();
        }
    }
}

pub(super) fn update_visibility(
    mut query: Query<(&mut Visibility), (With<ToolkitDialogComponent>, Added<Animator<Style>>)>,
) {
    for mut visibility in query.iter_mut() {
        *visibility = Visibility::Inherited;
    }
}
