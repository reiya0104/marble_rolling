mod component;
mod consts;
mod events;
mod fps_text;
mod resources;
mod systems;
mod ui;

use std::f32::consts::PI;

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, DefaultPlugins};

use component::*;
use consts::*;
use events::*;
use resources::*;
use systems::*;
use ui::components::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_event::<MarbleCreatedEvent>()
        .add_event::<CollisionEvent>()
        .insert_resource(MarbleCount { count: 0 })
        .init_resource::<StageRotation>()
        .insert_resource(TileState::new())
        .insert_resource(TileOrigin::new())
        .add_startup_system(setup)
        .add_startup_system(setup_camera)
        .add_startup_system(debug_res)
        .add_startup_system(setup_tile_origin)
        .add_startup_system(setup_tile)
        // ui
        .add_startup_system(ui::systems::setup_ui)
        // fps_text
        .add_startup_system(fps_text::setup_fps_text)
        .add_system(fps_text::text_update_system)
        // debug position
        // .add_system(debug_position)
        .add_system(update_velocity_by_force.label("update_velocity_by_force"))
        .add_system(
            update_position_by_velocity
                .label("update_position_by_velocity")
                .after("update_velocity_by_force"),
        )
        .add_system(update_object_view_by_position.label("update_object_view_by_position"))
        .add_system(
            update_transform_by_object_view
                .label("update_transform_by_object_view")
                .after("update_object_view_by_position"),
        )
        // .add_system(create_marble)
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
        // .add_system(
        //     update_normal_vector_transform_by_rotation
        //         .label("update_normal_vector_transform_by_rotation")
        //         .after("update_rotation"),
        // )
        .add_system(
            update_tile_transform_by_rotation
                .label("update_tile_transform_by_rotation")
                .after("update_rotation"),
        )
        .add_system(
            update_tile_origin
                .label("update_tile_origin")
                .after("update_rotation"),
        )
        .add_system(
            get_marble_relative_position_from_tile_origin
                .label("get_marble_relative_position_from_tile_origin")
                .after("update_tile_transform_by_rotation"),
        )
        // leg
        .add_system(create_leg.label("create_leg"))
        .add_system(
            update_legs
                .label("update_legs")
                .before("collision_board_and_marble"),
        )
        .add_system(
            collision_board_and_marble
                .label("collision_board_and_marble")
                .after("update_position_by_velocity")
                .before("update_object_view_by_position"),
        )
        .add_system(
            reset_force
                .label("reset_force")
                .before("collision_board_and_marble"),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // board
    // let rotation = Rotation::default();
    // let board_position = Position::default();
    // let board = commands
    //     .spawn()
    //     .insert(board_position.clone())
    //     .insert(rotation.clone())
    //     .insert(Board)
    //     .insert(Mass::new(1.0))
    //     .insert(PreviousRotation::from_quat(Quat::NAN))
    //     .insert_bundle(PbrBundle {
    //         mesh: meshes.add(Mesh::from(shape::Box::new(20.0, 0.1, 20.0))),
    //         material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
    //         transform: Transform {
    //             translation: board_position.vec3,
    //             rotation: rotation.clone().quat,
    //             scale: Vec3::ONE,
    //         },
    //         visibility: Visibility { is_visible: false },
    //         ..default()
    //     })
    //     .id();

    // normal vector
    // let mut normal_vector_position = Position::new(0.0, 1.0, 0.0);
    // let transform = &mut Transform::from_translation(normal_vector_position.vec3);
    // transform.rotate_around(Vec3::ZERO, rotation.quat);
    // normal_vector_position = Position::from_vec3(transform.translation);

    // let normal_vector = commands
    //     .spawn()
    //     .insert(NormalVector::new(board))
    //     .insert(Velocity::default())
    //     .insert(normal_vector_position.clone())
    //     .insert(ObjectView::from_position(normal_vector_position.clone()))
    //     .insert_bundle(PbrBundle {
    //         mesh: meshes.add(Mesh::from(shape::UVSphere {
    //             radius: 0.05,
    //             ..default()
    //         })),
    //         material: materials.add(StandardMaterial {
    //             base_color: Color::PINK,
    //             ..default()
    //         }),
    //         transform: *transform,
    //         ..default()
    //     })
    //     .id();

    // marble
    // let gravity = GRAVITY / 10.0;
    // let marble_mass = Mass::new(1.0);
    // let marble_position = Position::new(0.0, 4.0, 0.0);
    // let marble_force = Force::new(0.0, -marble_mass.mass * gravity, 0.0);
    // let marble_velocity = Velocity::default();

    // commands
    //     .spawn()
    //     .insert(Marble)
    //     .insert(marble_mass.clone())
    //     .insert(marble_force.clone())
    //     .insert(marble_velocity.clone())
    //     .insert(marble_position.clone())
    //     .insert(ObjectView::from_position(marble_position.clone()))
    //     .insert_bundle(PbrBundle {
    //         mesh: meshes.add(Mesh::from(shape::UVSphere {
    //             radius: 0.5,
    //             ..default()
    //         })),
    //         material: materials.add(StandardMaterial {
    //             base_color: Color::LIME_GREEN,
    //             ..default()
    //         }),
    //         transform: marble_position.into(),
    //         ..default()
    //     });

    // marble2
    // let marble_position = Position::new(1.0, 5.0, 1.0);
    // let marble2 = commands
    //     .spawn()
    //     .insert(Marble)
    //     .insert(marble_mass)
    //     .insert(marble_force)
    //     .insert(marble_velocity)
    //     .insert(marble_position.clone())
    //     .insert(ObjectView::from_position(marble_position.clone()))
    //     .insert_bundle(PbrBundle {
    //         mesh: meshes.add(Mesh::from(shape::UVSphere {
    //             radius: 0.5,
    //             ..default()
    //         })),
    //         material: materials.add(StandardMaterial {
    //             base_color: Color::LIME_GREEN,
    //             ..default()
    //         }),
    //         transform: marble_position.into(),
    //         ..default()
    //     })
    //     .id();

    // marble3
    let marble_position = Position::new(-1.0, 3.0, -2.0);
    let marble3 = commands
        .spawn()
        .insert(Marble)
        // .insert(marble_mass)
        // .insert(marble_force)
        // .insert(marble_velocity)
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
        })
        .id();

    commands.spawn().insert_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::UVSphere {
            radius: 0.05,
            ..default()
        })),
        material: materials.add(StandardMaterial {
            base_color: Color::PURPLE,
            ..default()
        }),
        transform: Transform::from_translation(Vec3::ZERO),
        ..default()
    });

    // Leg
    // let leg_position = Position::new(1.0, 0.0, 1.0);
    // commands
    //     .spawn()
    //     .insert(Leg::new(leg_position.vec3, marble2, normal_vector))
    //     .insert(leg_position.clone())
    //     .insert_bundle(PbrBundle {
    //         mesh: meshes.add(Mesh::from(shape::UVSphere {
    //             radius: 0.1,
    //             ..default()
    //         })),
    //         material: materials.add(StandardMaterial {
    //             base_color: Color::BLACK,
    //             ..default()
    //         }),
    //         transform: leg_position.into(),
    //         ..default()
    //     });
}

