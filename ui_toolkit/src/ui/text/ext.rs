use bevy::prelude::{default, TextBundle};
use bevy::text::TextStyle;

pub trait TextStyleExt {
    fn font_size(font_size: f32) -> Self;
}

impl TextStyleExt for TextStyle {
    fn font_size(font_size: f32) -> Self {
        Self {
            font_size,
            ..default()
        }
    }
}
