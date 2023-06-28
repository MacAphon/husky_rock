use std::f64::consts::*;

use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, WindowCanvas};

use specs::prelude::*;

use crate::components::*;
use crate::rays::*;

const FOV: f64 = FRAC_PI_2;

pub type SystemDataPl<'a> = (
    ReadStorage<'a, Position>,
    ReadStorage<'a, Rotation>,
    ReadStorage<'a, IsPlayer>,
    Read<'a, LevelMap>,
);

pub type SystemDataEN<'a> = (
    ReadStorage<'a, Position>,
    ReadStorage<'a, Sprite>,
);

pub fn render(
    canvas: &mut WindowCanvas,
    textures: &Vec<Texture>,
    (pos, rot, ipl, level_map): SystemDataPl,
    (pos_en, spr): SystemDataEN,
) -> Result<(), String> {
    let wall_texture = &textures[0];
    let n = 800;
    // Clear the screen.
    canvas.set_draw_color(Color::RGB(127, 127, 127));
    canvas.clear();

    render_rectangle(canvas, Color::RGB(127, 127, 255), (0., -150.), (800, 300))?;

    for (pos, rot, _) in (&pos, &rot, &ipl).join() {
        let mut entities: Vec<(f64, f64, f64, (f64, f64), usize, (i32, i32))> = Vec::new(); // dist, x, y, vp_pos, spritesheet, region
        let mut dist: f64;
        for (pos_en_i, spr_i) in (&pos_en, &spr).join() {
            dist = (pos.x - pos_en_i.x).hypot(pos.y - pos_en_i.y);
            entities.push((dist, pos_en_i.x, pos_en_i.y, vp_pos_h(&pos_en_i, &pos, &rot), spr_i.spritesheet, spr_i.region));
        }
        entities.sort_by(|b, a| a.0.partial_cmp(&b.0).unwrap());

        let rays = multi_cast_ray((pos.x, pos.y), rot.r, FOV, n, &level_map);
        
        let mut rendered_rays: Vec<bool> = vec![false; (n+1) as usize];
        for entity in &entities {
            for ray in &rays {
                if rendered_rays[ray.1 as usize] { continue }
                else if ray.0.0 < entity.0 { continue }
                let v_size = ((1. / (ray.0.0 + 0.001)) * 25000. * (-ray.2.cos() + 2.) ) as u32;
                let sprite_offset: (i32, i32) = match ray.0.3 {
                    WallDirection::Vertical => {
                        ((ray.0.4 as i32 - 1) * 32 + ray.0.2 as i32 % 64 / 2, 0)
                    }
                    WallDirection::Horizontal => {
                        ((ray.0.4 as i32 - 1) * 32 + ray.0.1 as i32 % 64 / 2, 32)
                    }
                };
                let render_x_offset = ray.1 as f64 - 400.;
                render_sprite(
                    canvas,
                    wall_texture,
                    (sprite_offset.0, sprite_offset.1),
                    (1, 32),
                    (render_x_offset, 0.),
                    (1, v_size),
                )?;
                rendered_rays[ray.1 as usize] = true;
            }
            //render_rectangle(canvas, Color::RGB(255, 0, 0), (entity.3, 0.), ((10000./entity.0) as u32, (10000./entity.0) as u32 ))?;
            render_sprite(
                canvas,
                &textures[entity.4],
                entity.5,
                (32, 32),
                (entity.3.0, 0.),
                ((27000./entity.0) as u32, (27000./entity.0) as u32 )
            )?;
        }
        for ray in &rays {
            if rendered_rays[ray.1 as usize] { continue }
            let v_size = ((1. / (ray.0.0 + 0.001)) * 25000. * (-ray.2.cos() + 2.) ) as u32;
            let sprite_offset: (i32, i32) = match ray.0.3 {
                WallDirection::Vertical => {
                    ((ray.0.4 as i32 - 1) * 32 + ray.0.2 as i32 % 64 / 2, 0)
                }
                WallDirection::Horizontal => {
                    ((ray.0.4 as i32 - 1) * 32 + ray.0.1 as i32 % 64 / 2, 32)
                }
            };
            let render_x_offset = ray.1 as f64 - 400.;
            render_sprite(
                canvas,
                wall_texture,
                (sprite_offset.0, sprite_offset.1),
                (1, 32),
                (render_x_offset, 0.),
                (1, v_size),
            )?;
            rendered_rays[ray.1 as usize] = true;
        }
        

        /* *****************************************************************************************/
        // debug rendering on top
        #[cfg(feature = "debug_rays")]
        {
            // top-down perspective of the rays
            canvas.set_draw_color(Color::RGB(255, 0, 255));
            for ray in &rays {
                canvas.set_draw_color(match ray.0 .3 {
                    WallDirection::Vertical => Color::RGB(0, 255, 0),
                    WallDirection::Horizontal => Color::RGB(0, 180, 0),
                });
                canvas.draw_line(
                    Point::new((pos.x/2.) as i32, (pos.y/2.) as i32),
                    Point::new((ray.0 .1/2.) as i32, (ray.0 .2/2.) as i32),
                )?;
            }
        }

        #[cfg(feature = "debug_map")]
        {
            // map
            for (y, yv) in level_map.0.iter().enumerate() {
                for (x, xv) in yv.iter().enumerate() {
                    if *xv != 0 {
                        render_rectangle_abs(
                            canvas,
                            Color::RGB(255, 0, 255),
                            ((x*32) as f64 + 16., (y*32) as f64 + 16.),
                            (30, 30),
                        )?;
                    }
                }
            }




            // positions of entities
            for entity in &entities {
                render_rectangle_abs(canvas, Color::RGB(255, 255, 0), (entity.1/2., entity.2/2.), (5, 5))?;
            }




            // position of the player
            render_rectangle_abs(canvas, Color::RGB(255, 255, 127), (pos.x/2.,pos.y/2.), (7, 7))?;
        }  


        /*
        // vertical line in the middle of the screen
        canvas.draw_line(
            Point::new(400, 0),
            Point::new(400, 600),
        )?;
         */
    }

    canvas.present();
    Ok(())
}

