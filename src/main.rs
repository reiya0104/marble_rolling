mod component;
mod fps_text;
mod ui;
mod systems;
mod events;
mod resources;

use std::f32::consts::PI;

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, DefaultPlugins};

use component::*;
use events::*;
use resources::*;
use systems::*;
use ui::components::*;

const GRAVITY: f32 = 9.80665;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_event::<MarbleCreatedEvent>()
        .add_event::<CollisionEvent>()
        .insert_resource(MarbleCount { count: 0 })
        .add_startup_system(setup)
        // ui
        .add_startup_system(ui::systems::setup_ui)
        // fps_text
        .add_startup_system(fps_text::setup_fps_text)
        .add_system(fps_text::text_update_system)
        // debug position
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
        .add_system(
            ui::systems::update_mouse_coltroller_main_position
                .label("update_mouse_coltroller_main_position"),
        )
        .add_system(
            ui::systems::update_ui_view_by_mouse_coltroller_main_position
                .label("update_ui_view_by_mouse_coltroller_main_position")
                .after("update_mouse_coltroller_main_position"),
        )
        .add_system(
            ui::systems::update_ui_style_by_ui_view
                .label("update_ui_style_by_ui_view")
                .after("update_ui_view_by_mouse_coltroller_main_position"),
        )
        .add_system(
            update_rotation
                .label("update_rotation")
                .after("update_mouse_coltroller_main_position"),
        )
        .add_system(
            update_board_transform_by_rotation
                .label("update_board_transform_by_rotation")
                .after("update_rotation"),
        )
        .add_system(
            update_normal_vector_transform_by_rotation
                .label("update_normal_vector_transform_by_rotation")
                .after("update_rotation"),
        )
        // leg
        .add_system(
            create_leg.label("create_leg")
        )
        .add_system(
            update_legs.label("update_legs")
        )
        .add_system(collision_board_and_marble.label("collision_board_and_marble").after("update_legs"))
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
    let camera_position = Position::new(0.0, 1.0, 10.0);
    commands
        .spawn()
        .insert(Camera)
        .insert(camera_position.clone())
        .insert_bundle(PerspectiveCameraBundle {
            transform: camera_position.into(),
            ..default()
        });

    // board
    let rotation = Rotation::default();
    let board_position = Position::default();
    let board = commands
        .spawn()
        .insert(board_position.clone())
        .insert(rotation.clone())
        .insert(Board)
        .insert(PreviousRotation::from_quat(Quat::NAN))
        .insert_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(5.0, 0.1, 5.0))),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            transform: Transform {
                translation: board_position.vec3,
                rotation: rotation.clone().quat,
                scale: Vec3::ONE,
            },
            ..default()
        })
        .id();

    // normal vector
    let mut normal_vector_position = Position::new(0.0, 1.0, 0.0);
    let transform = &mut Transform::from_translation(normal_vector_position.vec3);
    transform.rotate_around(Vec3::ZERO, rotation.quat);
    normal_vector_position = Position::from_vec3(transform.translation);

    let normal_vector = commands
        .spawn()
        .insert(NormalVector::new(board))
        .insert(Acceleration::default())
        .insert(Velocity::default())
        .insert(normal_vector_position.clone())
        .insert(ObjectView::from_position(normal_vector_position.clone()))
        .insert_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius: 0.05,
                ..default()
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::PINK,
                ..default()
            }),
            transform: *transform,
            ..default()
        })
        .id();

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
    let marble2 = commands
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
        })
        .id();

    // Leg
    let leg_position = Position::new(1.0, 0.0, 1.0);
    commands
        .spawn()
        .insert(Leg::new(
            leg_position.vec3,
            marble2,
            normal_vector,
        ))
        .insert(leg_position.clone())
        .insert_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius: 0.1,
                ..default()
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::BLACK,
                ..default()
            }),
            transform: leg_position.into(),
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

fn debug_position(query: Query<(&Position, Entity), With<NormalVector>>) {
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

fn update_rotation(
    query_main: Query<(&UIPosition, &UIMaxSize), With<MouseControllerMain>>,
    time: Res<Time>,
    mut quat_rotation: Query<(&mut Rotation, &mut PreviousRotation)>,
) {
    for (main_position, max_radius) in query_main.iter() {
        // println!("{:?}, {:?}", main_position, max_radius);
        let rotate_base_vec = Vec3::new(-main_position.vec2.y, 0.0, main_position.vec2.x);
        // println!("{:?}", rotate_base_vec);
        let rotate = if rotate_base_vec != Vec3::ZERO {
            Quat::from_axis_angle(
                rotate_base_vec.normalize(),
                PI / 4.0 * (rotate_base_vec.length() / max_radius.size) * time.delta_seconds(),
                // PI / 4.0 * (rotate_base_vec.length() / max_radius.size),
            )
        } else {
            Quat::IDENTITY
        };
        for (mut rotation, mut pre_rotation) in quat_rotation.iter_mut() {
            pre_rotation.quat = rotation.quat;

            // next quat
            let quat = (rotation.quat * rotate).normalize();
            let (mut vec, angle) = quat.to_axis_angle();
            vec.y = 0.0;
            rotation.quat = if vec != Vec3::ZERO {
                Quat::from_axis_angle(
                    vec.normalize(),
                    if angle < PI / 4.0 { angle } else { PI / 4.0 },
                )
            } else {
                Quat::IDENTITY
            };
        }
    }
}

fn update_board_transform_by_rotation(mut query: Query<(&mut Transform, &Rotation), With<Board>>) {
    for (mut transform, rotation) in query.iter_mut() {
        transform.rotation = rotation.quat;
        // println!("board: {:?}, {:?}", transform, rotation);
    }
}

fn update_normal_vector_transform_by_rotation(
    mut query: Query<(&mut Position, &mut Transform, &NormalVector)>,
    query_board: Query<&Rotation, With<Board>>,
) {
    for (mut position, mut transform, normal_vector) in query.iter_mut() {
        if let Ok(rotation) = query_board.get(normal_vector.board_entity) {
            transform.translation = Vec3::new(0.0, 1.0, 0.0);
            transform.rotation = Quat::IDENTITY;
            transform.rotate_around(Vec3::ZERO, rotation.quat);
            position.vec3 = transform.translation;
            // println!("{:?}, {:?}, {:?}", position, transform, rotation);
        }
    }
}
