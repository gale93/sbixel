use crate::{def, particle};
use crate::physics;


#[derive(Debug, Clone, PartialEq)]
pub enum ParticleType {
    Empty,
    Sand,
    Water
}


#[derive(Debug, Clone)]
pub struct Particle {
    pub particle_type: ParticleType,
}


#[derive(Debug)]
pub struct ParticleGrid {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Particle>,
}

impl ParticleGrid {
    pub fn new(width: usize, height: usize, default: ParticleType) -> Self {
        let data = vec![Particle { particle_type: default }; width * height];
        Self { width, height, data }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Particle> {
        if self.is_valid_position(x, y) {
            Some(&self.data[y * self.width + x])
        } else {
            None
        }
    }

    pub fn set(&mut self, x: usize, y: usize, value: ParticleType) {
        if self.is_valid_position(x, y) {
            self.data[y * self.width + x].particle_type = value;
        }
        else {
            println!("Attempted to set particle at invalid position ({}, {})", x, y);
        }
    }

    fn is_valid_position(&self, x: usize, y: usize) -> bool {
        x < def::WIDTH && y < def::HEIGHT
    }

}

pub struct ParticleSystem {
    pub sector_size: usize,
    pub sectors_active: Vec<bool>,
    pub grid: ParticleGrid,
    pixel_processed: Vec<bool>,
}

impl ParticleSystem {
    pub fn new() -> Self {
        Self {
            sector_size: def::SECTOR_SIZE,
            sectors_active: vec![false; def::WIDTH / def::SECTOR_SIZE * def::HEIGHT / def::SECTOR_SIZE],
            grid: ParticleGrid::new(def::WIDTH, def::HEIGHT, ParticleType::Empty),
            pixel_processed: vec![false; def::WIDTH * def::HEIGHT],
        }
    } 
    
    pub fn analyze(&mut self) {
        self.pixel_processed.fill(false);

        for i in 0..self.sectors_active.len() {
            if self.sectors_active[i] {
                self.sectors_active[i] = self.analyze_sector(i);
            }
        }
    }

    fn set_sector_active(&mut self, x: usize, y: usize) {
        let total_width = def::WIDTH / self.sector_size;
        let sector_index = (y / self.sector_size) * total_width + (x / self.sector_size);
        if sector_index < self.sectors_active.len() && !self.sectors_active[sector_index] {
            self.sectors_active[sector_index] = true;
            println!("Sector {} is now active", sector_index);
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, particle_type: ParticleType){   
        self.set_sector_active(x, y);
        self.set_processed_pixel(x, y);
        self.grid.set(x, y, particle_type);
    }

    fn get_sector_coordinates(&self, sector_index: usize) -> (usize, usize) {
        let total_width = def::WIDTH / self.sector_size;
        let x = (sector_index % total_width) * self.sector_size;
        let y = (sector_index / total_width) * self.sector_size;
        (x, y)
    }

    fn set_processed_pixel(&mut self, x: usize, y: usize) {
        if self.grid.is_valid_position(x, y) {
            self.pixel_processed[y * def::WIDTH + x] = true;
        }
    }

    pub fn is_pixel_processed(&self, x: usize, y: usize) -> bool {
        if self.grid.is_valid_position(x, y) {
            self.pixel_processed[y * def::WIDTH + x]
        } else {
            false
        }
    }

    fn analyze_sector(&mut self, sector_index: usize) -> bool {
        let (start_x, start_y) = self.get_sector_coordinates(sector_index);
let end_x = (start_x + self.sector_size + 1).min(def::WIDTH);
let end_y = (start_y + self.sector_size + 1).min(def::HEIGHT);  

        let mut active = false;
        
        for y in (start_y..end_y).rev() {
            for x in start_x..end_x {
                if self.is_pixel_processed(x, y) {
                    continue;
                }

                if let Some(particle) = self.grid.get(x, y) {
                    if particle.particle_type != ParticleType::Empty {
                        physics::update(x, y, self);
                        active = true;
                    }
                }
            }
        }
        if !active {
            println!("Sector {} is inactive", sector_index);
        }
        active
    }
}


