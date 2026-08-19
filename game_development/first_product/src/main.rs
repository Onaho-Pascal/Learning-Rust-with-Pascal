use macroquad::prelude::*;

#[macroquad::main("Nitro Base")]

async fn main() {
    const MOVEMENT_SPEED: f32 = 200.0;
    let mut x = screen_width() / 5.0;
    let mut y = screen_height() / 5.0;
    loop {
        clear_background(DARKBLUE);
        if is_key_down(KeyCode::Right) {
            x += 1.0;
        }
        if is_key_down(KeyCode::Left) {
            x -= 1.0;
        }
        if is_key_down(KeyCode::Down) {
            y += 1.0;
        }
        if is_key_down(KeyCode::Up) {
            y -= 1.0;
        }

        draw_circle(x, y, 16.0, YELLOW);
        next_frame().await
    }
}