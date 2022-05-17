use std::f64::consts::{PI, TAU};
use specs::prelude::*;
use specs::WorldExt;

use crate::components::*;

pub fn initialize_player(world: &mut World) {
    world
        .create_entity() // Player
        .with(UserControlled)
        .with(Position { x: 255., y: 255. }) // TODO read starting values from map
        .with(Rotation { r: PI })
        .with(VelocityMultiplier {
            speed: 2.,
            speed_rot: 0.1,
        })
        .with(VelocityRelative {
            movement_rel: (0., 0.),
            movement_rot: 0.,
        })
        .with(IsPlayer)
        .build();
}

pub fn initialize_world_object(world: &mut World, pos: (f64, f64)) {
    world
        .create_entity()
        .with(Position { x: pos.0, y: pos.1 })
        .with(Sprite {spritesheet: 1, region: (0, 0)})
        .with(IsEntity)
        .with(VelocityRelative {
            movement_rel: (1., 0.),
            movement_rot: 0.,
        })
        .with(Rotation { r: PI })
        .build();
}
