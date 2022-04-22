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
pub(crate) struct Acceleration {
    pub(crate) vec3: Vec3,
}

impl Acceleration {
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

#[derive(Debug, Default, Clone, Component)]
pub(crate) struct PreviousRotation {
    pub(crate) quat: Quat,
}

impl PreviousRotation {
    pub(crate) fn from_quat(quat: Quat) -> Self {
        Self { quat }
    }
}

#[derive(Debug, Default, Clone, Component)]
pub(crate) struct Rotation {
    pub(crate) quat: Quat,
}

impl Rotation {
    pub(crate) fn from_quat(quat: Quat) -> Self {
        Self { quat }
    }
}

// 垂線の足
// (天板の)法線ベクトルとビー玉に依存する
#[derive(Debug, Clone, Component)]
pub(crate) struct Leg {
    pub(crate) normal_vector_entity: Entity,
    pub(crate) marble_entity: Entity,
}

impl Leg {
    pub(crate) fn new(normal_vector_entity: Entity, marble_entity: Entity) -> Self {
        Self {
            normal_vector_entity,
            marble_entity,
        }
    }
}

// 衝突用のコンポーネント
#[derive(Debug, Component)]
struct Collision;

// 天板用のコンポーネント
#[derive(Debug, Component)]
pub(crate) struct NormalVector {
    pub(crate) board_entity: Entity
}

impl NormalVector {
    pub(crate) fn new(entity: Entity) -> Self {
        Self {
            board_entity: entity
        }
    }
}
// カメラ用のコンポーネン
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
struct Tile;

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
