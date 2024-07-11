use macroquad::prelude::*;

#[macroquad::main("Hello")]
async fn main() {
    loop {
        next_frame().await;
    }
}