use specs::prelude::*;

use crate::components::*;

pub struct Input;

impl<'a> System<'a> for Input {
    type SystemData = (
        Read<'a, PlayerInput>,
        ReadStorage<'a, UserControlled>,
        ReadStorage<'a, VelocityMultiplier>,
        WriteStorage<'a, VelocityRelative>,
    );

    fn run(&mut self, (p_in, usr_ctr, vel_mul, mut vel_rel): Self::SystemData) {
        for (_, vel_mul, vel_rel) in (&usr_ctr, &vel_mul, &mut vel_rel).join() {
            for element in &p_in.0 {
                match element {
                    PlayerInputCommand::Forward(val) => {
                        vel_rel.movement_rel.0 += vel_mul.speed * val
                    }
                    PlayerInputCommand::Sidewards(val) => {
                        vel_rel.movement_rel.1 += vel_mul.speed * val
                    }
                    PlayerInputCommand::Rotate(val) => {
                        vel_rel.movement_rot += vel_mul.speed_rot * val
                    }
                }
            }
        }
    }
}
