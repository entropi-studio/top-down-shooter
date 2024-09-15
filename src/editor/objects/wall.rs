use crate::editor::{
    EditorObject, EditorObjectPosition, EditorObjectPositionSnap, EditorObjectRotation,
    EditorObjectSize,
};
use crate::objects::{WallObject, WallObjectBundle};
use bevy::prelude::*;
use bevy::render::render_resource::ShaderRef::Handle;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2d, Mesh2dHandle};
use bevy_egui::{egui, EguiContexts};
use std::fmt::format;

#[derive(Component, Default)]
pub struct EditorWall {
    current_size: Vec2,
}

#[derive(Bundle)]
pub struct EditorWallBundle {
    pub position: EditorObjectPosition,
    pub position_snap: EditorObjectPositionSnap,
    pub size: EditorObjectSize,
    pub rotation: EditorObjectRotation,
    pub wall: EditorWall,
    pub editor_object: EditorObject,
}

impl Default for EditorWallBundle {
    fn default() -> Self {
        Self {
            position: EditorObjectPosition::default(),
            position_snap: EditorObjectPositionSnap(Vec2::new(20.0, 20.0)),
            size: EditorObjectSize::default(),
            rotation: EditorObjectRotation::default(),
            wall: EditorWall::default(),
            editor_object: EditorObject,
        }
    }
}

pub struct EditorWallPlugin;
impl Plugin for EditorWallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (wall_init, wall_update, wall_handle_place));
    }
}

fn wall_init(
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

fn wall_update(
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
            ui.label(format!(
                "Position ({:^10}, {:^10})",
                position.x.round(),
                position.y.round()
            ));
            ui.label(format!(
                "Size ({:^10}, {:^10})",
                size.x.round(),
                size.y.round()
            ));
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

fn wall_handle_place(
    mut commands: Commands,
    query: Query<
        (
            &EditorObjectPosition,
            &EditorObjectSize,
            &EditorObjectRotation,
        ),
        With<EditorWall>,
    >,
    query_walls: Query<Entity, With<WallObject>>,
    input_mouse: Res<ButtonInput<MouseButton>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if input_mouse.pressed(MouseButton::Left) {
        for (
            EditorObjectPosition(position),
            EditorObjectSize(size),
            EditorObjectRotation(rotation),
        ) in query.iter()
        {
            commands.spawn(WallObjectBundle::new(
                *position,
                *size * 20.0,
                *rotation,
                query_walls.iter().count() as f32,
                &mut meshes,
                &mut materials,
            ));
        }
    }
}
