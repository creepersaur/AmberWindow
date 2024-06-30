use macroquad::prelude::*;
mod window; use window::*;

#[macroquad::main("Window")]
async fn main() {
    let font: Font = load_ttf_font("src\\assets\\JetBrainsMono-Medium.ttf")
        .await
        .unwrap();

    let mut show_button = true;
    let mut windows = WindowManager::new();

    loop {
        clear_background(Color::new(0.2, 0.2, 0.2, 1.0));

        if let Some(win) = windows.begin("my_window", &font) {
            win
                .name("sigma").push_widgets(vec![
                    Widget::Button(
                        Button::new("bye", &font, None, None)
                    )
                ]);
            
            if show_button {
                win.push(Widget::Button(
                    Button::new("hello", &font, None, None)
                ));
            }

            if win.get_widget(0).as_button().is_just_pressed {
                show_button = false;
            }
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