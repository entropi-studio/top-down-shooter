use bevy::prelude::{Bundle, Component, TransformBundle, VisibilityBundle};

#[derive(Component, Default)]
pub struct Player;

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    player: Player,
    transform: TransformBundle,
    visibility: VisibilityBundle,
}

#[derive(Component)]
pub struct PlayerShieldShard;
