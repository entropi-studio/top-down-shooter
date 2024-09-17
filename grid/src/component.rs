use crate::options::InfiniteGridOptions;
use bevy::prelude::{
    Bundle, Component, GlobalTransform, InheritedVisibility, Transform, ViewVisibility, Visibility,
};
use bevy::render::view::{NoFrustumCulling, VisibleEntities};

#[derive(Component, Default)]
pub struct InfiniteGrid;

#[derive(Bundle, Default)]
pub struct InfiniteGridBundle {
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub settings: InfiniteGridOptions,
    pub grid: InfiniteGrid,
    pub visibility: Visibility,
    pub view_visibility: ViewVisibility,
    pub inherited_visibility: InheritedVisibility,
    pub shadow_casters: VisibleEntities,
    pub no_frustum_culling: NoFrustumCulling,
}