fn debug_res(tile_state: Res<TileState>) {
    println!("{:?}", tile_state.tile_state);
}

fn setup_camera(mut commands: Commands) {
    // ui camera
    commands.spawn_bundle(UiCameraBundle::default());

    // 3D camera
    let camera_position = Position::new(0.0, 2.0, 10.0);
    commands
        .spawn()
        .insert(Camera)
        .insert(camera_position.clone())
        .insert_bundle(PerspectiveCameraBundle {
            transform: Transform::from_translation(camera_position.vec3), // .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y)
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

fn setup_tile_origin(
    tile_origin: Res<TileOrigin>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // tile_origin
    let tile_origin_position = Position::from_vec3(tile_origin.base_position);
    let tile_origin = commands
        .spawn()
        .insert(TileOriginComponent)
        .insert(tile_origin_position.clone())
        .insert(ObjectView::from_position(tile_origin_position.clone()))
        .insert_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius: 0.1,
                ..default()
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::ORANGE,
                ..default()
            }),
            transform: tile_origin_position.into(),
            ..default()
        })
        .id();
}

fn setup_tile(
    tile_state: Res<TileState>,
    tile_origin: Res<TileOrigin>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut x_place = tile_origin.base_position.x + TILE_SHORT_WIDTH / 2.0;
    for x in 0..TILE_ALL_X_COUNT {
        let x_length = if x % 2 == 0 {
            TILE_SHORT_WIDTH
        } else {
            TILE_LONG_WIDTH
        };
        let mut y_place = tile_origin.base_position.y - TILE_SHORT_WIDTH / 2.0;
        for y in 0..TILE_ALL_Y_COUNT {
            let y_length = if y % 2 == 0 {
                TILE_SHORT_WIDTH
            } else {
                TILE_LONG_WIDTH
            };
            let mut z_place = tile_origin.base_position.z + TILE_SHORT_WIDTH / 2.0;
            for z in 0..TILE_ALL_Z_COUNT {
                let z_length = if z % 2 == 0 {
                    TILE_SHORT_WIDTH
                } else {
                    TILE_LONG_WIDTH
                };
                let tile = tile_state.tile_state[x][y][z];
                if tile == 1 {
                    let tile_position = Position::new(x_place, y_place, z_place);
                    commands
                        .spawn()
                        .insert(tile_position.clone())
                        .insert(Tile::new(tile_position.vec3))
                        .insert(Mass::new(1.0))
                        .insert_bundle(PbrBundle {
                            mesh: meshes
                                .add(Mesh::from(shape::Box::new(x_length, y_length, z_length))),
                            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
                            transform: tile_position.into(),
                            ..default()
                        });
                }
                z_place += TILE_WIDTH_AVERAGE;
            }
            y_place += TILE_WIDTH_AVERAGE;
        }
        x_place += TILE_WIDTH_AVERAGE;
    }
}

