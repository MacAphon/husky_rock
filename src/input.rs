use specs::prelude::*;

use crate::components::*;
use super::PlayerInputCommand;

pub struct Input;

impl<'a> System<'a> for Input {
    type SystemData = (
        Read<'a, Option<Vec<(PlayerInputCommand)>>>,
        ReadStorage<'a, UserControlled>,
        ReadStorage<'a, VelocityMultiplier>,
        WriteStorage<'a, VelocityRelative>,
    );

    fn run(&mut self, (move_cmd, usr_ctr, vel_mul, mut vel_rel): Self::SystemData) {
        for (_, vel_mul, vel_rel) in (&usr_ctr, &vel_mul, &mut vel_rel).join() {
            for el in move_cmd.iter() {
                for element in el.iter() {
                    match element {
                        PlayerInputCommand::Forward(val) => vel_rel.movement_rel.0 += vel_mul.speed * val,
                        PlayerInputCommand::Sidewards(val) => vel_rel.movement_rel.1 += vel_mul.speed * val,
                        PlayerInputCommand::Rotate(val) => vel_rel.movement_rot += vel_mul.speed_rot * val,
                    }
                    println!("{:?}", vel_rel)
                }
            }
        }
    }
}