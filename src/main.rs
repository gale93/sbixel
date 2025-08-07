use macroquad::{miniquad::window::{set_window_position, set_window_size}, prelude::*};
mod def;
mod particle;
mod draw_buffer;
mod spawner;
mod physics;

#[macroquad::main("sbixel")]
async fn main()
{
    set_window_size(def::WIDTH as u32, def::HEIGHT as u32);
    set_window_position((screen_width() / 2.0) as u32, (screen_height() / 2.0) as u32);
    let mut delta = 0.0;

    let mut psystem = particle::ParticleSystem::new();
    let mut draw_buffer = draw_buffer::DrawBuffer::new(def::WIDTH, def::HEIGHT);
    let spawner = spawner::Spawner::new();
    
    loop
    {
        clear_background(BLACK);

        if is_key_pressed(KeyCode::A) {
            draw_buffer.draw_sector_overlay = !draw_buffer.draw_sector_overlay;
        }

        spawner.handle(&mut psystem);

        delta += get_frame_time();
        if delta > def::FIXED_UPDATE_INTERVAL {
            delta -= def::FIXED_UPDATE_INTERVAL;

            psystem.analyze();
            draw_buffer.update(&psystem.grid);
        }

        draw_buffer.draw();
        draw_buffer.draw_sector_overlay(&psystem.sectors_active, psystem.sector_size);
        draw_text("Press 'A' to toggle sector overlay", 10.0, 20.0, 20.0, WHITE);
        draw_text("Press Left Mouse Button to spawn Sand", 10.0, 40.0, 20.0, YELLOW);
        draw_text("Press Right Mouse Button to spawn Water", 10.0, 60.0, 20.0, BLUE);
        draw_text(&format!("Fps: {:.2}", get_fps()), 10.0, 80.0, 20.0, WHITE);
        next_frame().await
    }
}