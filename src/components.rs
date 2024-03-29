use specs::prelude::*;
use specs_derive::*;

/* *************************************************************************************************/
// definitions

#[derive(Debug, PartialEq)]
pub enum WallDirection {
    Horizontal,
    Vertical,
}

#[derive(Debug)]
pub enum PlayerInputCommand {
    Forward(f64),
    Sidewards(f64),
    Rotate(f64),
    // TODO add other player input as it is introduced
}

/* *************************************************************************************************/
// movement

// is controlled by the player
#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct UserControlled;

// is controlled by AI
#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
pub struct HasAI {
    pub time_to_next_update: i32,
    pub path: Vec<(usize, usize)>,
}

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
#[derive(Default, Debug)]
pub struct PlayerInput(pub Vec<PlayerInputCommand>);

// position of the Player
#[derive(Default, Debug)]
pub struct PlayerPosition(pub (f64, f64));
/* *************************************************************************************************/
// rendering

// can be rendered on the map
#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct Renderable;

#[derive(Default, Debug)]
pub struct LevelMap(pub Vec<Vec<u32>>);

// is the player, only to be used ONCE
#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct IsPlayer;

// is an entity, NOT THE PLAYER
#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct IsEntity;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Sprite {
    pub spritesheet: usize,
    pub region: (i32, i32),
}
