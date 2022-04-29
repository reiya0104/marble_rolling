use crate::consts::*;
use bevy::prelude::*;

#[derive(Debug)]
pub(crate) struct MarbleCount {
    pub(crate) count: i32,
}

#[derive(Default)]
pub(crate) struct StageRotation {
    pub(crate) rotation: Quat,
}

pub(crate) struct TileState {
    pub(crate) tile_state: [[[i32; TILE_ALL_Z_COUNT]; TILE_ALL_Y_COUNT]; TILE_ALL_X_COUNT],
}

impl TileState {
    pub(crate) fn new() -> Self {
        let mut tile_state = [[[0; TILE_ALL_Z_COUNT]; TILE_ALL_Y_COUNT]; TILE_ALL_X_COUNT];

        for x in 0..TILE_ALL_X_COUNT {
            for z in 0..TILE_ALL_Z_COUNT {
                tile_state[x][0][z] = 1;

                if x == 0 || x == TILE_ALL_X_COUNT - 1 || z == 0 {
                    tile_state[x][1][z] = 1;
                    if x % 2 == 0 && z % 2 == 0 {
                        tile_state[x][2][z] = 1;
                        tile_state[x][4][z] = 1;
                    } else {
                        tile_state[x][3][z] = 1;
                    }
                }
            }
        }
        // 原点を確認
        tile_state[0][1][0] = 0;
        tile_state[0][1][1] = 0;
        tile_state[1][1][0] = 0;
        tile_state[0][2][0] = 0;

        // 穴を開けてみた
        tile_state[(TILE_ALL_X_COUNT - 1) / 2][0][(TILE_ALL_Z_COUNT - 1) / 2] = 0;

        Self { tile_state }
    }
}

pub(crate) struct TileOrigin {
    pub(crate) base_position: Vec3,
    pub(crate) position: Vec3,
}

impl TileOrigin {
    pub(crate) fn new() -> Self {
        let base_position = Vec3::new(-TILE_ALL_X_WIDTH / 2.0, 0.0, -TILE_ALL_Z_WIDTH / 2.0);
        Self {
            base_position,
            position: base_position,
        }
    }
}
