use amberwindow::*;
use macroquad::prelude::*;

#[macroquad::main("Hello")]
async fn main() {
    let mut windows = WindowManager::new();
    loop {
        clear_background(Color::new(0.2,0.2,0.2,1.0));

        if let Some(win) = windows.begin("int") {
            win.name("Integer Slider");
            win.Slider_int(0, 100, None, vec2(150.,20.)).0;
        }
        if let Some(win) = windows.begin("float") {
            win.name("Float Slider");
            win.Slider_float(0., 1., None, vec2(150.,20.)).0;
        }
        windows.end_windows();
        next_frame().await;
    }
}
