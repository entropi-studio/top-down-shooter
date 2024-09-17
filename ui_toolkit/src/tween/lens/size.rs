use bevy::ui::{Style, Val};
use bevy_tweening::{Lens, Targetable};

pub struct ValSize {
    width: Val,
    height: Val,
}

impl ValSize {
    pub fn all(all: Val) -> ValSize {
        Self {
            width: all,
            height: all,
        }
    }

    pub fn auto() -> ValSize {
        Self {
            width: Val::Auto,
            height: Val::Auto,
        }
    }

    pub fn width(width: Val) -> ValSize {
        Self {
            width,
            height: Val::default(),
        }
    }

    pub fn height(height: Val) -> ValSize {
        Self {
            width: Val::default(),
            height,
        }
    }

    pub fn with_width(mut self, width: Val) -> ValSize {
        self.width = width;
        self
    }

    pub fn with_height(mut self, height: Val) -> ValSize {
        self.height = height;
        self
    }
}

pub struct UiSizeLens {
    pub start: ValSize,
    pub end: ValSize,
}

fn lerp_val(start: &Val, end: &Val, ratio: f32) -> Val {
    match (start, end) {
        (Val::Percent(start), Val::Percent(end)) => {
            Val::Percent((end - start).mul_add(ratio, *start))
        }
        (Val::Px(start), Val::Px(end)) => Val::Px((end - start).mul_add(ratio, *start)),
        (Val::Vw(start), Val::Vw(end)) => Val::Vw((end - start).mul_add(ratio, *start)),
        (Val::Vh(start), Val::Vh(end)) => Val::Vh((end - start).mul_add(ratio, *start)),
        _ => *start,
    }
}

impl Lens<Style> for UiSizeLens {
    fn lerp(&mut self, target: &mut dyn Targetable<Style>, ratio: f32) {
        target.width = lerp_val(&self.start.width, &self.end.width, ratio);
        target.height = lerp_val(&self.start.height, &self.end.height, ratio);
    }
}
