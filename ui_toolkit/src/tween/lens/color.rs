use bevy::color::Color;
use bevy::prelude::{BorderColor, Mix};
use bevy_tweening::{Lens, Targetable};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct UiBorderColorLens {
    /// Start color.
    pub start: Color,
    /// End color.
    pub end: Color,
}

impl Lens<BorderColor> for UiBorderColorLens {
    fn lerp(&mut self, target: &mut dyn Targetable<BorderColor>, ratio: f32) {
        target.0 = self.start.mix(&self.end, ratio);
    }
}
