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
        println!("\ncollisioned!");
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
                        let marble_v = calculate_velocity_after_marble_and_board_collision(
                            normal_vector_position.vec3,
                            marble_velocity.vec3,
                            velocity
                        );
                        // let (marble_v, board_v) = calculate_velocity_after_collision(
                        //     normal_vector_position.vec3,
                        //     marble_mass.mass,
                        //     marble_velocity.vec3,
                        //     board_mass.mass,
                        //     velocity
                        // );

                        // println!("marble_v: {:?}, board_v: {:?}", marble_v, board_v);
                        println!(
                            "pre_marble_v: {:?}, pre_marble_pos: {:?}",
                            marble_velocity, marble_position
                        );
                        // marble_velocity.vec3 = marble_v;
                        marble_velocity.vec3 = marble_v;
                        marble_position.vec3 += 2.0 * marble_v * time.delta_seconds();

                        println!(
                            "marble_v: {:?}, marble_pos: {:?}, dis: {:?}",
                            marble_velocity, marble_position, marble_position.vec3.dot(normal_vector_position.vec3)
                        );

                        let distance = marble_position.vec3.dot(normal_vector_position.vec3);
                        
                        // めり込んでしまった場合
                        // めり込まない場所まで position を動かしてあげる
                        let min_distance = 0.25 + 0.05;
                        if distance.abs() < min_distance {
                            let add_distance = ((min_distance - distance.abs()) * 1000000.0).ceil() / 1000000.0;
                            println!("add: {:?}", add_distance);
                            marble_position.vec3 += add_distance * normal_vector_position.vec3;
                        }
                        println!(
                            "marble_v: {:?}, marble_pos: {:?}, dis: {:?}",
                            marble_velocity, marble_position, marble_position.vec3.dot(normal_vector_position.vec3)
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
    normal_vector: Vec3,
    mass1: f32,
    velocity1: Vec3,
    mass2: f32,
    velocity2: Vec3,
) -> (Vec3, Vec3) {
    let coefficient_of_restitution = 0.8;

    // 長いので省略
    let e = coefficient_of_restitution;
    let n = normal_vector;
    let v1 = velocity1;
    let v2 = velocity2;
    let m1 = mass1;
    let m2 = mass2;

    //  撃力 impulse (単位が N⋅s = J = ジュールなので j)
    let j = (1.0 + e) * (m1 * m2) / (m1 + m2) * ((v2 - v1) * n);
    println!("\ninput: e = {:?}, n = {:?}, \nv1 = {:?}, m1 = {:?}, v2 = {:?}, m2 = {:?}", e, n, v1, m1, v2, m2);

    println!("j = {:?}", j);

    let v1_n = v1 + j / m1;
    let v2_n = v2 + j / m2;
    println!("v1_n = {:?}, v2_n = {:?}", v1_n, v2_n);

    // 衝突後の速度
    (
        v1_n + (v1 - (v1 * n) * n) + v2_n,
        v2_n + (v2 - (v2 * n) * n),
    )
}

/// ビー玉 と 滑らかな天板 との衝突
/// [aaa](./events.rs)
fn calculate_velocity_after_marble_and_board_collision(
    normal_vector: Vec3,
    marble_velocity: Vec3,
    board_velocity: Vec3
) -> Vec3 {
    let coefficient_of_restitution = 0.7;

    // 長いので省略
    let e = coefficient_of_restitution;
    let n = normal_vector;
    let v_m = marble_velocity;
    let v_b = board_velocity;
    println!("\ninput: e = {:?}, n = {:?}, \nv_m = {:?}, {:?}, \nv_b = {:?}, {:?}", e, n, v_m, v_m.length(), v_b, v_b.length());

    // 衝突後の速度
    let v = v_m - (1.0 + e) * ((v_m - v_b).dot(n)) * n;
    // let v = v_m - (1.0 + e) * ((v_m).dot(n)) * n;
    // (*v0) - ((1.0 + e) * (v0.dot(*n))) * (*n);

    println!("output: v = {:?}, length: {:?}\n", v, v.length());
    return v;
}
