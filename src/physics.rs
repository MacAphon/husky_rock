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

    fn run(&mut self, mut data: Self::SystemData){
        let mut position = &mut data.0.position;
        let mut rotation = &mut data.1.rotation;
        let movement_rel = &data.2.movement_rel;
        let movement_rot = &data.2.movement_rot;

        let mut rot: f64 = rotation + movement_rot; // calculate new rotation
        if rot <= 0. { // adjust rotation to be in normal range
            rot += TAU;
        } else if rot > TAU {
            rot -= TAU;
        }

        let mut movement_abs: (i32, i32) = (
            movement_rel.0 * rot.cos() + movement_rel.1 * rot.sin(),
            movement_rel.1 * rot.cos() - movement_rel.0 * rot.sin(),
        );

        let mut pos: (f64, f64, f64) =
            (position.0 + movement_abs.0,
             position.1 + movement_abs.1,
             rot);

        // TODO add checks for collision and out of bounds

        rotation = rot;
        position = pos;
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