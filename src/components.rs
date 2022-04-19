use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

use specs_derive::*;
use specs::prelude::*;

/**************************************************************************************************/
// definitions

pub enum PlayerInputCommand {
    Forward(f64),
    Sidewards(f64),
    Rotate(f64),
    // TODO add other player input as it is introduced
}

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

// input handling of the player
#[derive(Default)]
pub struct PlayerInput(pub Vec<PlayerInputCommand>);
/**************************************************************************************************/
// rendering

// can be rendered on the map
#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct RenderableMap;

// the window on which things are rendered
#[derive(Default)]
pub struct Canvas(pub WindowCanvas);

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Sprite {
    pub spritesheet: usize,
    pub region: Rect,
}