
use sdl2::rect::{Point, Rect};
use sdl2::render::Texture;

use specs_derive::Component;
use specs::prelude::*;

/**************************************************************************************************/
// movement

// is controlled by the player
#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct UserControlled;

// position of an entity
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

// rotational position of an entity
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Rotation {
    pub r: f64,
}

// velocity multipliers of an entity
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct VelocityMultiplier {
    pub speed: f64,
    pub speed_rot: f64,
}

// relative velocity of an entity
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct VelocityRelative {
    pub movement_rel: (f64, f64),
    pub movement_rot: f64,
}
/**************************************************************************************************/

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Sprite {
    pub spritesheet: usize,
    pub region: Rect,
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