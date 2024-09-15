use bevy::math::{Quat, Vec2};
use bevy::prelude::Component;

#[derive(Component, Default)]
pub struct EditorObject;

#[derive(Component, Default)]
pub struct EditorObjectPosition(pub Vec2);

#[derive(Component, Default)]
pub struct EditorObjectPositionSnap(pub Vec2);

#[derive(Component)]
pub struct EditorObjectSize(pub Vec2);

impl Default for EditorObjectSize {
    fn default() -> Self {
        Self(Vec2::ONE)
    }
}

#[derive(Component, Default)]
pub struct EditorObjectSizeSnap(pub Vec2);

#[derive(Component)]
pub struct EditorObjectSizeRange {
    pub min: Vec2,
    pub max: Vec2,
}

impl EditorObjectSizeRange {
    pub fn new(min: Vec2, max: Vec2) -> Self {
        Self { min, max }
    }

    pub fn with_min(min: Vec2) -> EditorObjectSizeRange {
        Self {
            min,
            max: Vec2::INFINITY,
        }
    }

    pub fn with_max(max: Vec2) -> Self {
        Self {
            min: Vec2::ZERO,
            max,
        }
    }
}

impl Default for EditorObjectSizeRange {
    fn default() -> Self {
        Self::with_max(Vec2::INFINITY)
    }
}

#[derive(Component, Default)]
pub struct EditorObjectRotation(pub f32);

#[derive(Component, Default)]
pub struct EditorObjectRotationSnap(pub f32);
