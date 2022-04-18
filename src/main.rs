pub mod component;

use std::f32::consts::PI;

use bevy::{prelude::*, DefaultPlugins};
use component::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(debug_position)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // ui camera
    commands.spawn_bundle(UiCameraBundle::default());

    // 3D camera
    let camera_position = Position::new(0.0, 0.0, 10.0);
    commands
        .spawn()
        .insert(Camera)
        .insert(camera_position.clone())
        .insert_bundle(PerspectiveCameraBundle {
            transform: camera_position.into(),
            ..default()
        });

    // board
    let pi = PI;
    let rotation = Quat::from_axis_angle(Vec3::new(-1.0, 0.0, 1.0).normalize(), -pi / 4.0);
    let board_position = Position::default();
    commands
        .spawn()
        .insert(board_position.clone())
        .insert(Board)
        .insert_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane {
                size: 5.0,
                ..default()
            })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            transform: Transform {
                translation: board_position.vec3,
                rotation: rotation.clone(),
                scale: Vec3::ONE,
            },
            ..default()
        });

    // marble
    let marble_position = Position::new(0.0, 2.0, 0.0);
    commands
        .spawn()
        .insert(Marble)
        .insert(marble_position.clone())
        .insert_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius: 0.5,
                ..default()
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::LIME_GREEN,
                ..default()
            }),
            transform: marble_position.into(),
            ..default()
        });

    // light
    let light_position = Position::new(3.0, 5.0, 3.0);
    commands
        .spawn()
        .insert(light_position.clone())
        .insert(Light)
        .insert_bundle(PointLightBundle {
            point_light: PointLight {
                intensity: 1500.0,
                shadows_enabled: true,
                ..default()
            },
            transform: light_position.into(),
            ..default()
        });
}

fn debug_position(query: Query<(&Position, Entity)>) {
    for (position, entity) in query.iter() {
        println!("{:?}, {:?}", position, entity);
    }
}
