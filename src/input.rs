use specs::prelude::*;
use specs::{AccessorCow, RunningTime};
use specs::storage::GenericReadStorage;

use crate::components::*;
use super::PlayerInputCommand;

pub struct Input;

impl<'a> System<'a> for Input {
    type SystemData = (
        ReadExpect<'a, Option<Vec<(PlayerInputCommand, f64)>>>,
        ReadStorage<'a, UserControlled>,
        ReadStorage<'a, VelocityMultiplier>,
        WriteStorage<'a, VelocityRelative>,
    );

    fn run(&mut self, (move_cmd, usr_ctr, vel_mul, mut vel_rel): Self::SystemData) {

        let move_cmd = match move_cmd {
            Some(Vec) => move_cmd,
            None => return,
        };

        for (_, vel_mul, mut vel_rel) in (usr_ctr, vel_mul, &mut vel_rel).join() {
            for element in move_cmd{
                match element.0 {
                    PlayerInputCommand::Forward => vel_rel.movement_rel.0 += vel_mul.speed * element.1,
                    PlayerInputCommand::Sidewards => vel_rel.movement_rel.1 += vel_mul.speed * element.1,
                    PlayerInputCommand::Rotate => vel_rel.movement_rot += vel_mul.speed_rot * element.1,
                }
            }
        }
    }
}