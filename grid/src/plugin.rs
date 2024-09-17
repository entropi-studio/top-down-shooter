use bevy::prelude::{App, Plugin, PostUpdate, With};
use bevy::render::view::check_visibility;
use crate::options::InfiniteGridOptions;

pub struct InfiniteGridPlugin;

impl Plugin for InfiniteGridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, check_visibility::<With<InfiniteGridOptions>>);
    }

    fn finish(&self, app: &mut App) {
        render_app_builder(app);
    }
}