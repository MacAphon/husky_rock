use std::f64::consts::TAU;

use specs::prelude::*;
use specs::{AccessorCow, RunningTime};

use crate::components::*;

struct Physics;

impl<'a> System<'a> for Physics {

    type SystemData = (
        WriteStorage<'a, Position>,
        WriteStorage<'a, Rotation>,
        ReadStorage<'a, VelocityRelative>,
    );

    fn run(&mut self, (mut position, mut rotation, vel_rel): Self::SystemData) {

        for (position,
             rotation,
             vel_rel)
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

            let mut pos: (f64, f64) =
                (position.x + movement_abs.0,
                 position.y + movement_abs.1);

            // TODO add checks for collision and out of bounds

            rotation.r = rot;
            position.x = pos.0;
            position.y = pos.1;
        }

    }
/*

    fn update_movement_abs() {
        /*
        calculate absolute movement from relative movement
         */
        self.movement_abs.0 =
            self.movement_rel.0 * self.position.2.cos() + self.movement_rel.1 * self.position.2.sin();
        self.movement_abs.1 =
            self.movement_rel.1 * self.position.2.cos() - self.movement_rel.0 * self.position.2.sin();
    }

    fn set_movement_rel(f: f64, s: f64, r: f64) {
        self.movement_rel.0 += f * self.speed.0;
        self.movement_rel.1 += s * self.speed.0;
        self.movement_rel.3 += r * self.speed.1;
    }

 */
}