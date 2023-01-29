use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::render::Texture;
use sdl2::rect::{Point, Rect};

use crate::FONT_PATH;

pub fn render_stage(canvas: &mut Canvas<sdl2::video::Window>,
                    background_texture: &Texture,
                    heart_texture: &Texture,
                    lives: i32,
                    score: i32) -> Result<(), String> {
    canvas.copy(background_texture, None, None)?;
    let heart_positions = [50, 80, 110, 140, 170];
    let mut i = 1;
    for heart_position in heart_positions {
        if i > lives {
            continue;
        }
        i += 1;
        let point = Point::new(heart_position, 30);
        let rect = Rect::new(point.x - 15, point.y -15,
                             30, 30);
        canvas.copy_ex(heart_texture,
                       None, rect, 0.0, point, false, false)?;
    }
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let mut font = ttf_context.load_font(FONT_PATH, 24)?;
    font.set_style(sdl2::ttf::FontStyle::BOLD);
    let score_label = format!("Score : {}", score);
    let surface = font.render(&score_label).blended(Color::RGBA(0, 0, 0, 200)).map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.create_texture_from_surface(&surface).map_err(|e| e.to_string())?;
    let point = Point::new(450, 0);
    let rect = Rect::new(point.x, point.y, 14 * (score_label.len() as u32), 60);
    canvas.copy(&texture, None, Some(rect))?;

    Ok(())
}
