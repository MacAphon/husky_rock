use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::render::WindowCanvas;

fn render(canvas: &mut WindowCanvas, color: Color) {
    canvas.set_draw_color(color);
    canvas.clear();
    canvas.present();
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("HuskyRock", 1024, 512)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;
    let mut j = 0;

    'running: loop {
        // handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                Event::KeyDown {keycode: Some(Keycode::W), .. } => {
                    j = 128
                },
                Event::KeyUp {keycode: Some(Keycode::W), .. } => {
                    j = 0
                },
                _ => {}
            }
        }
        // update
        i = (i + 1) % 255;

        // render
        render(&mut canvas, Color::RGB(i, j, 255 - i));

        // time management
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}