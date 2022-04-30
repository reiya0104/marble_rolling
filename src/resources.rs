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

        // 適当に追加
        tile_state[2][2][1] = 1;
        tile_state[2][1][1] = 1;

        // 穴を開けてみた
        tile_state[(TILE_ALL_X_COUNT - 1) / 2][0][(TILE_ALL_Z_COUNT - 1) / 2] = 0;

        Self { tile_state }
    }

    pub(crate) fn get_entry_from_xyz(&self, x: usize, y: usize, z: usize) -> i32 {
        if x < TILE_ALL_X_COUNT && y < TILE_ALL_Y_COUNT && z < TILE_ALL_Z_COUNT {
            self.tile_state[x][y][z]
        } else {
            -1
        }
    }

    pub(crate) fn get_entry_from_ivec3(&self, ivec3: IVec3) -> i32 {
        if Self::check_index(ivec3) {
            self.tile_state[ivec3.x as usize][ivec3.y as usize][ivec3.z as usize]
        } else {
            -1
        }
    }

    pub(crate) fn set_entry_from_ivec3(&mut self, ivec3: IVec3, value: i32) {
        if Self::check_index(ivec3) {
            self.tile_state[ivec3.x as usize][ivec3.y as usize][ivec3.z as usize] = value;
        }
    }

    fn check_index(ivec3: IVec3) -> bool {
        ivec3.x < TILE_ALL_X_COUNT as i32
            && ivec3.y < TILE_ALL_Y_COUNT as i32
            && ivec3.z < TILE_ALL_Z_COUNT as i32
            && ivec3.x >= 0
            && ivec3.y >= 0
            && ivec3.z >= 0
    }
}

#[derive(Default)]
pub(crate) struct TileCollisioned {
    pub(crate) tile_collisioned: Vec<IVec3>,
}

pub(crate) struct TileOrigin {
    pub(crate) base_position: Vec3,
    pub(crate) position: Vec3,
}

impl TileOrigin {
    pub(crate) fn new() -> Self {
        let base_position = base_position_vec3();
        Self {
            base_position,
            position: base_position,
        }
    }
}

pub(crate) struct TileBaseVectorX {
    pub(crate) base_position: Vec3,
    pub(crate) position: Vec3,
    pub(crate) vector: Vec3,
}

impl TileSystem for TileBaseVectorX {
    fn new() -> Self {
        let vector = Vec3::new(1.0, 0.0, 0.0);
        let base_position = base_position_vec3() + vector;
        Self {
            base_position,
            position: base_position,
            vector,
        }
    }

    // これ同じコードなのどうにかしたいです...
    fn update_position_by_rotation(
        &mut self,
        stage_rotation: &StageRotation,
        tile_origin: &TileOrigin,
    ) {
        let mut transform = Transform::from_translation(self.base_position);
        transform.rotation = Quat::IDENTITY;
        transform.rotate_around(Vec3::ZERO, stage_rotation.rotation);
        self.position = transform.translation;
        self.vector = (self.position - tile_origin.position).normalize();
    }
}

pub(crate) struct TileBaseVectorY {
    pub(crate) base_position: Vec3,
    pub(crate) position: Vec3,
    pub(crate) vector: Vec3,
}

impl TileSystem for TileBaseVectorY {
    fn new() -> Self {
        let vector = Vec3::new(0.0, 1.0, 0.0);
        let base_position = base_position_vec3() + vector;
        Self {
            base_position,
            position: base_position,
            vector,
        }
    }

    fn update_position_by_rotation(
        &mut self,
        stage_rotation: &StageRotation,
        tile_origin: &TileOrigin,
    ) {
        let mut transform = Transform::from_translation(self.base_position);
        transform.rotation = Quat::IDENTITY;
        transform.rotate_around(Vec3::ZERO, stage_rotation.rotation);
        self.position = transform.translation;
        self.vector = (self.position - tile_origin.position).normalize();
    }
}

pub(crate) struct TileBaseVectorZ {
    pub(crate) base_position: Vec3,
    pub(crate) position: Vec3,
    pub(crate) vector: Vec3,
}

impl TileSystem for TileBaseVectorZ {
    fn new() -> Self {
        let vector = Vec3::new(0.0, 0.0, 1.0);
        let base_position = base_position_vec3() + vector;
        Self {
            base_position,
            position: base_position,
            vector,
        }
    }

    fn update_position_by_rotation(
        &mut self,
        stage_rotation: &StageRotation,
        tile_origin: &TileOrigin,
    ) {
        let mut transform = Transform::from_translation(self.base_position);
        transform.rotation = Quat::IDENTITY;
        transform.rotate_around(Vec3::ZERO, stage_rotation.rotation);
        self.position = transform.translation;
        self.vector = (self.position - tile_origin.position).normalize();
    }
}

fn base_position_vec3() -> Vec3 {
    Vec3::new(-TILE_ALL_X_WIDTH / 2.0, 0.0, -TILE_ALL_Z_WIDTH / 2.0)
}

pub(crate) trait TileSystem {
    fn new() -> Self;
    fn update_position_by_rotation(
        &mut self,
        stage_rotation: &StageRotation,
        tile_origin: &TileOrigin,
    );
}
