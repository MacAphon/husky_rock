/*use std::f64::consts::TAU;

trait Entity {
    fn movement(&mut self) {
        /*
        update position and rotation
         */
        let mut rot: f64 = self.position.3 + self.movement_rel.3; // calculate new rotation
        if rot <= 0. { // adjust rotation to be in normal range
            rot += TAU;
        } else if rot > TAU {
            rot -= TAU;
        }
        let mut new_pos: (f64, f64, f64) =
            (self.position.0 + self.movement_abs.0,
             self.position.1 + self.movement_abs.1,
             rot);

        // TODO add checks for collision and out of bounds

        self.position = new_pos;
    }

    fn update_movement_abs(&mut self){
        /*
        calculate absolute movement from relative movement
         */
        self.movement_abs.0 =
            self.movement_rel.0 * self.position.2.cos() + self.movement_rel.1 * self.position.2.sin();
        self.movement_abs.1 =
            self.movement_rel.1 * self.position.2.cos() - self.movement_rel.0 * self.position.2.sin();
    }

    fn set_movement_rel(&mut self, f: f64, s: f64, r: f64) {
        self.movement_rel.0 += f * self.speed.0;
        self.movement_rel.1 += s * self.speed.0;
        self.movement_rel.3 += r * self.speed.1;
    }

    fn cast_ray(){
        /*
        cast a ray in the map
        used to:
            - render the world
            - check sightlines
            - etc.
         */
        // TODO
    }

    fn draw_map();

    fn draw_viewport();
}
*/
use sdl2::rect::{Point, Rect};
use sdl2::render::Texture;

#[derive(Debug)]
pub struct Player {
    position: Point,
    rotation: f64,
    movement_rel: (f64, f64, f64), // forward, sidewards (right), rotation (anticlockwise)
    movement_abs: (f64, f64),      // x, y
    speed: (f64, f64),             // translation, rotation
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

/*
impl Entity for Player {
    fn draw_map() {
        // TODO
    }
    fn draw_viewport() {
        // TODO
    }
}
*/