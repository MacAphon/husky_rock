use std::f64::consts::{TAU, PI};

use sdl2::pixels::Color;
use sdl2::image::{self, LoadTexture, InitFlag};
use sdl2::render::{Texture, WindowCanvas};
use sdl2::rect::{Point, Rect};

use specs::prelude::*;
use specs::{AccessorCow, RunningTime};

use crate::components::*;
use crate::rays;
use crate::rays::*;

const PI_HALFS: f64 = PI / 2.;

pub type SystemData<'a> = (
    ReadStorage<'a, Position>,
    ReadStorage<'a, Rotation>,
    ReadStorage<'a, IsPlayer>,
    Read<'a, LevelMap>,
);

pub fn render(
    canvas: &mut WindowCanvas,
    wall_texture: &Texture,
    (pos, rot, _, level_map): SystemData,
) -> Result<(), String> {
    let n = 800;
    // Clear the screen.
    canvas.set_draw_color(Color::RGB(127, 127, 127));
    canvas.clear();

    render_rectangle(canvas, Color::RGB(127, 127, 255), (0., -150.), (800, 300))?;

    for (pos, rot) in (&pos, &rot).join() {
        let rays = multi_cast_ray_logic((pos.x, pos.y), rot.r, 0.01, n, &level_map);
        for ray in rays {
            let v_size = ((1. / (ray.0.0 + 0.001)) * 15000.) as u32;
            let sprite_x_offset = match ray.0.3 {
                WallDirection::Vertical => {
                    32 + (ray.0.2 as i32 % 64 / 2)
                },
                WallDirection::Horizontal => {
                    ray.1 as i32 % 64 / 2
                }
            };
            let render_x_offset = ray.1 as f64 - 400.;
            render_sprite(
                canvas,
                wall_texture,
                (sprite_x_offset, 32),
                (1, 32),
                (render_x_offset, 0.),
                (1, v_size),
            )?;
        }
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
    )?;
    Ok(())
}

fn render_sprite(
    canvas: &mut WindowCanvas,
    texture: &Texture,
    sprite_position: (i32, i32),
    sprite_size: (u32, u32),
    position: (f64, f64),
    render_size: (u32, u32),
) -> Result<(), String> {
    let (width, height) = canvas.output_size()?;
    canvas.copy(
        texture,
        Rect::from((
            sprite_position.0,
            sprite_position.1,
            sprite_size.0,
            sprite_size.1,
        )),
        Rect::from_center(
            Point::new(
                width as i32 / 2 + position.0 as i32,
                height as i32 / 2 + position.1 as i32),
            render_size.0,
            render_size.1,
        )
    )?;
    Ok(())
}