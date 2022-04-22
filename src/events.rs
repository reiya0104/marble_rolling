use bevy::prelude::*;

pub(crate) struct MarbleCreatedEvent {
    pub(crate) position: Vec3,
    pub(crate) entity: Entity,
}
