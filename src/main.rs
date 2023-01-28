extern crate sdl2;

use std::time::Duration;

use sdl2::pixels::Color;
use sdl2::image::LoadTexture;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::{Rect, Point};
use sdl2::render::Canvas;
use sdl2::render::Texture;

use arrayvec::ArrayVec;
use rand::Rng;

mod stage;
mod dummy;
mod collision;
mod sound;

use dummy::Dummy;

const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;

const BACKGROUND_TEXTURE_PATH: &str = "./assets/images/backgrounds/background.png";
const HEART_TEXTURE_PATH: &str = "./assets/images/icons/heart.png";
const HERO_TEXTURE_PATH: &str = "./assets/images/dummies/link1.png";
const DEALER_TEXTURE_PATH: &str = "./assets/images/dummies/wizard1.png";
const DANGEROUS_DEALER_TEXTURE_PATH: &str = "./assets/images/dummies/red1.png";

const MAIN_SPEED: u32 = 200;
const N_DEALERS: usize = 20;

fn init_dealers(dealers: &mut ArrayVec<Dummy, N_DEALERS>, size: Point, scale: Point) {
    for _ in 0..N_DEALERS {
        let random_x = rand::thread_rng().gen_range((size.x * scale.x)..=(SCREEN_WIDTH as i32 - (size.x * scale.x)));
        let random_y = rand::thread_rng().gen_range((size.y * scale.y)..=(SCREEN_HEIGHT as i32 -(size.y * scale.y)));
        let random_direction = rand::thread_rng().gen_range(0..4);
        dealers.push(
            Dummy {
                position: Point::new(random_x, random_y),
                speed: Point::new(0, 0),
                scale,
                size,
                n_direction: 4,
                direction: random_direction,
                animation: 0,
                n_animation: 2,
                state: 0,
                collided: false,
                movement: dummy::random_movement
            });
    }
}

fn render_dealers(dealers: &mut ArrayVec<Dummy, N_DEALERS>,
                  canvas: &mut Canvas<sdl2::video::Window>,
                  dealer_texture: &Texture,
                  dangerous_dealer_texture: &Texture) -> Result<(), String> {
    for dealer in dealers {
        let src_ox = dealer.direction * dealer.size.x;
        let src_oy = (dealer.animation % dealer.n_animation) * dealer.size.y;
        let src = Rect::new(src_ox, src_oy,
                            dealer.size.x as u32, dealer.size.y as u32);
        let dst = Rect::from_center(dealer.position,
                                    (dealer.scale.x as u32) * (dealer.size.x as u32),
                                    (dealer.scale.y as u32) * (dealer.size.y as u32));
        if dealer.state == 0 {
            canvas.copy_ex(dealer_texture, src, dst, 0.0, None, false, false)?;
        } else {
            canvas.copy_ex(dangerous_dealer_texture, src, dst, 0.0, None, false, false)?;
        }
    }
    Ok(())
}

fn move_dealers(dealers: &mut ArrayVec<Dummy, N_DEALERS>) {
    for dealer in &mut *dealers {
        (dealer.movement)(dealer);
    }
    for i in 0..N_DEALERS {
        (dealers[i].movement)(&mut dealers[i]);
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
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

    let mut dealers = ArrayVec::<Dummy, N_DEALERS>::new();
    init_dealers(&mut dealers, Point::new(16, 16), Point::new(3, 3));


    let dealer_texture = texture_creator.load_texture(DEALER_TEXTURE_PATH)?;
    let dangerous_dealer_texture =
        texture_creator.load_texture(DANGEROUS_DEALER_TEXTURE_PATH)?;

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
                _ => {
                    dummy::manage_cursor_keys(hero, event);
                }
            }
        }
        collision::compute_collisions(hero, &mut dealers);
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
        move_dealers(&mut dealers);
        render_dealers(&mut dealers,
                       &mut canvas,
                       &dealer_texture,
                       &dangerous_dealer_texture)?;
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / MAIN_SPEED));
    }
    Ok(())
}