/// Draws a rectangle with the given color on the given position on the screen from the center
fn render_rectangle(
    canvas: &mut WindowCanvas,
    color: Color,
    position: (f64, f64),
    size: (u32, u32),
) -> Result<(), String> {
    let (width, height) = canvas.output_size()?;
    canvas.set_draw_color(color);
    canvas.fill_rect(Rect::from_center(
        Point::new(
            width as i32 / 2 + position.0 as i32,
            height as i32 / 2 + position.1 as i32,
        ),
        size.0,
        size.1,
    ))?;
    Ok(())
}

/// Same as above, except the position on the screen is absolute (from the top-left corner)
/// Used for debugging
fn render_rectangle_abs(
    canvas: &mut WindowCanvas,
    color: Color,
    position: (f64, f64),
    size: (u32, u32),
) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.fill_rect(Rect::from_center(
        Point::new(
            position.0 as i32,
            position.1 as i32,
        ),
        size.0,
        size.1,
    ))?;
    Ok(())
}

/// Draws a sprite from the given texture to the screen from the center
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
                height as i32 / 2 + position.1 as i32,
            ),
            render_size.0,
            render_size.1,
        ),
    )?;
    Ok(())
}

/// Calculate the horizontal position of an entity on the viewport
fn vp_pos_h(pos_e: &Position, pos_p: &Position, r_p: &Rotation) -> (f64, f64) {
    let dx = pos_p.x - pos_e.x;
    let dy = pos_p.y - pos_e.y;

    let abs_angle = dy.atan2(dx);
    let mut rel_angle = r_p.r + abs_angle;
    while rel_angle > PI { rel_angle -= TAU }
    while rel_angle < -PI { rel_angle += TAU}

    ((rel_angle/FOV) * 800., rel_angle)
}