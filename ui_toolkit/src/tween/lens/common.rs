use bevy::prelude::Val;
use bevy::ui::UiRect;

pub struct ValSize {
    pub width: Val,
    pub height: Val,
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

pub trait ForcePositive {
    fn force_positive(self) -> Self;
}

impl ForcePositive for Val {
    fn force_positive(self) -> Self {
        match self {
            Val::Auto => self,
            Val::Px(px) => Val::Px(px.max(0.0)),
            Val::Percent(percent) => Val::Percent(percent.max(0.0)),
            Val::Vw(vw) => Val::Vw(vw.max(0.0)),
            Val::Vh(vh) => Val::Vh(vh.max(0.0)),
            Val::VMin(vm) => Val::VMin(vm.max(0.0)),
            Val::VMax(vm) => Val::VMax(vm.max(0.0)),
        }
    }
}

impl ForcePositive for UiRect {
    fn force_positive(mut self) -> Self {
        self.top = self.top.force_positive();
        self.bottom = self.bottom.force_positive();
        self.right = self.right.force_positive();
        self.left = self.left.force_positive();
        self
    }
}

pub trait LinearInterpolate {
    fn lerp(&self, other: Self, ratio: f32) -> Self;
}

impl LinearInterpolate for Val {
    fn lerp(&self, other: Self, ratio: f32) -> Self {
        match (self, other) {
            (Val::Percent(start), Val::Percent(end)) => {
                Val::Percent((end - start).mul_add(ratio, *start))
            }
            (Val::Px(start), Val::Px(end)) => Val::Px((end - start).mul_add(ratio, *start)),
            (Val::Vw(start), Val::Vw(end)) => Val::Vw((end - start).mul_add(ratio, *start)),
            (Val::Vh(start), Val::Vh(end)) => Val::Vh((end - start).mul_add(ratio, *start)),
            _ => *self,
        }
    }
}
