extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::image::LoadTexture;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod stage;

const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;

const BACKGROUND_TEXTURE_PATH: &str = "./assets/images/backgrounds/background.png";
const HEART_TEXTURE_PATH: &str = "./assets/images/icons/heart.png";

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsytem = sdl_context.video().unwrap();
    let window = video_subsytem
        .window("Game", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.clear();
    canvas.present();

    let texture_creator = canvas.texture_creator();
    let background_texture = texture_creator
        .load_texture(BACKGROUND_TEXTURE_PATH)?;
    let heart_texture = texture_creator
        .load_texture(HEART_TEXTURE_PATH)?;
    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown { keycode: Some(Keycode::Q), .. }
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    println!("Saliendo");
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::X), .. } => {
                    println!("Has pulsado la X");
                    stage::render_stage(&mut canvas,
                                        &background_texture,
                                        &heart_texture)?;
                },
                _ => { }
            }
        }
    }

    Ok(())
}
