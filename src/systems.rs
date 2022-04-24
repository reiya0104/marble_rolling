use bevy::prelude::*;

use crate::{
    component::*,
    events::{CollisionEvent, MarbleCreatedEvent},
    resources::MarbleCount,
};

pub(crate) fn create_leg(
    mut commands: Commands,
    mut event_reader: EventReader<MarbleCreatedEvent>,
    mut count: ResMut<MarbleCount>,
    query: Query<&Position>,
) {
    for e in event_reader.iter() {
        println!(
            "marble created! {:?}, position: {:?}, entity: {:?}",
            count.count, e.position, e.entity
        );
        count.count += 1;
        // commands.spawn()
        //     .insert(Leg::new(e.entity));
    }
}

pub(crate) fn update_legs(
    mut commands: Commands,
    mut query_leg: Query<
        (&Leg, &mut Transform, &mut Position, Entity),
        (Without<Marble>, Without<NormalVector>),
    >,
    // mut query_leg_position: Query<&mut Position, With<Leg>>,
    query_marble: Query<&Position, With<Marble>>,
    query_normal_vector: Query<(&Position, &NormalVector)>,
    mut event_writer: EventWriter<CollisionEvent>,
) {
    for (leg, mut transform, mut leg_position, entity) in query_leg.iter_mut() {
        if let Ok((normal_vector_position, normal_vector)) =
            query_normal_vector.get(leg.normal_vector_entity)
        {
            if let Ok(marble_position) = query_marble.get(leg.marble_entity) {
                // println!(
                //     "{:?}, normal_vector: {:?}, marble: {:?}",
                //     leg, normal_vector_position, marble_position
                // );

                let distance = normal_vector_position.vec3.dot(marble_position.vec3);
                let next_leg_position =
                    marble_position.vec3 - distance * normal_vector_position.vec3;
                // println!("distance: {:?}", distance);
                if distance.abs() <= 0.05 + 0.25 {
                    event_writer.send(CollisionEvent {
                        position: next_leg_position,
                        distance,
                        board_entity: normal_vector.board_entity,
                        marble_entity: leg.marble_entity,
                        normal_vector_entity: leg.normal_vector_entity,
                    });
                }
                leg_position.vec3 = next_leg_position;
                transform.translation = next_leg_position;
            } else {
                // 対象 marble がなくなったので leg も削除する
                commands.entity(entity).despawn();
                println!("leg deleted!");
            }
        } else {
            // 対象 marble がなくなったので leg も削除する
            commands.entity(entity).despawn();
            println!("leg deleted!");
        }
    }
}

pub(crate) fn collision_board_and_marble(
    time: Res<Time>,
    mut event_reader: EventReader<CollisionEvent>,
    query_board: Query<(&Rotation, &PreviousRotation, &Mass), With<Board>>,
    mut query_marble: Query<
        (&mut Velocity, &mut Position, &Mass),
        (With<Marble>, Without<NormalVector>),
    >,
    query_normal_vector: Query<&Position, (With<NormalVector>, Without<Marble>)>,
) {
    for e in event_reader.iter() {
        println!("collisioned!");
        if let Ok((rotation, pre_rotation, board_mass)) = query_board.get(e.board_entity) {
            let quat = pre_rotation.quat * rotation.quat.inverse();
            let position_quat = Quat::from_vec4(e.position.extend(0.0));
            let pre_position = (quat * position_quat * quat.conjugate()).xyz();
            let velocity = e.position - pre_position;
            // println!(
            //     "now_position: {:?}, pre_position: {:?}, velocity: {:?}, {:?}",
            //     e.position, pre_position, velocity, velocity.length()
            // );
            println!("velocity: {:?}, {:?}", velocity, velocity.length());
            println!("distance: {:?}", e.distance);
            if let Ok(normal_vector_position) = query_normal_vector.get(e.normal_vector_entity) {
                if let Ok((mut marble_velocity, mut marble_position, marble_mass)) =
                    query_marble.get_mut(e.marble_entity)
                {
                    if velocity.length() >= 0.0 {
                        // if velocity.length() < 0.0001 {
                        let marble_v = calculate_velocity_after_collision_if_board_isstoping(
                            normal_vector_position.vec3,
                            marble_velocity.vec3,
                        );
                        println!(
                            "pre_marble_v: {:?}, pre_marble_pos: {:?}",
                            marble_velocity, marble_position
                        );
                        // marble_velocity.vec3 = marble_v;
                        if marble_v.length() > 0.0 {
                            marble_velocity.vec3 = marble_v;
                            marble_position.vec3 += 2.0 * marble_v * time.delta_seconds();
                        } else {
                            marble_velocity.vec3 = Vec3::ZERO;
                            marble_position.vec3 += 2.0 * marble_v * time.delta_seconds();
                        }
                        println!(
                            "marble_v: {:?}, marble_pos: {:?}",
                            marble_velocity, marble_position
                        );
                    }
                }
            }
        }
    }
}

// ビー玉 と 静止した滑らかな天板 との衝突
// 天板の単位法線ベクトル n: Vec3
// ビー玉の速度 v0: Vec3
// 反発係数 e: f32
// 衝突後のビー玉の速度 v1: Vec3
// このとき，
// v1 = v0 - (1+e)(v0・n) n
// が成り立つ．
fn calculate_velocity_after_collision(
    mass1: f32,
    velocity1: Vec3,
    mass2: f32,
    velocity2: Vec3,
) -> (Vec3, Vec3) {
    let coefficient_of_restitution = 1.0;

    // 長いので省略
    let e = coefficient_of_restitution;

    // 衝突後の速度
    let v1 = (mass1 - e * mass2) / (mass1 + mass2) * velocity1
        + mass2 * (1.0 + e) / (mass1 + mass2) * velocity2;
    let v2 = mass1 * (1.0 + e) / (mass1 + mass2) * velocity1
        + (mass2 - e * mass1) / (mass1 + mass2) * velocity2;

    return (v1, v2);
}

// ビー玉 と 静止した滑らかな天板 との衝突
// 天板の単位法線ベクトル n: Vec3
// ビー玉の速度 v0: Vec3
// 反発係数 e: f32
// 衝突後のビー玉の速度 v1: Vec3
// このとき，
// v1 = v0 - (1+e)(v0・n) n
// が成り立つ．
fn calculate_velocity_after_collision_if_board_isstoping(
    normal_vector: Vec3,
    marble_velocity: Vec3,
) -> Vec3 {
    let coefficient_of_restitution = 1.0;

    // 長いので省略
    let e = &coefficient_of_restitution;
    let n = &normal_vector;
    let v0 = &marble_velocity;

    // 衝突後の速度
    let v1 = (*v0) - ((1.0 + e) * (v0.dot(*n))) * (*n);

    return v1;
}
