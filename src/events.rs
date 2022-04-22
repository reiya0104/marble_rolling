use bevy::prelude::*;

pub(crate) struct MarbleCreatedEvent {
    pub(crate) position: Vec3,
    pub(crate) entity: Entity,
}

pub(crate) struct CollisionEvent {
    pub(crate) position: Vec3,
    /// entity of board 
    /// 法線ベクトル(normal vector)の entity にするか迷いましたが，
    /// 天板(board) にある Rotation と PreviousRotation にアクセスしたいので，
    /// board_entity にしました．
    pub(crate) board_entity: Entity,
}
