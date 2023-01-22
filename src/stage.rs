use sdl2::render::Canvas;
use sdl2::render::Texture;
use sdl2::rect::{Point, Rect};

pub fn render_stage(canvas: &mut Canvas<sdl2::video::Window>,
                    background_texture: &Texture,
                    heart_texture: &Texture)
                    -> Result<(), String> {
    println!("Hola desde stage.rs");
    canvas.clear();
    canvas.copy(background_texture, None, None)?;
    let heart_positions = [50, 80, 110, 140, 170];
    for heart_position in heart_positions {
        let point = Point::new(heart_position, 30);
        let rect = Rect::new(point.x - 15, point.y -15,
                             30, 30);
        canvas.copy_ex(heart_texture,
                       None, rect, 0.0, point, false, false)?;
    }
    canvas.present();
    Ok(())
}
