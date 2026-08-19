use macroquad::prelude::*;

#[macroquad::main("Nitro Base")]

async fn main() {
    loop {
        clear_background(DARKBLUE);
        next_frame().await
    }
}