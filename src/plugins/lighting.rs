use bevy::app::App;
use bevy::prelude::Plugin;
use lighting::prelude::Light2dPlugin;

pub struct LightingPlugin;

impl Plugin for LightingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Light2dPlugin);
    }
}
