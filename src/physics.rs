use std::f64::consts::TAU;

use specs::prelude::*;

use crate::components::*;

pub struct Physics;

impl<'a> System<'a> for Physics {
    type SystemData = (
        Read<'a, LevelMap>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Rotation>,
        ReadStorage<'a, VelocityRelative>,
    );

    fn run(&mut self, (lvl_map, mut position, mut rotation, vel_rel): Self::SystemData) {
        let level = &lvl_map.0;
        let level_size = level.len();
        for (
            position,
            rotation,
            vel_rel
        ) in (
            &mut position,
            &mut rotation,
            &vel_rel
        ).join() {
            let movement_rot = vel_rel.movement_rot;
            let movement_rel = vel_rel.movement_rel;

            let mut rot: f64 = rotation.r + movement_rot; // calculate new rotation
            if rot <= 0. {
                // adjust rotation to be in normal range
                rot += TAU;
            } else if rot > TAU {
                rot -= TAU;
            }

            let movement_abs: (f64, f64) = (
                -movement_rel.0 * rot.cos() - movement_rel.1 * rot.sin(),
                -movement_rel.1 * rot.cos() + movement_rel.0 * rot.sin(),
            );

            let mut pos: (f64, f64) = (position.x + movement_abs.0, position.y + movement_abs.1);

            // keep the entity in the map
            if pos.0 <= 0. {
                pos.0 = 0.;
            } else if pos.0 >= (level_size << 6) as f64 {
                pos.0 = (level_size << 6) as f64;
            }
            if pos.1 <= 0. {
                pos.1 = 0.;
            } else if pos.1 >= (level_size << 6) as f64 {
                pos.1 = (level_size << 6) as f64;
            }

            // collision checks
            // left
            if level[pos.1 as usize >> 6][(pos.0 - 8.) as usize >> 6] != 0 {
                pos.0 += pos.0 - ((pos.0 as usize >> 6) << 6) as f64;
            }
            // right
            if level[pos.1 as usize >> 6][(pos.0 + 8.) as usize >> 6] != 0 {
                pos.0 += ((pos.0 as usize >> 6) << 6) as f64 - pos.0;
            }
            // up
            if level[(pos.1 - 8.) as usize >> 6][pos.0 as usize >> 6] != 0 {
                pos.1 += pos.1 - ((pos.1 as usize >> 6) << 6) as f64;
            }
            // down
            if level[(pos.1 + 8.) as usize >> 6][pos.0 as usize >> 6] != 0 {
                pos.1 += ((pos.1 as usize >> 6) << 6) as f64 - pos.1;
            }

            // update position and rotation
            rotation.r = rot;
            position.x = pos.0;
            position.y = pos.1;
        }
    }
}
