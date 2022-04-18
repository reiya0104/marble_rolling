pub mod component;
pub mod fps_text;

use std::f32::consts::PI;

use bevy::{prelude::*, DefaultPlugins, diagnostic::FrameTimeDiagnosticsPlugin};
use component::*;

const GRAVITY: f32 = 9.80665;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup)
        // fps_text
        .add_startup_system(fps_text::setup_fps_text)
        .add_system(fps_text::text_update_system)
        // .add_system(debug_position)
        .add_system(update_velocity_by_acceleration.label("update_velocity_by_acceleration"))
        .add_system(
            update_position_by_velocity
                .label("update_position_by_velocity")
                .after("update_velocity_by_acceleration"),
        )
        .add_system(
            update_object_view_by_position
                .label("update_object_view_by_position")
                .after("update_position_by_velocity"),
        )
        .add_system(
            update_transform_by_object_view
                .label("update_transform_by_object_view")
                .after("update_object_view_by_position"),
        )
        .add_system(create_marble)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
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
    let gravity = GRAVITY / 10.0;
    let marble_position = Position::new(0.0, 4.0, 0.0);
    let marble_acceleration = Acceleration::new(0.0, -gravity, 0.0);
    let marble_velocity = Velocity::default();

    commands
        .spawn()
        .insert(Marble)
        .insert(marble_acceleration.clone())
        .insert(marble_velocity.clone())
        .insert(marble_position.clone())
        .insert(ObjectView::from_position(marble_position.clone()))
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

    // marble2
    let marble_position = Position::new(1.0, 5.0, 1.0);
    commands
        .spawn()
        .insert(Marble)
        .insert(marble_acceleration.clone())
        .insert(marble_velocity.clone())
        .insert(marble_position.clone())
        .insert(ObjectView::from_position(marble_position.clone()))
        .insert_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius: 0.5,
                ..default()
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::LIME_GREEN,
                ..default()
            }),
            transform: marble_position.clone().into(),
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

    commands.spawn().insert_bundle(ImageBundle {
        style: Style {
            size: Size::new(Val::Px(150.0), Val::Px(150.0)),
            position_type: PositionType::Absolute,
            position: Rect {
                right: Val::Px(50.0),
                bottom: Val::Px(50.0),
                ..default()
            },
            ..default()
        },
        image: asset_server.load("image/mouse_controller_base.png").into(),
        ..default()
    }).with_children(|parent| {
        parent.spawn_bundle(ImageBundle {
            style: Style {
                size: Size::new(Val::Px(100.0), Val::Px(100.0)),
                position_type: PositionType::Absolute,
                position: Rect {
                    right: Val::Px(0.0),
                    bottom: Val::Px(0.0),
                    ..default()
                },
                ..default()
            },
            image: asset_server.load("image/mouse_controller_main.png").into(),
            ..default()
        });
    });

    // commands.spawn().insert_bundle(ImageBundle {
    //     style: Style {
    //         size: Size::new(Val::Px(100.0), Val::Px(100.0)),
    //         position_type: PositionType::Absolute,
    //         position: Rect {
    //             right: Val::Px(50.0),
    //             bottom: Val::Px(50.0),
    //             ..default()
    //         },
    //         ..default()
    //     },
    //     image: asset_server.load("image/mouse_controller_main.png").into(),
    //     ..default()
    // });
}

fn debug_position(query: Query<(&Position, Entity)>) {
    for (position, entity) in query.iter() {
        println!("{:?}, {:?}", position, entity);
    }
}

fn update_velocity_by_acceleration(
    time: Res<Time>,
    mut query: Query<(&mut Velocity, &Acceleration, Entity)>,
) {
    for (mut velocity, acceleration, entity) in query.iter_mut() {
        // println!("{:?}, {:?}, {:?}", velocity, acceleration, entity);
        velocity.vec3 += time.delta_seconds() * acceleration.vec3;
    }
}

fn update_position_by_velocity(
    time: Res<Time>,
    mut query: Query<(&mut Position, &Velocity, Entity)>,
) {
    for (mut position, velocity, entity) in query.iter_mut() {
        // println!("{:?}, {:?}, {:?}", position, velocity, entity);
        position.vec3 += time.delta_seconds() * velocity.vec3;
    }
}

fn update_object_view_by_position(mut query: Query<(&mut ObjectView, &Position, Entity)>) {
    for (mut object_view, position, entity) in query.iter_mut() {
        // println!("{:?}, {:?}, {:?}", object_view, position, entity);
        object_view.position = position.vec3;
    }
}

fn update_transform_by_object_view(mut query: Query<(&mut Transform, &ObjectView, Entity)>) {
    for (mut transform, object_view, entity) in query.iter_mut() {
        // println!("{:?}, {:?}, {:?}", transform, object_view, entity);
        transform.translation = object_view.position;
    }
}

fn create_marble(
    input: Res<Input<KeyCode>>,
    input_mause: Res<Input<MouseButton>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if input.pressed(KeyCode::Space) {
        // marbles
        let gravity = GRAVITY / 10.0;
        let marble_position = Position::new(0.0, 4.0, 0.0);
        let marble_acceleration = Acceleration::new(0.0, -gravity, 0.0);
        let marble_velocity = Velocity::default();
        commands
            .spawn()
            .insert(Marble)
            .insert(marble_acceleration.clone())
            .insert(marble_velocity.clone())
            .insert(marble_position.clone())
            .insert(ObjectView::from_position(marble_position.clone()))
            .insert_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::UVSphere {
                    radius: 0.5,
                    ..default()
                })),
                material: materials.add(StandardMaterial {
                    base_color: Color::LIME_GREEN,
                    ..default()
                }),
                transform: marble_position.clone().into(),
                ..default()
            });
    }
}
