use crate::ui::StyleBuilderExt;
use bevy::color::palettes::basic::GRAY;
use bevy::prelude::*;
use sickle_ui::ease::Ease;
use sickle_ui::prelude::*;

pub struct ToolkitButtonWidgetPlugin;

impl Plugin for ToolkitButtonWidgetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ComponentThemePlugin::<ToolkitButtonWidget>::default());
    }
}

#[derive(Component, Debug, Default, Reflect, UiContext)]
#[reflect(Component)]
pub struct ToolkitButtonWidget;

impl DefaultTheme for ToolkitButtonWidget {
    fn default_theme() -> Option<Theme<ToolkitButtonWidget>> {
        ToolkitButtonWidget::theme().into()
    }
}

impl ToolkitButtonWidget {
    pub fn theme() -> Theme<ToolkitButtonWidget> {
        let base_theme = PseudoTheme::deferred(None, Self::style);
        Theme::new(vec![base_theme])
    }

    fn style(style_builder: &mut StyleBuilder, theme_data: &ThemeData) {
        let colors = theme_data.colors();

        style_builder
            .justify_content(JustifyContent::Center)
            .align_items(AlignItems::Center)
            .padding(UiRect::all(Val::Px(8.0)))
            .border(UiRect::all(Val::Px(2.0)))
            .border_radius(BorderRadius::all(Val::Px(16.0)))
            .border_color(colors.on_surface);
        style_builder
            .animated()
            .scale(AnimatedVals {
                idle: 1.0,
                press: Some(0.95),
                ..default()
            })
            .copy_from(theme_data.interaction_animation)
            .pointer_enter(0.3, Ease::OutCubic, None)
            .pointer_leave(0.3, Ease::OutCubic, None)
            .non_interacted(0.3, Ease::OutCubic, None)
            .cancel(0.3, Ease::OutCubic, None)
            .cancel_reset(0.3, Ease::OutCubic, None)
            .press(0.3, Ease::OutCubic, None)
            .release(0.3, Ease::OutCubic, None);
        style_builder
            .animated()
            .background_color(AnimatedVals {
                idle: Color::WHITE.with_alpha(0.0),
                hover: Some(Color::WHITE.with_alpha(0.05)),
                press: Some(Color::WHITE.with_alpha(0.1)),
                ..default()
            })
            .copy_from(theme_data.interaction_animation)
            .pointer_enter(0.3, Ease::OutCubic, None)
            .pointer_leave(0.3, Ease::OutCubic, None)
            .non_interacted(0.3, Ease::OutCubic, None)
            .cancel(0.3, Ease::OutCubic, None)
            .cancel_reset(0.3, Ease::OutCubic, None)
            .press(0.3, Ease::OutCubic, None)
            .release(0.3, Ease::OutCubic, None);
    }
}

pub trait UiToolkitButtonWidgetExt {
    fn toolkit_button(
        &mut self,
        spawn_children: impl FnOnce(&mut UiBuilder<Entity>),
    ) -> UiBuilder<'_, Entity>;

    fn toolkit_text_button(&mut self, text: impl Into<TextBundle>) -> UiBuilder<'_, Entity> {
        self.toolkit_button(|builder| {
            builder.spawn(text.into());
        })
    }
}

impl UiToolkitButtonWidgetExt for UiBuilder<'_, Entity> {
    fn toolkit_button(
        &mut self,
        spawn_children: impl FnOnce(&mut UiBuilder<Entity>),
    ) -> UiBuilder<'_, Entity> {
        self.container(
            (
                ButtonBundle::default(),
                TrackedInteraction::default(),
                ToolkitButtonWidget,
            ),
            spawn_children,
        )
    }
}
