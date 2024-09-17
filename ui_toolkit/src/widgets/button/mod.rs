use crate::ui::StyleBuilderExt;
use bevy::prelude::*;
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
            .border_radius(BorderRadius::all(Val::Px(20.0)))
            .border_color(colors.on_surface);
    }
}

pub trait UiToolkitButtonWidgetExt {
    fn toolkit_button(
        &mut self,
        spawn_children: impl FnOnce(&mut UiBuilder<Entity>),
    ) -> UiBuilder<'_, Entity>;
}

impl UiToolkitButtonWidgetExt for UiBuilder<'_, Entity> {
    fn toolkit_button(
        &mut self,
        spawn_children: impl FnOnce(&mut UiBuilder<Entity>),
    ) -> UiBuilder<'_, Entity> {
        self.container((ButtonBundle::default(), ToolkitButtonWidget), spawn_children)
    }
}
