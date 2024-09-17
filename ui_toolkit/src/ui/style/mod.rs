use bevy::prelude::Entity;
use sickle_ui::prelude::{DynamicStyle, StyleBuilder, UiColumnExt};
use sickle_ui::ui_builder::UiBuilder;

pub trait StyleBuilderExt {
    fn apply_style(&mut self, build: impl FnOnce(&mut StyleBuilder)) -> &UiBuilder<Entity>;
}

impl StyleBuilderExt for UiBuilder<'_, Entity> {
    fn apply_style(
        &mut self,
        build: impl FnOnce(&mut StyleBuilder),
    ) -> &UiBuilder<'_, Entity> {
        let mut style_builder = StyleBuilder::new();
        build(&mut style_builder);
        self.insert(DynamicStyle::from(style_builder));
        self
    }
}
