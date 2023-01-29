use crate::{N_DEALERS, dummy, SCREEN_WIDTH, SCREEN_HEIGHT};
use arrayvec::ArrayVec;

use sdl2::rect::Rect;

use crate::dummy::Dummy;
use crate::sound;

pub fn compute_dealers_out(dealers: &mut ArrayVec<Dummy, N_DEALERS>, hero: &mut Dummy) {
    for dealer in dealers {
        if dealer.state < 2 && dealer.position.x > SCREEN_WIDTH as i32 {
            hero.score += 1;
            dealer.state = 2;
            sound::play_out();
        }
        if dealer.state < 2 && dealer.position.x < 0 {
            hero.score += 1;
            dealer.state = 2;
            sound::play_out();
        }
        if dealer.state < 2 && dealer.position.y > SCREEN_HEIGHT as i32 {
            hero.score += 1;
            dealer.state = 2;
            sound::play_out();
        }
        if dealer.state < 2 && dealer.position.y  < 0 {
            hero.score += 1;
            dealer.state = 2;
            sound::play_out();
        }
    }
}

pub fn collision(a: &Dummy, b: &Dummy) -> bool {
    let rect_a = Rect::from_center(a.position,
                                   (a.size.x * a.scale.x) as u32 / 2,
                                   (a.size.y * a.scale.y) as u32 / 2);
    let rect_b = Rect::from_center(b.position,
                                   (b.size.x * b.scale.x) as u32 / 2,
                                   (b.size.y * b.scale.y) as u32 / 2);
    rect_a.has_intersection(rect_b)

}

pub fn compute_collisions(hero: &mut Dummy, dealers: &mut ArrayVec<Dummy, N_DEALERS>) {
    for dealer in &mut *dealers {
        if dealer.collided {
            continue;
        }
        if hero.speed.x == 0 && hero.speed.y == 0 {
            continue;
        }
        if collision(hero, dealer) {
            if dealer.state != 0 {
                hero.lives -= 1;
                sound::play_hurt();
                hero.transition = 100;
                sound::play_fire();
                dealer.transition = 100;
            } else {
                sound::play_hit();
            }
            dealer.collided = true;
            dealer.speed.x = 2 * hero.speed.x;
            dealer.speed.y = 2 * hero.speed.y;
            dealer.movement = dummy::uniform_movement;
        }
    }
    compute_collisions_2(dealers);
    compute_dealers_out(dealers, hero);
}

pub fn compute_collisions_2(dealers: &mut ArrayVec<Dummy, N_DEALERS>) {
    for i in 0..N_DEALERS {
        for j in 0..N_DEALERS {
            if i <= j {
                continue;
            }
            if !dealers[i].collided && !dealers[j].collided {
                continue;
            }
            if collision(&dealers[i], &dealers[j]) {
                sound::play_hit();
                sound::play_fire();
                dealers[i].transition = 100;
                dealers[i].collided = false;
                dealers[i].movement = dummy::random_movement;
                dealers[i].state = 1;
                dealers[j].collided = false;
                dealers[j].movement = dummy::random_movement;
                dealers[j].state = 1;
            }
        }
    }
}
