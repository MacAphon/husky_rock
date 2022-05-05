use std::f64::consts::{TAU, PI};

use sdl2::pixels::Color;
use sdl2::image::{self, LoadTexture, InitFlag};
use sdl2::render::{Texture, WindowCanvas};
use sdl2::rect::{Point, Rect};

use specs::prelude::*;
use specs::{AccessorCow, RunningTime};
use crate::LevelMap;

const PI_HALFS: f64 = PI / 2.;

pub enum WallDirection {
    Horizontal,
    Vertical
}

pub type SystemData<'a> = Read<'a, LevelMap>;

/// Calculates the length of a ray across the map.
///
/// Returns the length of the ray, x- and y-coordinates of its end
/// and the direction of the wall which was hit.
pub fn cast_ray_logic(
    start: (f64, f64),
    mut angle: f64,
    data: &SystemData,
) -> (f64, f64, f64, WallDirection) {
    let level = &data.0;
    let level_size = level.len();
    let mut h_is_0 = false;
    let mut v_is_0 = false;
    let mut rx: f64 = 0.;
    let mut ry: f64 = 0.;
    let mut x_offset: f64 = 64.;
    let mut y_offset: f64 = 64.;
    let mut dof: usize = 0;

    if angle < 0. {
        angle = angle + TAU;
    } else if angle > TAU {
        angle = angle - TAU;
    }

    let mut angle_x = angle + PI_HALFS;
    if angle_x < 0. {
        angle_x = angle_x + TAU;
    } else if angle_x > TAU {
        angle_x = angle_x - TAU;
    }

    let atan = 1. / angle.tan();
    let natan = -1. / angle_x.tan();

    /**********************************************************************************************/
    // Vertical lines

    if angle > PI_HALFS && angle < (PI_HALFS+PI) { // looking right
        rx = ((start.0  / 64.) * 64.) + 64.;
        ry = (start.0 - rx) * natan + start.1;
        x_offset = 64.;
        y_offset = x_offset * natan;
    }
    if angle > (PI_HALFS+PI) || angle < PI_HALFS { // looking left
        rx = ((start.0 / 64.) * 64.) -1.;
        ry = (start.0 - rx) * natan + start.1;
        x_offset = -64.;
        y_offset = -x_offset * natan;
    }
    if angle == PI_HALFS || angle == (PI+PI_HALFS) { // looking straight up or down
        rx = start.0;
        ry = start.1;
        v_is_0 = true;
        dof = level_size;

    }

    while dof < level_size { // check for walls
        let mx = rx as usize >> 6;
        let my = ry as usize >> 6;
        if mx >= 0 && mx < level_size && my >= 0 && my < level_size {
            if level[my][mx] == 1 { // hit wall
                dof = level_size;
            } else {
                rx += x_offset;
                ry += y_offset;
                dof += 1;
            }
        } else {
            rx += x_offset;
            ry += y_offset;
            dof += 1;
        }
    }
    let rvx: f64 = rx;
    let rvy: f64 = ry;

    /**********************************************************************************************/
    // Horizontal lines

    dof = 0;

    if angle > PI { // looking up
        ry = ((start.1 / 64.) * 64.) - 1.;
        rx = (start.1 - ry) * atan + start.0;
        y_offset = -64.;
        x_offset = -y_offset * atan;
    }
    if angle < PI { // looking down
        ry = ((start.1 / 64.) * 64.) + 64.;
        rx = (start.1 - ry) * atan + start.0;
        y_offset = 64.;
        x_offset = -y_offset * atan;
    }
    if angle == 0. || angle == PI || angle == TAU { // looking straight left or right
        rx = start.0;
        ry = start.1;
        h_is_0 = true;
        dof = level_size;
    }

    while dof < level_size { // check for walls
        let mx = rx as usize >> 6;
        let my = ry as usize >> 6;
        if mx >= 0 && mx < level_size && my >= 0 && my < level_size {
            if level[my][mx] == 1 {
                dof = level_size;
            } else {
                rx += x_offset;
                ry += y_offset;
                dof += 1
            }
        } else {
            rx += x_offset;
            ry += y_offset;
            dof += 1;
        }
    }
    let rhx: f64 = rx;
    let rhy: f64 = ry;

    let mut dist: f64;
    let mut wd: WallDirection;
    let vdist = ((rvx - start.0).powi(2) + (rvy - start.1).powi(2)).sqrt(); // pythagoras
    let hdist = ((rhx - start.0).powi(2) + (rhy - start.1).powi(2)).sqrt();

    if hdist > vdist && !v_is_0 {
        dist = vdist;
        rx = rvx;
        ry = rvy;
        wd = WallDirection::Vertical;
    } else if !h_is_0 {
        dist = hdist;
        rx = rhx;
        ry = rhy;
        wd = WallDirection::Horizontal;
    } else {
        dist = vdist;
        rx = rvx;
        ry = rvy;
        wd = WallDirection::Vertical;
    }

    return (dist, rx, ry, wd);
}
/**************************************************************************************************/

pub fn multi_cast_ray_logic(
    start: (f64, f64),
    angle: f64,
    fov: f64,
    n: u32,
    data: &SystemData,
) -> Vec<((f64, f64, f64, WallDirection), u32)> {
    let mut ret_data: Vec<((f64, f64, f64, WallDirection), u32)> = Vec::new();
    let mut current_angle = angle - fov/2.;
    let delta_angle = fov / n as f64;
    for i in 0..n+1 {
        ret_data.push((cast_ray_logic(start, current_angle, data), i));
        current_angle += delta_angle * (i as f64);
    }
    ret_data
}
/**************************************************************************************************/

pub fn cast_ray(
    start: (f64, f64),
    angle: f64,
    data: SystemData,
) -> (f64, f64, f64, WallDirection) {
    cast_ray_logic(start, angle, &data)
}

pub fn multi_cast_ray(
    start: (f64, f64),
    angle: f64,
    fov: f64,
    n: u32,
    data: SystemData,
) -> Vec<((f64, f64, f64, WallDirection), u32)> {
    multi_cast_ray_logic(start, angle, fov, n, &data)
}