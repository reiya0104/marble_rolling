mod component;
mod events;
mod fps_text;
mod resouces;
mod systems;
mod ui;

use std::f32::consts::PI;

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, DefaultPlugins};
use rand::prelude::*;

use component::*;
use events::*;
use ui::components::*;

const GRAVITY: f32 = 9.80665;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .insert_resource(resouces::MarbleCount { count: 0 })
        .add_event::<MarbleCreatedEvent>()
        .add_startup_system(setup)
        // ui
        .add_startup_system(ui::systems::setup_ui)
        // fps_text
        .add_startup_system(fps_text::setup_fps_text)
        .add_system(fps_text::text_update_system)
        // event
        .add_system(systems::create_leg)
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
    let rotation = Rotation::from_quat(Quat::IDENTITY);
    let board_position = Position::default();
    commands
        .spawn()
        .insert(board_position.clone())
        .insert(rotation.clone())
        .insert(Board)
        .insert_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(5.0, 0.1, 5.0))),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            transform: Transform {
                translation: board_position.vec3,
                rotation: rotation.clone().quat,
                scale: Vec3::ONE,
            },
            ..default()
        });

    // normal vector
    let mut normal_vector_position = Position::new(0.0, 1.0, 0.0);
    let transform = &mut Transform::from_translation(normal_vector_position.vec3);
    transform.rotate_around(Vec3::ZERO, rotation.clone().quat);
    normal_vector_position = Position::from_vec3(transform.translation);

    commands
        .spawn()
        .insert(NormalVector)
        .insert(Acceleration::default())
        .insert(Velocity::default())
        .insert(normal_vector_position.clone())
        .insert(rotation.clone())
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

    // Perpendicular legs
    let position = Position::from_vec3(Vec3::ZERO);
    commands
        .spawn()
        .insert(Leg)
        .insert(position.clone())
        .insert_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius: 0.1,
                ..default()
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::BLACK,
                ..default()
            }),
            transform: position.into(),
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
    mut commands: Commands,
    mut query: Query<(&mut Position, &Velocity, Entity)>,
) {
    let min_altitude: f32 = -100.0;
    for (mut position, velocity, entity) in query.iter_mut() {
        println!("{:?}, {:?}, {:?}", position, velocity, entity);
        position.vec3 += time.delta_seconds() * velocity.vec3;

        // 高度 (position.vec3.y) が min_altitude を下回ったら， entity を削除する
        if position.vec3.y < min_altitude {
            commands.entity(entity).despawn();
        }
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
    mut event_writer: EventWriter<MarbleCreatedEvent>,
) {
    if input.pressed(KeyCode::Space) {
        let max_position_num = 2.5;

        // 乱数生成
        let mut rng = rand::thread_rng();
        let x_base: f32 = rng.gen();
        // let y_base: f32 = rng.gen();
        let z_base: f32 = rng.gen();

        // marbles
        let gravity = GRAVITY / 10.0;
        let marble_position = Position::new(
            if x_base < 0.5 {
                -x_base * max_position_num * 2.0
            } else {
                (x_base - 0.5) * max_position_num * 2.0
            },
            4.0,
            if z_base < 0.5 {
                -z_base * max_position_num * 2.0
            } else {
                (z_base - 0.5) * max_position_num * 2.0
            },
        );
        // let marble_position = Position::new(0.0, 4.0, 0.0);
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
                    radius: 0.1,
                    ..default()
                })),
                material: materials.add(StandardMaterial {
                    base_color: Color::BLUE,
                    ..default()
                }),
                transform: marble_position.clone().into(),
                ..default()
            });
        event_writer.send(MarbleCreatedEvent);
    }
}

fn update_rotation(
    query_main: Query<(&UIPosition, &UIMaxSize), With<MouseControllerMain>>,
    mut quat_rotation: Query<&mut Rotation>,
) {
    for (main_position, max_radius) in query_main.iter() {
        // println!("{:?}, {:?}", main_position, max_radius);
        let rotate_base_vec = Vec3::new(-main_position.vec2.y, 0.0, main_position.vec2.x);
        // println!("{:?}", rotate_base_vec);
        let rotate = if rotate_base_vec != Vec3::ZERO {
            Quat::from_axis_angle(
                rotate_base_vec.normalize(),
                PI / 4.0 * (rotate_base_vec.length() / max_radius.size),
            )
        } else {
            Quat::IDENTITY
        };
        for mut rotation in quat_rotation.iter_mut() {
            rotation.quat = rotate;
        }
    }
}

fn update_board_transform_by_rotation(mut query: Query<(&mut Transform, &Rotation), With<Board>>) {
    for (mut transform, rotation) in query.iter_mut() {
        transform.rotation = rotation.quat;
        // println!("{:?}, {:?}", transform, rotation);
    }
}

fn update_normal_vector_transform_by_rotation(
    mut query: Query<(&mut Position, &mut Transform, &Rotation), With<NormalVector>>,
) {
    for (mut position, mut transform, rotation) in query.iter_mut() {
        transform.translation = Vec3::new(0.0, 1.0, 0.0);
        transform.rotation = Quat::IDENTITY;
        transform.rotate_around(Vec3::ZERO, rotation.quat);
        position.vec3 = transform.translation;
        // println!("{:?}, {:?}, {:?}", position, transform, rotation);
    }
}
