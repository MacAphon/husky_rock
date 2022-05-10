use specs::prelude::*;
use specs::WorldExt;

use crate::components::*;

pub fn initialize_player(world: &mut World) {
    world
        .create_entity() // Player
        .with(UserControlled)
        .with(Position { x: 255., y: 255. }) // TODO read starting values from map
        .with(Rotation { r: 0. })
        .with(VelocityMultiplier {
            speed: 2.,
            speed_rot: 0.1,
        })
        .with(VelocityRelative {
            movement_rel: (0., 0.),
            movement_rot: 0.,
        })
        .with(Renderable)
        .with(IsPlayer)
        .build();
}
