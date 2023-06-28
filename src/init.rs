use std::f64::consts::PI;
use specs::prelude::*;
use specs::WorldExt;

use crate::components::*;

pub fn initialize_player(world: &mut World) {
    world
        .create_entity() // Player
        .with(UserControlled)
        .with(Position { x: 255., y: 255.}) // TODO read starting values from map
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

pub fn initialize_world_object(world: &mut World, pos: (i32, i32)) {
    world
        .create_entity()
        .with(Position { x: ((pos.0 << 6) + 32) as f64, y: ((pos.1 << 6) + 32) as f64 })
        .with(Sprite {spritesheet: 1, region: (0, 0)})
        .with(IsEntity)
        .build();
}

pub fn initialize_enemy(world: &mut World, pos: (i32, i32)) {
    world
        .create_entity()
        .with(Position { x: ((pos.0 << 6) + 32) as f64, y: ((pos.1 << 6) + 32) as f64 })
        .with(Sprite {spritesheet: 2, region: (0, 0)})
        .with(IsEntity)
        .with(VelocityMultiplier {
            speed: 1.,
            speed_rot: 0.05,
        })
        .with(VelocityRelative {
            movement_rel: (0., 0.),
            movement_rot: 0.,
        })
        .with(Rotation { r: 0. })
        .with(HasAI {time_to_next_update: 0, path: Vec::new()})
        .build();
}
