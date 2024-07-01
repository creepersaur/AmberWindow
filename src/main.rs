use amberwindow::{WindowManager, WindowWidget};
use macroquad::prelude::*;

#[macroquad::main("Hello")]
async fn main() {
    let font_path = "src\\font.ttf";
    let widget = WindowWidget::new(font_path).await;
    let mut windows = WindowManager::new(font_path).await;

    loop {
        if let Some(win) = windows.begin("") {
            win.name("")
                .push(
                    widget.Text("text", None, None)
                );
        }

        windows.update_windows();
        next_frame().await;
    }
}