use sdl2::pixels::Color;
use sdl2::image::{self, LoadTexture, InitFlag};
use sdl2::render::{Texture, WindowCanvas};
use sdl2::rect::{Point, Rect};

// TODO replace render function

pub fn render(
    canvas: &mut WindowCanvas,
    color: Color,
    texture: &Texture,
    position: Point,
    sprite: Rect
) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();

    let (width, height) = canvas.output_size()?;
    let screen_position = position + Point::new(width as i32 / 2, height as i32 / 2);
    let screen_rect = Rect::from_center(screen_position, sprite.width(), sprite.height());

    canvas.copy(texture, sprite, screen_rect)?;

    canvas.present();

    Ok(())
}