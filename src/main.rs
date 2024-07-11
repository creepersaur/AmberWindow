use amberwindow::WindowManager;
use macroquad::prelude::*;

#[macroquad::main("Hello")]
async fn main() {
    let mut windows = WindowManager::new();

    loop {
        clear_background(DARKGRAY);

        if let Some(win) = windows.begin("id") {
            if win.Button("Clear").1.is_just_pressed {
                println!("BUTTON");
            }
        }
        windows.update_windows();

        next_frame().await;
    }
}
