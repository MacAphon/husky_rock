use sdl2::pixels::Color;
use sdl2::image::{self, LoadTexture, InitFlag};
use sdl2::render::{Texture, WindowCanvas};
use sdl2::rect::{Point, Rect};

// TODO replace render function

pub fn render_rectangle(
    canvas: &mut WindowCanvas,
    color: Color,
    position: (f64, f64),
    size: (u32, u32)
) -> Result<(), String> {
    let (width, height) = canvas.output_size()?;
    let screen_position = Point::new(width as i32 / 2 + position.0 as i32, height as i32 / 2 + position.1 as i32);
    let screen_rect = Rect::from_center(screen_position, size.0, size.1);

    canvas.set_draw_color(color);
    canvas.draw_rect(screen_rect);

    canvas.present();

    Ok(())
}

pub fn clear_canvas(
    canvas: &mut WindowCanvas,
    color: Color,
) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();
    canvas.present();
    Ok(())
}