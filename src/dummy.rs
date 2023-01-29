use rand::Rng;

use sdl2::rect::Point;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Texture;
use sdl2::render::Canvas;
use sdl2::rect::Rect;

use crate::SCREEN_WIDTH;
use crate::SCREEN_HEIGHT;

const DIRECTION_N: i32 = 0;
const DIRECTION_S: i32 = 1;
const DIRECTION_W: i32 = 2;
const DIRECTION_E: i32 = 3;

pub struct Dummy {
    pub position: Point,
    pub speed: Point,
    pub scale: Point,
    pub size: Point,
    pub n_direction: i32,
    pub direction: i32,
    pub animation: i32,
    pub n_animation: i32,
    pub collided: bool,
    pub state: i32,
    pub movement: fn(&mut Dummy),
    pub transition: i32,
    pub n_transition: i32,
    pub texture_size: Point,
    pub lives: i32,
    pub score: i32
}

impl Default for Dummy {
    fn default() -> Self {
        Self {
            position: Point::new(100, 100),
            speed: Point::new(0, 0),
            scale: Point::new(3, 3),
            size: Point::new(16, 24),
            n_direction: 4,
            direction: 0,
            animation: 0,
            n_animation: 9,
            transition: 0,
            n_transition: 3,
            collided: false,
            state: 0,
            movement: arrow_movement,
            texture_size: Point::new(16, 32),
            lives: 5,
            score: 0
        }
    }
}

pub fn render_dummy(o: &mut Dummy,
                    canvas: &mut Canvas<sdl2::video::Window>,
                    texture: &Texture,
                    dangerous_texture: &Texture,
                    fire_texture: &Texture) -> Result<(), String> {
    let src_ox = o.direction * o.size.x;
    let src_oy = (o.animation % o.n_animation) * o.size.y;
    let src = Rect::new(src_ox, src_oy,
                        o.size.x as u32, o.size.y as u32);
    let dst = Rect::from_center(
        o.position,
        (o.scale.x as u32) * (o.size.x as u32),
        (o.scale.y as u32) * (o.size.y as u32));
    if o.state == 0 {
        canvas.copy_ex(texture, src, dst, 0.0, None, false, false)?;
    } else {
        canvas.copy_ex(dangerous_texture, src, dst, 0.0, None, false, false)?;
    }
    if o.transition > 0 {
        let fire_ox = 0;
        let fire_oy = (o.transition % o.n_transition) * o.texture_size.y;
        let fire_src = Rect::new(fire_ox, fire_oy, o.texture_size.x as u32, o.texture_size.y as u32);
        canvas.copy_ex(fire_texture, fire_src, dst, 0.0, None, false, false)?;
        o.transition -= 1;
    }
    Ok(())
}

pub fn manage_cursor_keys(o: &mut Dummy, event: sdl2::event::Event) {
    match event {
        Event::KeyDown { keycode: Some(Keycode::Up), repeat: false, .. } => {
            o.direction = DIRECTION_N;
            o.speed.y = -1;
        },
        Event::KeyDown { keycode: Some(Keycode::Down), repeat: false, .. } => {
            o.direction = DIRECTION_S;
            o.speed.y = 1;
        },
        Event::KeyDown { keycode: Some(Keycode::Left), repeat: false, .. } => {
            o.direction = DIRECTION_W;
            o.speed.x = -1;
        },
        Event::KeyDown { keycode: Some(Keycode::Right), repeat: false, .. } => {
            o.direction = DIRECTION_E;
            o.speed.x = 1;
        },
        Event::KeyUp { keycode: Some(Keycode::Up), repeat: false, .. } => {
            o.speed.y = 0;
        },
        Event::KeyUp { keycode: Some(Keycode::Down), repeat: false, .. } => {
            o.speed.y = 0;
        },
        Event::KeyUp { keycode: Some(Keycode::Left), repeat: false, .. } => {
            o.speed.x = 0;
        },
        Event::KeyUp { keycode: Some(Keycode::Right), repeat: false, .. } => {
            o.speed.x = 0;
        },
        _ => {}
    }
}

pub fn change_position(o: &mut Dummy) {
    o.position.x += o.scale.x * o.speed.x;
    o.position.y += o.scale.y * o.speed.y;
}

pub fn random_movement(o: &mut Dummy) {
    if o.speed.x != 0 || o.speed.y != 0 {
        o.animation += 1;
    }

    let mut boundary: bool = false;

    if o.position.x - o.scale.x * o.size.x / 2 <= 0 {
        o.position.x += 1;
        o.speed.x *= -1;
        o.direction = DIRECTION_E;
        boundary = true;
    }
    if o.position.x + o.scale.x * o.size.x / 2 >= SCREEN_WIDTH as i32 {
        o.position.x += -1;
        o.speed.x *= -1;
        o.direction = DIRECTION_W;
        boundary = true;
    }
    if o.position.y - o.scale.y * o.size.y / 2 <= 0 {
        o.position.y += 1;
        o.speed.y *= -1;
        o.direction = DIRECTION_S;
        boundary = true;
    }
    if o.position.y + o.scale.y * o.size.y / 2 >= SCREEN_HEIGHT as i32 {
        o.position.y += -1;
        o.speed.y *= -1;
        o.direction = DIRECTION_N;
        boundary = true;
    }

    if rand::thread_rng().gen_range(0..=100) > 98 && !boundary {
        o.direction = rand::thread_rng().gen_range(0..o.n_direction);
    }

    if rand::thread_rng().gen_range(0..=10) > 8 {
        o.animation += 1;
        if o.direction == DIRECTION_N {
            o.speed.x = 0;
            o.speed.y = -1;
        }
        if o.direction == DIRECTION_S {
            o.speed.x = 0;
            o.speed.y = 1;
        }
        if o.direction == DIRECTION_W {
            o.speed.x = -1;
            o.speed.y = 0;
        }
        if o.direction == DIRECTION_E {
            o.speed.x = 1;
            o.speed.y = 0;
        }
        change_position(o);
    }
}

pub fn uniform_movement(o: &mut Dummy) {
    o.animation += 1;
    change_position(o);
}

pub fn arrow_movement(o: &mut Dummy) {
    change_position(o);
    if o.speed.x != 0 || o.speed.y != 0 {
        o.animation += 1;
    }
    if o.position.x - o.scale.x * o.size.x / 2 <= 0 {
        o.position.x += o.scale.x + 1;
    }
    if o.position.x + o.scale.x * o.size.x / 2 >= SCREEN_WIDTH as i32 {
        o.position.x -= o.scale.x + 1;
    }
    if o.position.y - o.scale.y * o.size.y / 2 <= 0 {
        o.position.y += o.scale.y + 1;
    }
    if o.position.y + o.scale.y * o.size.y / 2 >= SCREEN_HEIGHT as i32  {
        o.position.y -= o.scale.y + 1;
    }
}
