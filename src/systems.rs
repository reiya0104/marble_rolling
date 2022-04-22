use bevy::prelude::*;

use crate::{
    component::{Leg, Marble, NormalVector, Position},
    events::{CollisionEvent, MarbleCreatedEvent},
    resouces::MarbleCount,
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
    query_normal_vector: Query<&Position, With<NormalVector>>,
    mut event_writer: EventWriter<CollisionEvent>,
) {
    for (leg, mut transform, mut leg_position, entity) in query_leg.iter_mut() {
        if let Ok(normal_vector_position) = query_normal_vector.get(leg.normal_vector_entity) {
            if let Ok(marble_position) = query_marble.get(leg.marble_entity) {
                // println!(
                //     "{:?}, normal_vector: {:?}, marble: {:?}",
                //     leg, normal_vector_position, marble_position
                // );

                let distance = normal_vector_position.vec3.dot(marble_position.vec3);
                println!("distance: {:?}", distance);
                if distance.abs() <= 0.05 + 0.25 {
                    event_writer.send(CollisionEvent {});
                }
                let next_vector = marble_position.vec3 - distance * normal_vector_position.vec3;
                leg_position.vec3 = next_vector;
                transform.translation = next_vector;
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

pub(crate) fn collision_board_and_marble(mut event_reader: EventReader<CollisionEvent>) {
    for e in event_reader.iter() {
        println!("collisioned!");
    }
}
