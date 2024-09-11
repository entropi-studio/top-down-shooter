use crate::player::PlayerBundle;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use voronoice::{BoundingBox, Voronoi, VoronoiBuilder};

fn generate_voronoi(shards: usize, radius: f64) -> Voronoi {
    let result = VoronoiBuilder::default()
        .generate_circle_sites(shards, radius)
        .set_bounding_box(BoundingBox::new_centered_square(1000.0))
        .set_lloyd_relaxation_iterations(20)
        .build()
        .expect("Failed to build voronoi for player");

    result
}

fn generate_triangles_from_voronoi(voronoi: &Voronoi) -> Vec<Triangle2d> {
    let site_vectors = voronoi
        .sites()
        .iter()
        .map(|point| Vec2::new(point.x as f32, point.y as f32))
        .collect::<Vec<_>>();
    let triangles = voronoi
        .triangulation()
        .triangles
        .chunks(3)
        .map(|triangle| {
            let triangle = Triangle2d::new(
                site_vectors[triangle[0]],
                site_vectors[triangle[1]],
                site_vectors[triangle[2]],
            );

            triangle
        })
        .filter(|t| {
            let [v1, v2, v3] = t.vertices;

            true
        })
        .collect::<Vec<_>>();

    triangles
}

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let voronoi = generate_voronoi(100, 500.0);
    let triangles = generate_triangles_from_voronoi(&voronoi);

    commands
        .spawn(PlayerBundle::default())
        .with_children(|parent| {
            for (i, &triangle) in triangles.iter().enumerate() {
                let mesh_handle = meshes.add(triangle);
                let ucolor = i as u8 % 255;

                parent.spawn(MaterialMesh2dBundle {
                    mesh: mesh_handle.into(),
                    material: materials
                        .add(ColorMaterial::from(Color::srgb_u8(ucolor, ucolor, ucolor))),
                    ..default()
                });
            }
        });
}
