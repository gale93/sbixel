use macroquad::input::{is_mouse_button_down, mouse_position, MouseButton};

use crate::particle::{Particle, ParticleSystem, ParticleType};
use crate::def;

pub struct Spawner;


impl Spawner {
    pub fn new() -> Self {
        Spawner {}
    }

    pub fn handle(&self, system: &mut ParticleSystem) {
        if is_mouse_button_down(MouseButton::Left) {
            let mouse_pos = mouse_position();
            let x = mouse_pos.0 as usize;
            let y = mouse_pos.1 as usize;

            for dy in 0..def::SPAWN_SIZE {
                for dx in 0..def::SPAWN_SIZE {
                    let new_x = x + dx;
                    let new_y = y + dy;

                    system.set_pixel(new_x, new_y, ParticleType::Sand);
                }
            }
        }
        if is_mouse_button_down(MouseButton::Right) {
            let mouse_pos = mouse_position();
            let x = mouse_pos.0 as usize;
            let y = mouse_pos.1 as usize;

            for dy in 0..def::SPAWN_SIZE {
                for dx in 0..def::SPAWN_SIZE {
                    let new_x = x + dx;
                    let new_y = y + dy;

                    system.set_pixel(new_x, new_y, ParticleType::Water);
                }
            }
        }
    }
}