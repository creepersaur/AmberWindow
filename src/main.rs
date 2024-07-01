use macroquad::prelude::*;
mod window;
use window::*;

#[macroquad::main("Window")]
async fn main() {
    let font: Font = load_ttf_font("src\\assets\\JetBrainsMono-Medium.ttf")
        .await
        .unwrap();

    let ferris = Texture2D::from_image(&load_image("src\\assets\\ferris.png").await.unwrap());

    let mut windows = WindowManager::new();

    loop {
        clear_background(Color::new(0.2, 0.2, 0.2, 1.0));

        if let Some(win) = windows.begin("my_window", &font.clone()) {
            let win_size = vec2(win.rect.w - 30., win.rect.h -45.);

            win.name("").push_widgets(vec![
                Widget::DisplayImage(DisplayImage::new(
                    Some(ferris.clone()),
                    win_size,
                    None,
                    None,
                )),
            ]);
        }

        windows.update_windows();
        next_frame().await;
    }
}

// if let Some(win) =  {
//     win
//         .name("sigma")
//         .push_widgets(vec![
//             Widget::Text(
//                 Text::new("hello", &font, None, None)
//             )
//         ]);

//     win.widgets[0].as_text().unwrap().set_text(mouse_position().0.to_string());
// }
