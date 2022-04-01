mod components;
mod render;
mod physics;

use std::time::{Duration, Instant};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use sdl2::rect::{Point, Rect};
use sdl2::pixels::Color;
use sdl2::image::{self, LoadTexture, InitFlag};
//use clap::{Arg, Command};



fn main() -> Result<(), String> {
    /**********************************************************************************************/
    // TODO Command-line Arguments
    /**********************************************************************************************/
    // setup
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem.window("HuskyRock", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");

    let texture_creator = canvas.texture_creator();

    let mut event_pump = sdl_context.event_pump()?;

    let player_start = (0, 0, 0.); // TODO import starting position from map


    let frame_time = Duration::from_millis(1000/60); // for 60 fps
    /**********************************************************************************************/
    // game loop
    'running: loop {
        let start_time = Instant::now();

        // handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                Event::KeyDown {keycode: Some(Keycode::W), .. } => {

                },
                Event::KeyUp {keycode: Some(Keycode::W), .. } => {

                },
                _ => {}
            }
        }
        // update

        // render
        //render::render(&mut canvas, Color::RGB(i, j, 255 - i), &texture, position, sprite)?;

        // time management
        ::std::thread::sleep(frame_time.saturating_sub(start_time.elapsed()));
    }

    Ok(())
}