pub mod component;

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
    // camera
    let camera_position = Position {
        vec3: Vec3::new(0.0, 0.0, 5.0),
    };
    commands
        .spawn()
        .insert(Camera)
        .insert(camera_position.clone())
        .insert_bundle(PerspectiveCameraBundle {
            transform: camera_position.into(),
            ..default()
        });

    // marble
    let marble_position = Position::default();
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
    let light_position = Position {
        vec3: Vec3::new(3.0, 5.0, 3.0),
    };
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
