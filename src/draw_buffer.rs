use macroquad::{self, color::*, shapes::draw_rectangle_lines, texture::draw_texture};

use crate::{
    def,
    particle::{ParticleGrid, ParticleType},
};

pub struct DrawBuffer {
    image: macroquad::texture::Image,
    texture: macroquad::texture::Texture2D,

    pub draw_sector_overlay: bool,
}

impl DrawBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        let image = macroquad::texture::Image::gen_image_color(width as u16, height as u16, BLACK);
        let texture = macroquad::texture::Texture2D::from_image(&image);
        Self { image, texture, draw_sector_overlay: true }
    }

    pub fn draw(&mut self) {
        self.texture.update(&self.image);
        draw_texture(&self.texture, 0.0, 0.0, WHITE);
    }

    pub fn update(&mut self, grid: &ParticleGrid) {
        for y in 0..def::HEIGHT {   
            for x in 0..def::WIDTH {
                if let Some(particle) = grid.get(x, y) {
                    let color = match particle.particle_type {
                        ParticleType::Empty => BLACK,
                        ParticleType::Sand => YELLOW,
                        ParticleType::Water => BLUE,
                    };
                    self.image.set_pixel(x as u32, y as u32, color);
                }
            }
        }
    }

    pub fn draw_sector_overlay(&self, sectors_active: &[bool], sector_size: usize) {
        if !self.draw_sector_overlay {
            return;
        }
        let sectors_x = def::WIDTH / sector_size;
        let sectors_y = def::HEIGHT / sector_size;

        for sy in 0..sectors_y {
            for sx in 0..sectors_x {
                let index = sy * sectors_x + sx;
                let active = sectors_active.get(index).copied().unwrap_or(false);

                let x = (sx * sector_size) as f32;
                let y = (sy * sector_size) as f32;
                let size = sector_size as f32;
                let color = if active { GREEN } else { RED };

                draw_rectangle_lines(x, y, size, size, 1.0, color);
            }
        }
    }

}