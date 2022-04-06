use std::f64::consts::TAU;

use specs::prelude::*;
use specs::{AccessorCow, RunningTime};

use crate::components::*;

pub struct Physics;

impl<'a> System<'a> for Physics {
    type SystemData = (
        WriteStorage<'a, Position>,
        WriteStorage<'a, Rotation>,
        ReadStorage<'a, VelocityRelative>,
    );

    fn run(&mut self, (mut position, mut rotation, vel_rel): Self::SystemData) {
        for (
            position,
            rotation,
            vel_rel
        )
        in (&mut position, &mut rotation, &vel_rel).join() {

            let movement_rot = vel_rel.movement_rot;
            let movement_rel = vel_rel.movement_rel;

            let mut rot: f64 = rotation.r + movement_rot; // calculate new rotation
            if rot <= 0. { // adjust rotation to be in normal range
                rot += TAU;
            } else if rot > TAU {
                rot -= TAU;
            }

            let mut movement_abs: (f64, f64) = (
                movement_rel.0 * rot.cos() + movement_rel.1 * rot.sin(),
                movement_rel.1 * rot.cos() - movement_rel.0 * rot.sin(),
            );

            let mut pos: (f64, f64) = (
                position.x + movement_abs.0,
                position.y + movement_abs.1
            );

            // TODO add checks for collision and out of bounds

            rotation.r = rot;
            position.x = pos.0;
            position.y = pos.1;
        }
    }
}