use bevy::prelude::*;

use crate::{
    component::{Board, Leg, Marble, NormalVector, Position, PreviousRotation, Rotation},
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
                if distance.abs() <= 0.05 + 0.25 {
                    event_writer.send(CollisionEvent {
                        position: next_leg_position,
                        board_entity: normal_vector.board_entity,
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
    mut event_reader: EventReader<CollisionEvent>,
    query_board: Query<(&Rotation, &PreviousRotation), With<Board>>,
) {
    for e in event_reader.iter() {
        println!("collisioned!");
        if let Ok((rotation, pre_rotation)) = query_board.get(e.board_entity) {
            let quat = pre_rotation.quat * rotation.quat.inverse();
            let position_quat = Quat::from_vec4(e.position.extend(0.0));
            let pre_position = (quat * position_quat * quat.conjugate()).xyz();
            let velocity = e.position - pre_position;
            // println!(
            //     "now_position: {:?}, pre_position: {:?}, velocity: {:?}, {:?}",
            //     e.position, pre_position, velocity, velocity.length()
            // );
            println!("velocity: {:?}, {:?}", velocity, velocity.length());
        }
    }
}
