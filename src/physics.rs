use rand::{Rng};

use crate::particle::{self, ParticleSystem};


pub fn update(x: usize, y: usize, psystem: &mut ParticleSystem) {
    let particle_type = {
        let particle = psystem.grid.get(x, y);
        if let Some(p) = particle {
            p.particle_type.clone()
        } else {
            return;
        }
    };

    match particle_type {
        particle::ParticleType::Sand => {
            if !psystem.is_pixel_processed(x, y+1) && let Some(below) = psystem.grid.get(x, y + 1) {
                if below.particle_type == particle::ParticleType::Empty || below.particle_type == particle::ParticleType::Water {
                    if swap_particles_types(psystem, x, y, x, y + 1) {
                        return;
                    }
                }
            } 
            
            let mut directions: Vec<(i32, i32)> = Vec::new();
            directions.push((0, 0));

            if x > 0 && !psystem.is_pixel_processed(x - 1, y + 1) && let Some(left) = psystem.grid.get(x - 1, y + 1) {
                if left.particle_type == particle::ParticleType::Empty || left.particle_type == particle::ParticleType::Water {
                    directions.push((-1, 1));
                }
            }

            if !psystem.is_pixel_processed(x + 1, y + 1) && let Some(right) = psystem.grid.get(x + 1, y + 1) {
                if right.particle_type == particle::ParticleType::Empty || right.particle_type == particle::ParticleType::Water {
                    directions.push((1, 1));
                }
            }

            if !directions.is_empty() {
                let (dx, dy) = directions[rand::rng().random_range(0..directions.len())];
                if swap_particles_types(psystem, (x as i32 + dx) as usize, (y as i32 + dy) as usize, x, y) {
                    return;
                }
            }

        }
        particle::ParticleType::Water => {
            let mut directions: Vec<(i32, i32)> = Vec::new();

            if !psystem.is_pixel_processed(x, y + 1) && let Some(below) = psystem.grid.get(x, y + 1) {
                if below.particle_type == particle::ParticleType::Empty {
                    swap_particles_types(psystem, x, y, x, y + 1);
                    return;
                }
            }


            // left and right
            if let Some(left) = psystem.grid.get(x - 1, y) {
                if left.particle_type == particle::ParticleType::Empty  {
                    directions.push((-1, 0));
                }
            } 
            if let Some(right) = psystem.grid.get(x + 1, y) {
                if right.particle_type == particle::ParticleType::Empty  {
                    directions.push((1, 0));
                }
            }

            if !directions.is_empty() {
                let (dx, dy) = directions[rand::rng().random_range(0..directions.len())];
                swap_particles_types(psystem, (x as i32 + dx) as usize, (y as i32 + dy) as usize, x, y);
                return;
            }

            // below left and below right
            if x > 0 && !psystem.is_pixel_processed(x - 1, y + 1) && let Some(left) = psystem.grid.get(x - 1, y + 1) {
                if left.particle_type == particle::ParticleType::Empty  {
                    directions.push((-1, 1));
                }
            }

            if !psystem.is_pixel_processed(x + 1, y + 1) && let Some(right) = psystem.grid.get(x + 1, y + 1) {
                if right.particle_type == particle::ParticleType::Empty  {
                    directions.push((1, 1));
                }
            }

            if !directions.is_empty() {
                let (dx, dy) = directions[rand::rng().random_range(0..directions.len())];
                if swap_particles_types(psystem, (x as i32 + dx) as usize, (y as i32 + dy) as usize, x, y) {
                    return;
                }
            }
        }
        _ => {}
    }
}


fn swap_particles_types(psystem: &mut ParticleSystem, x: usize, y: usize, x2: usize, y2: usize) -> bool{
    if psystem.is_pixel_processed(x, y) || psystem.is_pixel_processed(x2, y2) {
        false;
    }

    let particle1 = psystem.grid.get(x, y).unwrap().particle_type.clone();
    let particle2 = psystem.grid.get(x2, y2).unwrap().particle_type.clone();

    psystem.set_pixel(x, y, particle2);
    psystem.set_pixel(x2, y2, particle1);

    true
}