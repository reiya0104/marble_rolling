use bevy::prelude::*;

// オブジェクト描画用コンポーネント
#[derive(Debug, Default, Clone, Component)]
pub(crate) struct ObjectView {
    pub(crate) position: Vec3,
}

impl ObjectView {
    pub(crate) fn new(position_vec3: Vec3) -> Self {
        Self {
            position: position_vec3,
        }
    }

    pub(crate) fn from_position(position: Position) -> Self {
        Self {
            position: position.vec3,
        }
    }
}

// 位置情報などのコンポーネント
#[derive(Debug, Default, Clone, Component)]
pub(crate) struct Force {
    pub(crate) vec3: Vec3,
}

impl Force {
    pub(crate) fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            vec3: Vec3::new(x, y, z),
        }
    }
}

#[derive(Debug, Default, Clone, Component)]
pub(crate) struct Velocity {
    pub(crate) vec3: Vec3,
}

impl Velocity {
    pub(crate) fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            vec3: Vec3::new(x, y, z),
        }
    }

    pub(crate) fn from_vec3(vec3: Vec3) -> Self {
        Self { vec3 }
    }
}

#[derive(Debug, Default, Clone, Component)]
pub(crate) struct Position {
    pub(crate) vec3: Vec3,
}

impl Position {
    pub(crate) fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            vec3: Vec3::new(x, y, z),
        }
    }

    pub(crate) fn from_vec3(vec3: Vec3) -> Self {
        Self { vec3 }
    }
}

impl From<Position> for Transform {
    fn from(position: Position) -> Self {
        Transform::from_xyz(position.vec3.x, position.vec3.y, position.vec3.z)
    }
}

// 質量
#[derive(Debug, Default, Clone, Component)]
pub(crate) struct Mass {
    pub(crate) mass: f32,
}

impl Mass {
    pub(crate) fn new(mass: f32) -> Self {
        Self { mass }
    }
}

// 半径
#[derive(Debug, Default, Clone, Component)]
pub(crate) struct Radius {
    pub(crate) radius: f32,
}

impl Radius {
    pub(crate) fn new(radius: f32) -> Self {
        Self { radius }
    }
}

#[derive(Debug, Default, Clone, Component)]
pub(crate) struct Rotation {
    pub(crate) quat: Quat,
}

impl Rotation {
    // pub(crate) fn new(x: f32, y: f32, z: f32) -> Self {
    //     Self {
    //         vec3: Vec3::new(x, y, z),
    //     }
    // }

    pub(crate) fn from_quat(quat: Quat) -> Self {
        Self { quat }
    }
}

#[derive(Debug, Default, Clone, Component)]
pub(crate) struct PreviousRotation {
    pub(crate) quat: Quat,
}

impl PreviousRotation {
    pub(crate) fn from_quat(quat: Quat) -> Self {
        Self { quat }
    }
}

// 垂線の足
#[derive(Debug, Component)]
pub(crate) struct Leg {
    pub(crate) position: Vec3,
    pub(crate) marble_entity: Entity,
    pub(crate) normal_vector_entity: Entity,
}

impl Leg {
    pub(crate) fn new(position: Vec3, marble_entity: Entity, normal_vector_entity: Entity) -> Self {
        Self {
            position,
            marble_entity,
            normal_vector_entity,
        }
    }
}

// 衝突用のコンポーネント
#[derive(Debug, Component)]
struct Collision;

// 天板用のコンポーネント
#[derive(Debug, Component)]
pub(crate) struct NormalVector {
    pub(crate) board_entity: Entity,
}

impl NormalVector {
    pub(crate) fn new(entity: Entity) -> Self {
        Self {
            board_entity: entity,
        }
    }
}
// カメラ用のコンポーネント
#[derive(Debug, Component)]
struct LookAt;

#[derive(Debug, Component)]
struct Audio;

// Entity 用コンポーネント
#[derive(Debug, Component)]
pub(crate) struct Marble;

#[derive(Debug, Component)]
struct Actor;

#[derive(Debug, Component)]
pub(crate) struct Board;

#[derive(Debug, Component)]
pub(crate) struct Tile {
    pub(crate) base_position: Vec3,
    pub(crate) index: IVec3,
}

impl Tile {
    pub(crate) fn new(base_position: Vec3, index: IVec3) -> Self {
        Self {
            base_position,
            index,
        }
    }
}

#[derive(Debug, Component)]
pub(crate) struct TileOriginComponent;

#[derive(Debug, Component)]
pub(crate) struct Light;

#[derive(Debug, Component)]
pub(crate) struct Camera;

#[derive(Debug, Component)]
struct Start;

#[derive(Debug, Component)]
struct Goal;

#[derive(Debug, Component)]
struct Item;

#[derive(Debug, Component)]
struct Enemy;

#[derive(Debug, Component)]
pub(crate) struct Apple;
