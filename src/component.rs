use bevy::prelude::*;

// オブジェクト描画用コンポーネント
#[derive(Debug, Component)]
struct ObjectView;

// 位置情報などのコンポーネント
#[derive(Debug, Component)]
struct Acceleration;

#[derive(Debug, Component)]
struct Velocity;

#[derive(Debug, Default, Clone, Component)]
pub(crate) struct Position {
    pub(crate) vec3: Vec3,
}

impl Position {
    pub(crate) fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            vec3: Vec3::new(x, y, z)
        }
    }
}

impl From<Position> for Transform {
    fn from(position: Position) -> Self {
        Transform::from_xyz(position.vec3.x, position.vec3.y, position.vec3.z)
    }
}

// 衝突用のコンポーネント
#[derive(Debug, Component)]
struct Collision;

// 天板用のコンポーネント
#[derive(Debug, Component)]
struct NormalVector;

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
