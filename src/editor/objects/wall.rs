use crate::editor::{
    EditorObject, EditorObjectPosition, EditorObjectPositionSnap, EditorObjectRotation,
    EditorObjectSize, EditorObjectSizeRange,
};
use crate::objects::{WallObject, WallObjectBundle};
use bevy::color::palettes::basic::RED;
use bevy::color::palettes::css::DARK_CYAN;
use bevy::math::NormedVectorSpace;
use bevy::prelude::*;
use bevy::render::render_resource::ShaderRef::Handle;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2d, Mesh2dHandle};
use bevy_egui::{egui, EguiContexts};
use std::f32::consts::PI;
use std::fmt::format;

#[derive(Default)]
enum WallCornerType {
    None,
    #[default]
    Sharp,
}

#[derive(Component, Default)]
pub struct EditorWall {
    from_position: Option<Vec2>,
    to_position: Vec2,
    current_size: Vec2,
    thickness: f32,
    corner_type: WallCornerType,
}

#[derive(Bundle)]
pub struct EditorWallBundle {
    pub position: EditorObjectPosition,
    pub position_snap: EditorObjectPositionSnap,
    pub size: EditorObjectSize,
    pub size_range: EditorObjectSizeRange,
    pub wall: EditorWall,
    pub editor_object: EditorObject,
}

impl Default for EditorWallBundle {
    fn default() -> Self {
        Self {
            position: EditorObjectPosition::default(),
            position_snap: EditorObjectPositionSnap(Vec2::new(20.0, 20.0)),
            size: EditorObjectSize::default(),
            size_range: EditorObjectSizeRange::with_min(Vec2::splat(2.0)),
            wall: EditorWall::default(),
            editor_object: EditorObject,
        }
    }
}

pub struct EditorWallPlugin;
impl Plugin for EditorWallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, wall_interface)
            .add_systems(Update, (wall_init, wall_update));
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

fn wall_interface(
    mut query: Query<(&mut EditorWall, &EditorObjectPosition, &EditorObjectSize)>,
    mut gizmos: Gizmos,
    mut contexts: EguiContexts,
) {
    for (mut wall, EditorObjectPosition(position), EditorObjectSize(size)) in query.iter_mut() {
        egui::Window::new("[Placing] Wall").show(contexts.ctx_mut(), |ui| {
            if let Some(start_position) = wall.from_position {
                gizmos.circle_2d(start_position, 1.0, DARK_CYAN);
                gizmos.circle_2d(*position, 1.0, RED);
                ui.label(format!(
                    "From: ({:^10}, {:^10})",
                    start_position.x.round(),
                    start_position.y.round()
                ));
                ui.label(format!(
                    "To:   ({:^10}, {:^10})",
                    position.x.round(),
                    position.y.round()
                ));
            } else {
                ui.label(format!(
                    "Current ({:^10}, {:^10})",
                    position.x.round(),
                    position.y.round()
                ));
            }
            ui.label(format!("Thickness: {:^10}", size.x));
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
    mut materials: ResMut<Assets<ColorMaterial>>,
    input_mouse: Res<ButtonInput<MouseButton>>,
    time: Res<Time>,
    mut commands: Commands,
    mut egui_contexts: EguiContexts,
) {
    if let Some(ctx_mut) = egui_contexts.try_ctx_mut() {
        if ctx_mut.wants_pointer_input() {
            return;
        }
    }

    for (mut wall, mut transform, mesh, EditorObjectPosition(position), EditorObjectSize(size)) in
        query.iter_mut()
    {
        let interpolation = time.delta_seconds() * 10.0;
        let mut wall_size = Vec2::ZERO;
        let mut should_interpolate_wall_size = false;

        wall.to_position = wall.to_position.lerp(*position, interpolation * 2.0);

        if input_mouse.pressed(MouseButton::Left) {
            wall.thickness = wall.thickness.lerp(size.x, interpolation);
            if let Some(start_position) = wall.from_position {
                let diff = start_position - wall.to_position;
                let atan = f32::atan2(diff.y, diff.x);
                let angle = atan + (PI * 0.5);

                let middle = start_position.midpoint(wall.to_position);

                // transform.rotation = transform
                //     .rotation
                //     .lerp(Quat::from_rotation_z(angle), interpolation);
                // transform.translation = transform
                //     .translation
                //     .lerp(Vec3::new(middle.x, middle.y, 0.0), interpolation);
                transform.rotation = Quat::from_rotation_z(angle);
                transform.translation = Vec3::new(middle.x, middle.y, 0.0);

                let diff = start_position - transform.translation.xy();
                let length = diff.length() * 2.0 + wall.thickness;
                wall_size = Vec2::new(wall.thickness, length);
            } else {
                wall.from_position = Some(*position);
            }
        } else {
            if let Some(start_position) = wall.from_position {
                let diff = start_position - wall.to_position;
                let atan = f32::atan2(diff.y, diff.x);
                let angle = atan + (PI * 0.5);
                let middle = start_position.midpoint(wall.to_position);
                let diff = start_position - transform.translation.xy();
                let length = diff.length() * 2.0 + wall.thickness;
                let size = Vec2::new(wall.thickness, length);

                wall.from_position = None;
                commands.spawn(WallObjectBundle::new(
                    middle,
                    size,
                    angle,
                    100.0,
                    &mut meshes,
                    &mut materials,
                ));
            }

            should_interpolate_wall_size = true;

            wall.thickness = size.x;
            wall_size = Vec2::splat(wall.thickness);

            if transform.translation.xy().distance(*position) <= wall.thickness {
                transform.rotation = transform.rotation.lerp(Quat::IDENTITY, interpolation);
            }

            transform.translation = transform
                .translation
                .lerp(Vec3::new(position.x, position.y, 0.0), interpolation);
        }

        if let Some(mesh) = meshes.get_mut(mesh.0.id()) {
            if should_interpolate_wall_size {
                wall.current_size = wall.current_size.lerp(wall_size, interpolation);
            } else {
                wall.current_size = wall_size;
            }
            *mesh = Rectangle::from_size(wall.current_size).into();
        }
    }
}
