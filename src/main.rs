extern crate sdl2;

use std::time::Duration;

use sdl2::pixels::Color;
use sdl2::image::LoadTexture;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;

mod stage;
mod dummy;

use dummy::Dummy;

const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;

const BACKGROUND_TEXTURE_PATH: &str = "./assets/images/backgrounds/background.png";
const HEART_TEXTURE_PATH: &str = "./assets/images/icons/heart.png";
const HERO_TEXTURE_PATH: &str = "./assets/images/dummies/link1.png";

const MAIN_SPEED: u32 = 200;

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

    let hero: &mut Dummy = &mut Dummy::default();
    let hero_texture = texture_creator.load_texture(HERO_TEXTURE_PATH)?;

    let src = Rect::new(0, 0, hero.size.x as u32, hero.size.y as u32);
    let dst = Rect::from_center(hero.position,
                                (hero.scale.x * hero.size.x) as u32,
                                (hero.scale.y * hero.size.y) as u32);
    canvas.clear();
    stage::render_stage(&mut canvas,
                        &background_texture,
                        &heart_texture)?;
    canvas.copy_ex(&hero_texture, src, dst, 0.0, None, false, false)?;
    canvas.present();

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
                },
                _ => {
                    dummy::manage_cursor_keys(hero, event);
                }
            }
        }
        stage::render_stage(&mut canvas, &background_texture, &heart_texture)?;
        let src_ox = hero.direction * hero.size.x;
        let src_oy = (hero.animation % hero.n_animation) * hero.size.y;
        let src = Rect::new(src_ox, src_oy,
                            hero.size.x as u32, hero.size.y as u32);
        let dst = Rect::from_center(
            hero.position,
            (hero.scale.x as u32) * (hero.size.x as u32),
            (hero.scale.y as u32) * (hero.size.y as u32));
        canvas.copy_ex(&hero_texture, src, dst, 0.0, None, false, false)?;
        (hero.movement)(hero);
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / MAIN_SPEED));
    }
    Ok(())
}
