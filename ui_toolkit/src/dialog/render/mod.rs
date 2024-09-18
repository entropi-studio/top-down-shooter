pub mod markers;
pub mod type_alert;
pub mod type_select;

use crate::dialog::render::type_alert::render_alert_type;
use crate::dialog::{ToolkitDialog, ToolkitDialogCloseTrigger, ToolkitDialogType};
use crate::prelude::{
    ToolkitDialogCloseAllTrigger, ToolkitDialogId, ToolkitDialogOpenTrigger, UiToolkitCardWidgetExt,
};
use crate::tween::lens::{UiBorderLens, UiSizeLens, ValSize};
use crate::ui::{StyleBuilderExt, TextBuilderExt, TextStyleExt};
use crate::widgets::UiToolkitButtonWidgetExt;
use bevy::color::palettes::basic::WHITE;
use bevy::color::{Color, Srgba};
use bevy::ecs::system::SystemId;
use bevy::prelude::*;
use bevy::text::TextStyle;
use bevy::ui::{Style, Val};
use bevy::utils::default;
use bevy_tweening::lens::{TransformScaleLens, UiBackgroundColorLens};
use bevy_tweening::{Animator, EaseFunction, Tracks, Tween};
use sickle_ui::prelude::*;
use std::convert::Into;
use std::time::Duration;
use crate::dialog::type_select::render_select_type;

#[derive(Resource, Default)]
pub(super) struct ToolkitDialogGlobalState {
    modal_despawn_system: Option<SystemId>,
}

#[derive(Component)]
pub struct ToolkitDialogInstance {
    pub id: ToolkitDialogId,
}

#[derive(Component)]
pub struct ToolkitDialogTimeout(pub Timer);

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
    let event = trigger.event();

    commands.ui_builder(UiRoot).column(|builder| {
        let modal_entity = builder.id();

        builder
            .style()
            .width(Val::Vw(100.0))
            .height(Val::Vh(100.0))
            .justify_content(JustifyContent::Center)
            .align_items(AlignItems::Center);

        builder.card(|builder| {
            let container_entity = builder.id();

            builder.insert(ToolkitDialogInstance { id: event.1 });
            if let Some(timeout) = event.0.timeout {
                builder.insert(ToolkitDialogTimeout(Timer::new(timeout, TimerMode::Once)));
            }

            builder
                .style()
                .visibility(Visibility::Hidden)
                .overflow(Overflow::clip())
                .padding(UiRect::ZERO);

            builder.column(|builder| {
                builder
                    .style()
                    .min_width(Val::Vw(35.0))
                    .height(Val::Auto)
                    .margin(UiRect::axes(Val::Px(32.0), Val::Px(24.0)))
                    .row_gap(Val::Px(8.0));

                match event.0.dialog_type {
                    ToolkitDialogType::Alert { .. } => {
                        render_alert_type(modal_entity, container_entity, event, builder);
                    }
                    ToolkitDialogType::Select { .. } => {
                        render_select_type(modal_entity, container_entity, event, builder);
                    }
                }
            });
        });
    });
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
    query: Query<(Entity, &Parent, &ToolkitDialogInstance)>,
    mut commands: Commands,
) {
    let ToolkitDialogCloseTrigger(target) = trigger.event();
    for (entity, parent, ToolkitDialogInstance { id, .. }) in query.iter() {
        if *target == *id {
            trigger_close(&mut commands, parent.get(), entity);
        }
    }
}

pub(super) fn on_close_all(
    trigger: Trigger<ToolkitDialogCloseAllTrigger>,
    query: Query<(Entity, &Parent, &ToolkitDialogInstance)>,
    mut commands: Commands,
) {
    for (entity, parent, ToolkitDialogInstance { id, .. }) in query.iter() {
        trigger_close(&mut commands, parent.get(), entity);
    }
}

pub(super) fn handle_timeout(
    mut query: Query<(Entity, &Parent, &mut ToolkitDialogTimeout)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, parent, mut timeout) in query.iter_mut() {
        timeout.0.tick(time.delta());
        if timeout.0.just_finished() {
            trigger_close(&mut commands, parent.get(), entity);
        }
    }
}

pub(super) fn insert_enter_animator(
    mut query: Query<
        (
            Entity,
            &Node,
            Option<&mut BackgroundColor>,
            &Style,
            &mut Visibility,
        ),
        (
            Without<Animator<BackgroundColor>>,
            With<ToolkitDialogInstance>,
        ),
    >,
    mut commands: Commands,
) {
    for (entity, node, mut background_color, mut style, mut visibility) in query.iter_mut() {
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

        let tween_style = Tracks::new([
            Tween::new(
                easing,
                duration,
                UiSizeLens {
                    start: ValSize::auto().with_width(Val::Px(0.0)),
                    end: ValSize::auto().with_width(Val::Px(node.size().x)),
                },
            ),
            Tween::new(
                easing,
                duration,
                UiBorderLens {
                    start: UiRect::ZERO,
                    end: style.border,
                },
            ),
        ]);
        let tween_color = Tween::new(
            easing,
            duration,
            UiBackgroundColorLens {
                start: background_initial.with_alpha(0.9),
                end: background_initial,
            },
        );
        let tween_scale = Tween::new(
            easing,
            duration,
            TransformScaleLens {
                start: Vec3::splat(0.6),
                end: Vec3::ONE,
            },
        );

        commands.entity(entity).insert((
            Animator::new(tween_style),
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
                end: background_initial.with_alpha(0.9),
            },
        );
        let tween_transform = Tween::new(
            easing,
            duration,
            TransformScaleLens {
                start: Vec3::ONE,
                end: Vec3::splat(0.6),
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
    mut query: Query<
        (&mut Visibility),
        (
            With<ToolkitDialogInstance>,
            Added<Animator<BackgroundColor>>,
        ),
    >,
) {
    for mut visibility in query.iter_mut() {
        *visibility = Visibility::Inherited;
    }
}
