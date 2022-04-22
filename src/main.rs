mod components;
mod render;
mod physics;
mod input;

use std::time::{Duration, Instant};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::{Point, Rect};
use sdl2::pixels::Color;
use sdl2::image::{self, LoadTexture, InitFlag};

use specs::prelude::*;
use specs::WorldExt;

use crate::components::*;
use crate::render::*;

fn initialize_player(world: &mut World) {
    world.create_entity() // Player
        .with(UserControlled)
        .with(Position{x: 0., y: 0.}) // TODO read starting values from map
        .with(Rotation{r: 0.})
        .with(VelocityMultiplier{speed: 32., speed_rot: 0.1})
        .with(VelocityRelative{movement_rel: (0., 0.), movement_rot: 0.})
        .with(Renderable)
        .with(IsPlayer)
        .build();
}

fn main() -> Result<(), String> {
    /**********************************************************************************************/
    // command-line arguments
    // TODO Command-line Arguments
    /**********************************************************************************************/
    // setup

    // Set up the rendering system, window etc.
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let _image_context = image::init(InitFlag::PNG)?;

    let window = video_subsystem
        .window("HuskyRock", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("could not make a canvas");

    let texture_creator = canvas.texture_creator();

    let mut event_pump = sdl_context.event_pump()?;

    // Set up the ECS

    let mut world = World::new();
    world.register::<UserControlled>();
    world.register::<Position>();
    world.register::<Rotation>();
    world.register::<VelocityMultiplier>();
    world.register::<VelocityRelative>();
    world.register::<Renderable>();
    world.register::<IsPlayer>();
    world.register::<Sprite>();

    let mut dispatcher = DispatcherBuilder::new()
        .with(input::Input, "Input", &[])
        .with(physics::Physics, "Physics", &[])
        // TODO add remaining systems
        .build();

    dispatcher.setup(&mut world);

    initialize_player(&mut world);

    world.insert(PlayerInput(Vec::new()));

    // set up the map
    // TODO add ability to load map from file
    world.insert(LevelMap(vec![
        vec![1,1,1,1,1,1,1,1],
        vec![1,0,1,0,0,0,0,1],
        vec![1,0,1,0,0,0,0,1],
        vec![1,0,1,0,0,0,0,1],
        vec![1,0,1,0,0,1,0,1],
        vec![1,0,0,0,0,1,0,1],
        vec![1,0,0,0,0,0,0,1],
        vec![1,1,1,1,1,1,1,1],
    ]));

    // TODO create entities

    let frame_time = Duration::from_millis(1000/60); // for 60 fps
    /**********************************************************************************************/
    // game loop
    'running: loop {
        let start_time = Instant::now();
        {
            let mut player_input: Vec<PlayerInputCommand> = Vec::new();
            let mut player_input_resource = world.write_resource::<PlayerInput>();

            // handle events
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running;
                    },

                    Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                        player_input.push(PlayerInputCommand::Forward(1.))
                    },
                    Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                        player_input.push(PlayerInputCommand::Sidewards(-1.))
                    },
                    Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                        player_input.push(PlayerInputCommand::Forward(-1.))
                    },
                    Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                        player_input.push(PlayerInputCommand::Sidewards(1.))
                    },
                    Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                        player_input.push(PlayerInputCommand::Rotate(1.))
                    },
                    Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                        player_input.push(PlayerInputCommand::Rotate(-1.))
                    },

                    Event::KeyUp { keycode: Some(Keycode::W), .. } => {
                        player_input.push(PlayerInputCommand::Forward(-1.))
                    },
                    Event::KeyUp { keycode: Some(Keycode::A), .. } => {
                        player_input.push(PlayerInputCommand::Sidewards(1.))
                    },
                    Event::KeyUp { keycode: Some(Keycode::S), .. } => {
                        player_input.push(PlayerInputCommand::Forward(1.))
                    },
                    Event::KeyUp { keycode: Some(Keycode::D), .. } => {
                        player_input.push(PlayerInputCommand::Sidewards(-1.))
                    },
                    Event::KeyUp { keycode: Some(Keycode::Right), .. } => {
                        player_input.push(PlayerInputCommand::Rotate(-1.))
                    },
                    Event::KeyUp { keycode: Some(Keycode::Left), .. } => {
                        player_input.push(PlayerInputCommand::Rotate(1.))
                    },
                    _ => {}
                }
            }

            *player_input_resource = PlayerInput(player_input);
        }
        // update
        dispatcher.dispatch(&mut world);
        world.maintain();

        // render
        render::render(&mut canvas, world.system_data())?;

        // time management
        ::std::thread::sleep(frame_time.saturating_sub(start_time.elapsed()));
    }

    Ok(())
}