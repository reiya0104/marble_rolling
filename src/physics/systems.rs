use crate::{component::*, consts::*, resources::*};
use bevy::prelude::*;

// 衝突判定

// 点の座標を含むタイルの位置
fn get_coordinates(vec3: Vec3) -> IVec3 {
    let s = TILE_SHORT_WIDTH;
    let l = TILE_LONG_WIDTH;
    let v1 = (vec3 + Vec3::new(0.0, s, 0.0)) / (s + l);
    let v2 = v1 - Vec3::splat(s / (s + l)) - v1.floor();
    2 * v1.floor().as_ivec3() + v2.floor().as_ivec3() + IVec3::splat(1)
}

// (i_minus, j_minus, k_minus),
// (i_plus,  j_plus,  k_plus),
fn get_tile_inf_sup(vec3: Vec3, r: f32) -> (IVec3, IVec3) {
    (
        get_coordinates(vec3 - Vec3::splat(r)),
        get_coordinates(vec3 + Vec3::splat(r)),
    )
}

fn get_tiles_include_marble(
    tile_state: &TileState,
    marble_position: Vec3,
    marble_radius: f32,
) -> Vec<IVec3> {
    let mut vec = Vec::new();
    let (inf, sup) = get_tile_inf_sup(marble_position, marble_radius);
    let (i_m, j_m, k_m): (i32, i32, i32) = From::from(inf);
    let (i_p, j_p, k_p): (i32, i32, i32) = From::from(sup);

    for i in i_m..=i_p {
        for j in j_m..=j_p {
            for k in k_m..=k_p {
                if tile_state.get_entry_from_ivec3(IVec3::new(i, j, k)) == 1 {
                    vec.push(IVec3::new(i, j, k));
                }
            }
        }
    }

    vec
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

fn get_distance(marble_position: Vec3, marble_radius: f32, tile_position: IVec3) -> f32 {
    let s = TILE_SHORT_WIDTH;
    let l = TILE_LONG_WIDTH;
    let (a, b, c): (f32, f32, f32) = From::from(marble_position);
    let (i, j, k): (i32, i32, i32) = From::from(tile_position);

    let x_m = ((i as f32 + 1.0) / 2.0).floor() * s + (i as f32 / 2.0).floor() * l;
    let x_p = ((i as f32 + 2.0) / 2.0).floor() * s + ((i as f32 + 1.0) / 2.0).floor() * l;
    let y_m = ((j as f32 - 1.0) / 2.0).floor() * s + (j as f32 / 2.0).floor() * l;
    let y_p = (j as f32 / 2.0).floor() * s + ((j as f32 + 1.0) / 2.0).floor() * l;
    let z_m = ((k as f32 + 1.0) / 2.0).floor() * s + (k as f32 / 2.0).floor() * l;
    let z_p = ((k as f32 + 2.0) / 2.0).floor() * s + ((k as f32 + 1.0) / 2.0).floor() * l;

    let d_x_m = if -(a - x_m) >= 0.0 { -(a - x_m) } else { 0.0 };
    let d_x_p = if a - x_p >= 0.0 { a - x_p } else { 0.0 };
    let d_y_m = if -(b - y_m) >= 0.0 { -(b - y_m) } else { 0.0 };
    let d_y_p = if b - y_p >= 0.0 { b - y_p } else { 0.0 };
    let d_z_m = if -(c - z_m) >= 0.0 { -(c - z_m) } else { 0.0 };
    let d_z_p = if c - z_p >= 0.0 { c - z_p } else { 0.0 };

    (d_x_m.powi(2) + d_x_p.powi(2) + d_y_m.powi(2) + d_y_p.powi(2) + d_z_m.powi(2) + d_z_p.powi(2))
        .sqrt()
        - marble_radius
}

enum CollsionCheckType {
    plane,
    edge,
    vertex,
}

// 衝突判定
// タイル座標系における marble_position が衝突しているかを判断
// 衝突していたら，tile_collisioned に追加
fn is_collisioned(
    tile_collisioned: &mut TileCollisioned,
    marble_position: &Vec3,
    marble_radius: f32,
    // check_type: &CollsionCheckType,
    tile_state: &TileState,
) {
    let tiles = get_tiles_include_marble(tile_state, *marble_position, marble_radius);
    for tile in tiles {
        let d = get_distance(*marble_position, marble_radius, tile);
        println!("d = {:?}", d);
        if d <= 0.0 {
            println!("{:?}, distance = {:?}", tile, d);
            tile_collisioned.tile_collisioned.push(tile);
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

        tile_collisioned.tile_collisioned.clear();

        is_collisioned(
            &mut tile_collisioned,
            &relative_position,
            radius.radius,
            &tile_state,
        );
    }
}
