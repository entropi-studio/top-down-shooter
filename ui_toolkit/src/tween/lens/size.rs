use crate::tween::lens::common::ValSize;
use bevy::ui::{Style, Val};
use bevy_tweening::{Lens, Targetable};
use sickle_ui::lerp::Lerp;

pub struct UiSizeLens {
    pub start: ValSize,
    pub end: ValSize,
}
impl Lens<Style> for UiSizeLens {
    fn lerp(&mut self, target: &mut dyn Targetable<Style>, ratio: f32) {
        target.width = self.start.width.lerp(self.end.width, ratio);
        target.height = self.start.height.lerp(self.end.height, ratio);
    }
}
