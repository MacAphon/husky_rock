
use sdl2::rect::{Point, Rect};
use sdl2::render::Texture;

use specs_derive::Component;
use specs::prelude::*;


// position of an entity
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position {
    position: (f64, f64),
}

// rotational position of an entity
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Rotation {
    rotation: f32,
}

// absolute velocity of an entity
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct VelocityAbsolute {
    speed: i32,
}

// relative velovity of an entity
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct VelocityRelative {
    movement_rel: (f64, f64),
    movement_rot: (f64),
    speed_rot: f64,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Sprite {
    spritesheet: usize,
    region: Rect,
}

// TODO move this into a new file
/*
trait Entity {


    fn cast_ray(){
        /*
        cast a ray in the map
        used to:
            - render the world
            - check sightlines
            - etc.
         */
        // TODO add cast ray function body
    }

    fn draw_map();

    fn draw_viewport();
}


pub fn new_player(start_pos: (i32, i32, f64)) -> Player {
    Player {
        position: Point::new(start_pos.0, start_pos.1),
        rotation: start_pos.2,
        movement_rel: (0.0, 0.0, 0.0),
        movement_abs: (0.0, 0.0),
        speed: (0.0, 0.0)
    }
}


impl Entity for Player {
    fn draw_map() {
        // TODO add draw map function body
    }
    fn draw_viewport() {
        // TODO add draw viewport function body
    }
}
*/