use std::f64::consts::{TAU, PI};

use sdl2::pixels::Color;
use sdl2::image::{self, LoadTexture, InitFlag};
use sdl2::render::{Texture, WindowCanvas};
use sdl2::rect::{Point, Rect};

use specs::prelude::*;
use specs::{AccessorCow, RunningTime};

use crate::components::*;
use crate::WallDirection::{horizontal, vertical};

const PI_HALFS: f64 = PI / 2;

// TODO replace render function

pub type SystemDataRender<'a> = (
    ReadStorage<'a, Position>,
    ReadStorage<'a, Sprite>,
    ReadStorage<'a, Renderable>
);

pub fn render(
    canvas: &mut WindowCanvas,
    data: SystemDataRender,
) -> Result<(), String> {
    clear_canvas(canvas, Color::RGB(255, 255, 255))?;

    for (position, sprite, _) in (&data.0, &data.1, &data.2).join() {

    }

    canvas.present();
    Ok(())
}

fn render_rectangle(
    canvas: &mut WindowCanvas,
    color: Color,
    position: (f64, f64),
    size: (u32, u32)
) -> Result<(), String> {
    let (width, height) = canvas.output_size()?;
    canvas.set_draw_color(color);
    canvas.fill_rect(
        Rect::from_center(
            Point::new(
                width as i32 / 2 + position.0 as i32,
                height as i32 / 2 + position.1 as i32),
            size.0,
            size.1
        )
    );
    Ok(())
}

fn clear_canvas(
    canvas: &mut WindowCanvas,
    color: Color,
) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();
    Ok(())
}

/**************************************************************************************************/
/**************************************************************************************************/
// raycaster

pub type SystemDataRay<'a> = (
    Read<'a, LevelMap>,
);
pub enum WallDirection {
    horizontal,
    vertical
}

/// Calculates the length of a ray across the map.
///
/// Returns the length of the ray, x- and y-coordinates of its end and the direction of the wall which was hit.
pub fn cast_ray(
    start: (f64, f64),
    mut angle: f64,
    level: SystemDataRay,
) -> (f64, f64, f64, WallDirection) {
    let level_size = level.len();
    let mut h_is_0 = false;
    let mut v_is_0 = false;
    let mut rx: f64 = 0.;
    let mut ry: f64 = 0.;
    let mut x_offset: i32 = 64;
    let mut y_offset: i32 = 64;
    let mut dof: i32 = 0;

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

    let atan = 1 / angle.tan();
    let natan = -1 / angle_x.tan();

    /**********************************************************************************************/
    // Vertical lines

    if angle > PI_HALFS && angle < (PI_HALFS+PI) { // looking right
        rx = ((start.0  / 64) * 64) + 64;
        ry = (start.0 - rx) * natan + start.1;
        x_offset = 64;
        y_offset = x_offset * natan;
    }
    if angle > (PI_HALFS+PI) || angle < PI_HALFS { // looking left
        rx = ((start.0 / 64) * 64) -1;
        ry = (start.0 - rx) * natan + start.1;
        x_offset = -64;
        y_offset = -x_offset * natan;
    }
    if angle == PI_HALFS || angle == (PI+PI_HALFS) { // looking straight up or down
        rx = start.0;
        ry = start.1;
        v_is_0 = true;
        dof = level_size;

    }

    while dof < level_size { // check for walls
        let mx: i32 = rx as i32 >> 6;
        let my: i32 = ry as i32 >> 6;
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
        ry = ((start.1 / 64) * 64) - 1;
        rx = (start.1 - ry) * atan + start.0;
        y_offset = -64;
        x_offset = -y_offset * atan;
    }
    if angle < PI { // looking down
        ry = ((start.1 / 64) * 64) + 64;
        rx = (start.1 - ry) * atan + start.0;
        y_offset = 64;
        x_offset = -y_offset * atan;
    }
    if angle == 0. || angle == PI || angle == TAU { // looking straight left or right
        rx = start.0;
        ry = start.1;
        h_is_0 = true;
        dof = level_size;
    }

    while dof < level_size { // check for walls
        let mx = rx as i32 >> 6;
        let my = ry as i32 >> 6;
        if mx >= 0 && mx < levle_size && my >= 0 && my < level_size {
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
    let vdist = ((rvx - start.0).powi(2) + (rvy - start.2).powi(2)).sqrt(); // pythagoras
    let vdist = ((rhx - start.0).powi(2) + (rhy - start.2).powi(2)).sqrt();

    if hdist > vdist && !v_is_0 {
        dist = vdist;
        rx = rvx;
        ry = rvy;
        wd = vertical;
    } else if !h_is_0 {
        dist = hdist;
        rx = rhx;
        ry = rhy;
        wd = horizontal;
    } else {
        dist = vdist;
        rx = rvx;
        ry = rvy;
        wd = vertical;
    }

    return (dist, rx, ry, wd);
}
/**************************************************************************************************/