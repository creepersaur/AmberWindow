use macroquad::prelude::*;
use amberwindow::{WindowManager, WindowWidget};

#[macroquad::main("Window")]
async fn main() {
    let font: Font = load_ttf_font("src\\assets\\JetBrainsMono-Medium.ttf")
        .await
        .unwrap();

    let mut windows = WindowManager::new();

    loop {
        clear_background(Color::new(0.2, 0.2, 0.2, 1.0));

        if let Some(window) = windows.begin("window", &font) {
            window.push(
                WindowWidget::Text("Hello world!", &font, None, None)
            );
        }

        windows.update_windows();
        next_frame().await;
    }
}