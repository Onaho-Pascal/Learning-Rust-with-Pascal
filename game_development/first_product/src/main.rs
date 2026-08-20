use macroquad::prelude::*;

#[macroquad::main("Nitro Base")]

async fn main() {
    const MOVEMENT_SPEED: f32 = 200.0;
    let delta_time = get_frame_time();
    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;
    
    loop {
        clear_background(RED);
        if is_key_down(KeyCode::D) {
            x += MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::A) {
            x -= MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::S) {
            y += MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::W) {
            y -= MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Right) {
            x += MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Left) {
            x -= MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Down) {
            y += MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Up) {
            y -= MOVEMENT_SPEED * delta_time;
        }
        x = clamp(x, 0.0, screen_width());
        y = clamp(y, 0.0, screen_height());

        draw_circle(x, y, 16.0, WHITE);
        next_frame().await
    }
}