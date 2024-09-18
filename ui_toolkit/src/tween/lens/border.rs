use crate::tween::lens::common::ValSize;
use crate::tween::lens::ForcePositive;
use bevy::ui::{Style, UiRect, Val};
use bevy_tweening::{Lens, Targetable};
use sickle_ui::lerp::Lerp;

pub struct UiBorderLens {
    pub start: UiRect,
    pub end: UiRect,
}
impl Lens<Style> for UiBorderLens {
    fn lerp(&mut self, target: &mut dyn Targetable<Style>, ratio: f32) {
        target.border = self.start.lerp(self.end, ratio);
        target.border = target.border.force_positive();
    }
}
