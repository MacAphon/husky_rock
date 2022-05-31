mod components;
mod init;
mod input;
mod physics;
mod rays;
mod render;
mod ai;

use std::time::{Duration, Instant};
use std::thread;

use sdl2::event::Event;
use sdl2::image::{self, InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::render::Texture;

use specs::prelude::*;
use specs::WorldExt;

use crate::components::*;

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
    let mut textures: Vec<Texture> = Vec::new();
    textures.push(texture_creator.load_texture("assets/bricks_01.png")?);
    textures.push(texture_creator.load_texture("assets/lamp_01.png")?);
    textures.push(texture_creator.load_texture("assets/pot_01.png")?);

    let mut event_pump = sdl_context.event_pump()?;

    // Set up the ECS

    let mut world = World::new();
    world.register::<UserControlled>();
    world.register::<HasAI>();
    world.register::<Position>();
    world.register::<Rotation>();
    world.register::<VelocityMultiplier>();
    world.register::<VelocityRelative>();
    world.register::<IsPlayer>();
    world.register::<IsEntity>();
    world.register::<Sprite>();

    let mut dispatcher = DispatcherBuilder::new()
        .with(input::Input, "Input", &[])
        .with(physics::Physics, "Physics", &[])
        .with(ai::AI, "AI", &["Physics"])
        // TODO add remaining systems
        .build();

    dispatcher.setup(&mut world);

    init::initialize_player(&mut world);
    //for i in (128..769).step_by(256) {
    //    init::initialize_world_object(&mut world, (512., i as f64));
    //}
    init::initialize_world_object(&mut world, (1, 1));
    init::initialize_world_object(&mut world, (3, 1));
    init::initialize_world_object(&mut world, (5, 1));
    init::initialize_world_object(&mut world, (7, 1));
    init::initialize_world_object(&mut world, (9, 1));
    init::initialize_world_object(&mut world, (11, 1));
    init::initialize_world_object(&mut world, (13, 1));
    init::initialize_enemy(&mut world, (8, 8));

    world.insert(PlayerInput(Vec::new()));
    world.insert(PlayerPosition((0., 0.)));

    // set up the map
    // TODO add ability to load map from file
    world.insert(LevelMap(vec![
        vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![1, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 2, 0, 0, 0, 1],
        vec![1, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![1, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 1],
        vec![1, 0, 0, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 1],
        vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 1],
        vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 1],
        vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 0, 1],
        vec![1, 0, 0, 0, 2, 2, 2, 0, 0, 0, 0, 2, 0, 0, 0, 1],
        vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 1],
        vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    ]));

    // pre-loop setup
    let frame_time = Duration::from_millis(1000 / 60); // for 60 fps

    // W A S D -> <-
    let mut pressed_keys = [0, 0, 0, 0, 0, 0];
    /**********************************************************************************************/
    // game loop
    'running: loop {
        let start_time = Instant::now();
        {
            let mut player_input: Vec<PlayerInputCommand> = Vec::new();
            let mut player_input_resource = world.write_resource::<PlayerInput>();
            let mut player_position = world.write_resource::<PlayerPosition>();

            // handle events
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        break 'running;
                    }

                    Event::KeyDown {
                        keycode: Some(Keycode::W),
                        ..
                    } => {
                        if pressed_keys[0] == 0 {
                            player_input.push(PlayerInputCommand::Forward(1.));
                            pressed_keys[0] = 1;
                        }
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::A),
                        ..
                    } => {
                        if pressed_keys[1] == 0 {
                            player_input.push(PlayerInputCommand::Sidewards(-1.));
                            pressed_keys[1] = 1;
                        }
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::S),
                        ..
                    } => {
                        if pressed_keys[2] == 0 {
                            player_input.push(PlayerInputCommand::Forward(-1.));
                            pressed_keys[2] = 1;
                        }
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::D),
                        ..
                    } => {
                        if pressed_keys[3] == 0 {
                            player_input.push(PlayerInputCommand::Sidewards(1.));
                            pressed_keys[3] = 1;
                        }
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Right),
                        ..
                    } => {
                        if pressed_keys[4] == 0 {
                            player_input.push(PlayerInputCommand::Rotate(-1.));
                            pressed_keys[4] = 1;
                        }
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Left),
                        ..
                    } => {
                        if pressed_keys[5] == 0 {
                            player_input.push(PlayerInputCommand::Rotate(1.));
                            pressed_keys[5] = 1;
                        }
                    }

                    Event::KeyUp {
                        keycode: Some(Keycode::W),
                        ..
                    } => {
                        if pressed_keys[0] == 1 {
                            player_input.push(PlayerInputCommand::Forward(-1.));
                            pressed_keys[0] = 0;
                        }
                    }
                    Event::KeyUp {
                        keycode: Some(Keycode::A),
                        ..
                    } => {
                        if pressed_keys[1] == 1 {
                            player_input.push(PlayerInputCommand::Sidewards(1.));
                            pressed_keys[1] = 0;
                        }
                    }
                    Event::KeyUp {
                        keycode: Some(Keycode::S),
                        ..
                    } => {
                        if pressed_keys[2] == 1 {
                            player_input.push(PlayerInputCommand::Forward(1.));
                            pressed_keys[2] = 0;
                        }
                    }
                    Event::KeyUp {
                        keycode: Some(Keycode::D),
                        ..
                    } => {
                        if pressed_keys[3] == 1 {
                            player_input.push(PlayerInputCommand::Sidewards(-1.));
                            pressed_keys[3] = 0;
                        }
                    }
                    Event::KeyUp {
                        keycode: Some(Keycode::Right),
                        ..
                    } => {
                        if pressed_keys[4] == 1 {
                            player_input.push(PlayerInputCommand::Rotate(1.));
                            pressed_keys[4] = 0;
                        }
                    }
                    Event::KeyUp {
                        keycode: Some(Keycode::Left),
                        ..
                    } => {
                        if pressed_keys[5] == 1 {
                            player_input.push(PlayerInputCommand::Rotate(-1.));
                            pressed_keys[5] = 0;
                        }
                    }
                    _ => {}
                }
            }

            *player_input_resource = PlayerInput(player_input);
            *player_position = PlayerPosition(physics::get_player_position(world.system_data()));
        }
        // update
        dispatcher.dispatch(&mut world);
        world.maintain();

        // render
        render::render(&mut canvas, &textures, world.system_data(), world.system_data())?;

        // time management
        thread::sleep(frame_time.saturating_sub(start_time.elapsed()));
    }

    Ok(())
}
