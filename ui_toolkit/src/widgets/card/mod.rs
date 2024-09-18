use bevy::prelude::*;
use sickle_ui::prelude::*;

pub struct ToolkitCardWidgetPlugin;

impl Plugin for ToolkitCardWidgetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ComponentThemePlugin::<ToolkitCardWidget>::default());
    }
}

#[derive(Component, Debug, Default, Reflect, UiContext)]
#[reflect(Component)]
pub struct ToolkitCardWidget;

impl DefaultTheme for ToolkitCardWidget {
    fn default_theme() -> Option<Theme<ToolkitCardWidget>> {
        ToolkitCardWidget::theme().into()
    }
}

impl ToolkitCardWidget {
    pub fn theme() -> Theme<ToolkitCardWidget> {
        let base_theme = PseudoTheme::deferred(None, ToolkitCardWidget::primary_style);
        Theme::new(vec![base_theme])
    }

    fn primary_style(builder: &mut StyleBuilder, theme_data: &ThemeData) {
        let colors = theme_data.colors();

        builder
            .border_color(colors.surface_container_high)
            .background_color(colors.surface_container);
    }

    fn frame() -> impl Bundle {
        (
            Name::new("ToolkitCard"),
            ToolkitCardWidget,
            NodeBundle {
                style: Style {
                    padding: UiRect::axes(Val::Px(32.0), Val::Px(24.0)),
                    border: UiRect::all(Val::Px(2.0)),
                    overflow: Overflow::clip(),
                    height: Val::Auto,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                border_radius: BorderRadius::all(Val::Px(16.0)),
                ..default()
            },
        )
    }
}

pub trait UiToolkitCardWidgetExt {
    fn card(&mut self, spawn_children: impl FnOnce(&mut UiBuilder<Entity>)) -> UiBuilder<Entity>;
}

impl UiToolkitCardWidgetExt for UiBuilder<'_, Entity> {
    fn card(&mut self, spawn_children: impl FnOnce(&mut UiBuilder<Entity>)) -> UiBuilder<Entity> {
        self.container(ToolkitCardWidget::frame(), spawn_children)
    }
}
