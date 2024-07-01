use macroquad::prelude::*;
mod window;
use window::{WindowManager, WindowWidget, Widget};

#[macroquad::main("Window")]
async fn main() {
    let font: Font = load_ttf_font("src\\assets\\JetBrainsMono-Medium.ttf")
        .await
        .unwrap();

    let ferris = Texture2D::from_image(&load_image("src\\assets\\ferris.png").await.unwrap());

    let mut windows = WindowManager::new();
    let wid = WindowWidget::Text("Hello", &font, None, None);
    let mut wid_vec: Vec<Widget> = vec![
        WindowWidget::Button("Add something", &font, None, None)
    ];

    loop {
        clear_background(Color::new(0.2, 0.2, 0.2, 1.0));

        if let Some(win) = windows.begin("my_window", &font.clone()) {
            win.name("").push_widgets(wid_vec.clone());
            if win.get_widget(0).as_button().is_just_pressed {
                wid_vec.push(wid.clone());
            }
        }

        windows.update_windows();
        next_frame().await;
    }
}