fn debug_position(query: Query<(&Position, Entity), With<NormalVector>>) {
    for (position, entity) in query.iter() {
        println!("{:?}, {:?}", position, entity);
    }
}

fn update_velocity_by_force(
    time: Res<Time>,
    mut query: Query<(&mut Velocity, &Force, &Mass, Entity)>,
) {
    for (mut velocity, force, mass, entity) in query.iter_mut() {
        // println!("{:?}, {:?}, {:?}", velocity, force, entity);
        velocity.vec3 += time.delta_seconds() * force.vec3 / mass.mass;
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

fn update_object_view_by_position(
    mut query: Query<(&mut ObjectView, &Position, Entity)>,
    mut commands: Commands,
) {
    for (mut object_view, position, entity) in query.iter_mut() {
        // println!("{:?}, {:?}, {:?}", object_view, position, entity);
        object_view.position = position.vec3;
        if object_view.position.length() > 100.0 {
            delete_entity(&mut commands, entity);
        }
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
        let marble_mass = Mass::new(0.5);
        let marble_force = Force::new(0.0, -marble_mass.mass * gravity, 0.0);
        let marble_velocity = Velocity::default();
        commands
            .spawn()
            .insert(Marble)
            .insert(marble_mass)
            .insert(marble_force.clone())
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
    mut stage_rotation: ResMut<StageRotation>,
) {
    for (main_position, max_radius) in query_main.iter() {
        // println!("{:?}, {:?}", main_position, max_radius);
        let rotate_base_vec = Vec3::new(-main_position.vec2.y, 0.0, main_position.vec2.x);
        // println!("{:?}", rotate_base_vec);
        let angular_v = STAGE_ROTATION_ANGULAR_VELOCITY;
        let rotate = if rotate_base_vec != Vec3::ZERO {
            Quat::from_axis_angle(
                rotate_base_vec.normalize(),
                angular_v * (rotate_base_vec.length() / max_radius.size) * time.delta_seconds(),
            )
        } else {
            Quat::IDENTITY
        };

        // next quat
        let quat = (stage_rotation.rotation * rotate).normalize();
        let (mut vec, angle) = quat.to_axis_angle();
        vec.y = 0.0;
        let max_angle = STAGE_ROTATION_MAX_ANGLE;
        stage_rotation.rotation = if vec != Vec3::ZERO {
            Quat::from_axis_angle(
                vec.normalize(),
                if angle < max_angle { angle } else { max_angle },
            )
        } else {
            Quat::IDENTITY
        };
    }
}

fn update_board_transform_by_rotation(mut query: Query<(&mut Transform, &Rotation), With<Board>>) {
    for (mut transform, rotation) in query.iter_mut() {
        transform.rotation = rotation.quat;
        // println!("board: {:?}, {:?}", transform, rotation);
    }
}

// fn update_normal_vector_transform_by_rotation(
//     mut query: Query<(&mut Position, &mut Transform, &NormalVector)>,
//     query_board: Query<&Rotation, With<Board>>,
// ) {
//     for (mut position, mut transform, normal_vector) in query.iter_mut() {
//         if let Ok(rotation) = query_board.get(normal_vector.board_entity) {
//             transform.translation = Vec3::new(0.0, 1.0, 0.0);
//             transform.rotation = Quat::IDENTITY;
//             transform.rotate_around(Vec3::ZERO, rotation.quat);
//             position.vec3 = transform.translation;
//             // println!("{:?}, {:?}, {:?}", position, transform, rotation);
//         }
//     }
// }

fn update_tile_transform_by_rotation(
    mut query_tile: Query<(&mut Position, &mut Transform, &Tile)>,
    stage_rotation: Res<StageRotation>,
) {
    for (mut position, mut transform, tile) in query_tile.iter_mut() {
        transform.translation = tile.base_position;
        transform.rotation = Quat::IDENTITY;
        transform.rotate_around(Vec3::ZERO, stage_rotation.rotation);
        position.vec3 = transform.translation;
        // println!("{:?}, {:?}, {:?}", position, transform, rotation);
    }
}

fn update_tile_origin(
    mut tile_origin: ResMut<TileOrigin>,
    stage_rotation: Res<StageRotation>,
    mut query: Query<&mut Position, With<TileOriginComponent>>,
) {
    let mut transform = Transform::from_translation(tile_origin.base_position);
    transform.rotation = Quat::IDENTITY;
    transform.rotate_around(Vec3::ZERO, stage_rotation.rotation);
    tile_origin.position = transform.translation;
    println!("{:?}", tile_origin.position);

    if let Some(mut origin) = query.iter_mut().next() {
        origin.vec3 = tile_origin.position;
    }
}

fn get_marble_relative_position_from_tile_origin(
    query: Query<&Position, With<Marble>>,
    tile_origin: Res<TileOrigin>,
    stage_rotation: Res<StageRotation>
) {
    for position in query.iter() {
        println!("rela: {:?}", position.vec3 - tile_origin.position);
    }
}

fn delete_entity(commands: &mut Commands, entity: Entity) {
    println!("entity: {:?} deleted!", entity);
    commands.entity(entity).despawn();
}
