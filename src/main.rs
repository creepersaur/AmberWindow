use macroquad::prelude::*;
use amberwindow::*;

#[macroquad::main("Hello")]
async fn main() {
    let font = "src\\font.ttf";
    let mut windows = WindowManager::new(font).await;
    let widget = WindowWidget::new(font).await;

    loop {
        if let Some(win) = windows.begin("") {
            widget.Checkbox(win, "hello", true);
            widget.Button(win, "hi");
            //widget.Checkbox(win, "hello", false);
        }
        
        windows.update_windows();
        next_frame().await;
    }
}