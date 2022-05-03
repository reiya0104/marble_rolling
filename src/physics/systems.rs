use crate::{component::*, consts::*, resources::*};
use bevy::prelude::*;

// 衝突判定

fn get_coordinate(a: f32) -> usize {
    let s = TILE_SHORT_WIDTH;
    let l = TILE_LONG_WIDTH;
    let k = (a / (s + l)).floor();
    if a < k + s {
        k as usize
    } else {
        k as usize + 1
    }
}

fn get_coordinates(vec3: Vec3) -> IVec3 {
    let s = TILE_SHORT_WIDTH;
    let l = TILE_LONG_WIDTH;
    let v1 = (vec3 / (s + l)).floor();
    let v2 = IVec3::new(
        if vec3.x < (s + l) * v1.x + s { 0 } else { 1 },
        if vec3.y < (s + l) * v1.y + l { 1 } else { 2 },
        if vec3.z < (s + l) * v1.z + s { 0 } else { 1 },
    );
    2 * v1.as_ivec3() + v2
}

fn get_relative_position(
    vec_pos: &Vec3,
    tile_origin: &TileOrigin,
    vec_x: &TileBaseVectorX,
    vec_y: &TileBaseVectorY,
    vec_z: &TileBaseVectorZ,
) -> Vec3 {
    let mat_p = Mat3::from_cols(vec_x.vector, vec_y.vector, vec_z.vector);
    mat_p.inverse() * (*vec_pos - tile_origin.position)
}

enum CollsionCheckType {
    plane,
    edge,
    vertex,
}

// 衝突判定
// タイル上の座標 ivec3 が衝突しているかを判断
// 衝突していたら，tile_collisioned に追加
fn is_collisioned(
    tile_collisioned: &mut TileCollisioned,
    ivec3: &IVec3,
    check_type: &CollsionCheckType,
    tile_state: &TileState,
) {
    if tile_state.get_entry_from_ivec3(*ivec3) == 1 {
        match check_type {
            CollsionCheckType::plane => tile_collisioned.tile_collisioned.push(*ivec3),
            CollsionCheckType::edge => todo!(),
            CollsionCheckType::vertex => todo!(),
        }
    }
}


pub(crate) fn collision_detection(
    query_marble: Query<(&Position, &Radius), With<Marble>>,
    tile_origin: Res<TileOrigin>,
    vec_x: Res<TileBaseVectorX>,
    vec_y: Res<TileBaseVectorY>,
    vec_z: Res<TileBaseVectorZ>,
    tile_state: Res<TileState>,
    mut tile_collisioned: ResMut<TileCollisioned>,
) {
    for (position, radius) in query_marble.iter() {
        let relative_position =
            get_relative_position(&position.vec3, &tile_origin, &vec_x, &vec_y, &vec_z);
        let radius_x_vec = Vec3::new(radius.radius, 0.0, 0.0);
        let radius_y_vec = Vec3::new(0.0, radius.radius, 0.0);
        let radius_z_vec = Vec3::new(0.0, 0.0, radius.radius);

        let vec_x_d = get_coordinates(relative_position - radius_x_vec);
        let vec_x_u = get_coordinates(relative_position + radius_x_vec);

        let vec_y_d = get_coordinates(relative_position - radius_y_vec);
        let vec_y_u = get_coordinates(relative_position + radius_y_vec);

        let vec_z_d = get_coordinates(relative_position - radius_z_vec);
        let vec_z_u = get_coordinates(relative_position + radius_z_vec);

        // let vertex = CollsionCheckType::vertex;
        // let edge = CollsionCheckType::edge;
        // let plane = CollsionCheckType::plane;
        let vec_iter = vec![
            (vec_x_d, CollsionCheckType::plane),
            (vec_x_u, CollsionCheckType::plane),
            (vec_y_d, CollsionCheckType::plane),
            (vec_y_u, CollsionCheckType::plane),
            (vec_z_d, CollsionCheckType::plane),
            (vec_z_u, CollsionCheckType::plane),
        ];
        // println!("{:?}, {:?}", position.vec3, tile_origin.position);
        // println!("{:?}, {:?}", relative_position, relative_position + radius_y_vec);
        // let detection = vec_iter.map(|v| (v, tile_state.get_entry_from_ivec3(v)));
        // let collisioned = vec_iter.iter().filter(|&v| tile_state.get_entry_from_ivec3(*v) == 1).map(|v| *v);
        // tile_collisioned.tile_collisioned = collisioned.collect::<Vec<_>>();
        // println!("{:?}", detection);
        tile_collisioned.tile_collisioned.clear();
        for (v, check_type) in vec_iter {
            is_collisioned(&mut tile_collisioned, &v, &check_type, &tile_state);
        }
    }
}
