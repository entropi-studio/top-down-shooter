use crate::editor::{
    EditorObjectPosition, EditorObjectPositionSnap, EditorObjectRotation, EditorObjectSize,
};
use bevy::prelude::*;
use bevy::render::render_resource::ShaderRef::Handle;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2d, Mesh2dHandle};
use bevy_egui::{egui, EguiContexts};
use std::fmt::format;

#[derive(Component, Default)]
pub struct EditorWall {
    current_size: Vec2,
}

#[derive(Bundle, Default)]
pub struct EditorWallBundle {
    pub position: EditorObjectPosition,
    pub position_snap: EditorObjectPositionSnap,
    pub size: EditorObjectSize,
    pub rotation: EditorObjectRotation,
    pub wall: EditorWall,
}

pub(super) fn wall_object_init(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(Entity), (Added<EditorWall>)>,
) {
    for (entity) in query.iter_mut() {
        commands.entity(entity).insert(MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::new(10.0, 100.0)).into(),
            material: materials.add(Color::WHITE),
            ..default()
        });
    }
}

pub(super) fn wall_object_update(
    mut query: Query<(
        &mut EditorWall,
        &mut Transform,
        &Mesh2dHandle,
        &EditorObjectPosition,
        &EditorObjectSize,
    )>,
    mut meshes: ResMut<Assets<Mesh>>,
    time: Res<Time>,
    mut contexts: EguiContexts,
) {
    for (mut wall, mut transform, mesh, EditorObjectPosition(position), EditorObjectSize(size)) in
        query.iter_mut()
    {
        egui::Window::new("[Placing] Wall").show(contexts.ctx_mut(), |ui| {
            ui.label(format!("Position ({:^10}, {:^10})", position.x.round(), position.y.round()));
            ui.label(format!("Size ({:^10}, {:^10})", size.x.round(), size.y.round()));
        });

        transform.translation = transform.translation.lerp(
            Vec3::new(position.x, position.y, 0.0),
            time.delta_seconds() * 10.0,
        );

        wall.current_size = wall.current_size.lerp(*size, time.delta_seconds() * 10.0);

        if let Some(mesh) = meshes.get_mut(mesh.0.id()) {
            let rect = Rectangle::from_size(wall.current_size * 20.0);
            *mesh = rect.into();
        }
    }
}
