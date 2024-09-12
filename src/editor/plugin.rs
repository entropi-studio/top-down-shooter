use crate::editor::{
    EditorObjectPosition, EditorObjectPositionSnap, EditorObjectSize, EditorObjectSizeRange,
    EditorObjectSizeSnap, EditorObjectsPlugin, EditorWallBundle,
};
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::{egui, EguiContexts};

#[derive(Default)]
struct WriteLevelDialogState {
    open: bool,
}

#[derive(Resource, Default)]
struct EditorState {
    level_name: String,
    write_level_dialog: WriteLevelDialogState,
}

pub struct ShootEditorPlugin;

impl Plugin for ShootEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EditorObjectsPlugin)
            .insert_resource(EditorState { ..default() })
            .add_systems(Startup, init)
            .add_systems(
                Update,
                (
                    update_interface,
                    update_object_positions,
                    update_object_sizes,
                    update_object_rotations,
                ),
            );

        return;
    }
}

fn init(mut commands: Commands) {
    commands.spawn((
        EditorWallBundle {
            position_snap: EditorObjectPositionSnap(Vec2::new(20.0, 20.0)),
            ..default()
        },
        EditorObjectSizeSnap(Vec2::new(1.0, 1.0)),
        EditorObjectSizeRange::with_min(Vec2::ONE),
    ));
}

fn update_interface(mut contexts: EguiContexts, mut state: ResMut<EditorState>) {
    egui::SidePanel::left("Editor Panel Left")
        .resizable(true)
        .show(contexts.ctx_mut(), |ui| {
            ui.label("Editor");

            ui.text_edit_singleline(&mut state.level_name);

            if ui.button("Write level").clicked() {
                state.write_level_dialog.open = true;
            }

            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        });

    if state.write_level_dialog.open {
        egui::Window::new("Write Level").show(contexts.ctx_mut(), |ui| {
            ui.text_edit_singleline(&mut state.level_name);

            ui.horizontal(|ui| {
                if ui.button("Cancel").clicked() {
                    state.write_level_dialog.open = false;
                }

                if ui.button("Write").clicked() {
                    state.write_level_dialog.open = false;
                }
            });
        });
    }
}

fn update_object_positions(
    mut query: Query<(&mut EditorObjectPosition, Option<&EditorObjectPositionSnap>)>,
    query_camera: Query<(&Camera, &GlobalTransform)>,
    query_window: Query<&Window, With<PrimaryWindow>>,
) {
    let (camera, camera_transform) = query_camera.single();
    let Ok(window) = query_window.get_single() else {
        return;
    };

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let window_size = Vec2::new(window.width(), window.height());
    let mut mouse_ndc = (cursor_position / window_size) * 2.0 - Vec2::ONE;
    mouse_ndc = Vec2::new(mouse_ndc.x, -mouse_ndc.y);
    let ndc_to_world = camera_transform.compute_matrix() * camera.clip_from_view().inverse();
    let projection = ndc_to_world.project_point3(mouse_ndc.extend(-1.0));
    let projection = Vec2::new(projection.x, projection.y);

    for (mut position, snap) in query.iter_mut() {
        if let Some(EditorObjectPositionSnap(snap)) = snap {
            let mut projection = projection;
            if *snap == Vec2::ZERO {
            } else if snap.x == 0.0 {
                projection.y = (projection.y / snap.y).round() * snap.y;
            } else if snap.y == 0.0 {
                projection.x = (projection.x / snap.x).round() * snap.x;
            } else {
                projection = (projection / *snap).round() * *snap;
            }
            position.0 = projection;
        } else {
            position.0 = projection;
        }
    }
}

fn update_object_sizes(
    mut query: Query<(
        &mut EditorObjectSize,
        Option<&EditorObjectSizeSnap>,
        Option<&EditorObjectSizeRange>,
    )>,
    mut input_wheel: EventReader<MouseWheel>,
    input_mouse: Res<ButtonInput<MouseButton>>,
    input_keyboard: Res<ButtonInput<KeyCode>>,
) {
    let size_factor = if input_keyboard.pressed(KeyCode::ShiftLeft) {
        if input_keyboard.pressed(KeyCode::ControlLeft) {
            Vec2::new(1.0, 1.0)
        } else {
            Vec2::new(1.0, 0.0)
        }
    } else if input_keyboard.pressed(KeyCode::ControlLeft) {
        Vec2::new(0.0, 1.0)
    } else {
        Vec2::ZERO
    };

    let mut size_delta = Vec2::ZERO;
    for event in input_wheel.read() {
        size_delta += size_factor * event.y;
    }

    let reset_size = input_mouse.pressed(MouseButton::Middle);

    for (mut size, snap, range) in query.iter_mut() {
        if reset_size {
            size.0 = Vec2::ZERO;
            continue;
        }

        if let Some(EditorObjectSizeSnap(snap)) = snap {
            if size_delta.x != 0.0 && size_delta.x.abs() < snap.x {
                size_delta.x = snap.x * size_delta.x.signum();
            }
            if size_delta.y != 0.0 && size_delta.y.abs() < snap.y {
                size_delta.y = snap.y * size_delta.y.signum();
            }

            size.0 += size_delta;

            let mut resize = size.0;
            if *snap == Vec2::ZERO {
            } else if snap.x == 0.0 {
                resize.y = (resize.y / snap.y).round() * snap.y;
            } else if snap.y == 0.0 {
                resize.x = (resize.x / snap.x).round() * snap.x;
            } else {
                resize = (resize / *snap).round() * *snap;
            }
            size.0 = resize;
        } else {
            size.0 += size_delta;
        }

        if let Some(range) = range {
            size.0 = size.0.clamp(range.min, range.max);
        } else {
            size.0 = size.0.max(Vec2::ZERO);
        }
    }
}

fn update_object_rotations() {}
