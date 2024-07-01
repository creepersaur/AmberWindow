// use macroquad::prelude::*;
// use amberwindow::{WindowManager, WindowWidget};

// #[macroquad::main("Window")]
// async fn main() {
//     let font: Font = load_ttf_font("src\\assets\\JetBrainsMono-Medium.ttf")
//         .await
//         .unwrap();

//     let mut windows = WindowManager::new();

//     loop {
//         clear_background(Color::new(0.2, 0.2, 0.2, 1.0));

//         if let Some(win) = windows.begin("my_window", &font) {
//             win.push(
//                 WindowWidget::Button("press me", &font, None, None)
//             );
    
//             if win.get_widget(0).as_button().is_just_pressed {
//                 println!("BUTTON WAS JUST PRESSED!");
//             }
//         }

//         windows.update_windows();
//         next_frame().await;
//     }
// }

fn main() {}