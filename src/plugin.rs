use bevy::prelude::*;

#[derive(Event)]
pub struct ClickEvent {
    position: Vec3,
    collision_entity: Entity,
}
