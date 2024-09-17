use bevy::prelude::{Color, Component};

#[derive(Component, Copy, Clone)]
pub struct InfiniteGridOptions {
    pub x_axis_color: Color,
    pub z_axis_color: Color,
    pub minor_line_color: Color,
    pub major_line_color: Color,
    pub fadeout_distance: f32,
    pub dot_fadeout_strength: f32,
    pub scale: f32,
}

impl Default for InfiniteGridOptions {
    fn default() -> Self {
        Self {
            x_axis_color: Color::srgb(1.0, 0.2, 0.2),
            z_axis_color: Color::srgb(0.2, 0.2, 1.0),
            minor_line_color: Color::srgb(0.1, 0.1, 0.1),
            major_line_color: Color::srgb(0.25, 0.25, 0.25),
            fadeout_distance: 100.,
            dot_fadeout_strength: 0.25,
            scale: 1.,
        }
    }
}
