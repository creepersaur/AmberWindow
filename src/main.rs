use macroquad::prelude::*;
use amberwindow::*;

#[macroquad::main("Hello")]
async fn main() {
    let mut windows = WindowManager::new();
    windows.set_font("src\\font.ttf").await;

    let mut widget = WindowWidget::new();
    widget.set_font("src\\font.ttf").await;

    let font = windows.font.clone();

    loop {
        if let Some(win) = windows.begin("") {
            win.name("Debug");
            widget.Text(win, "Hello, world 123", None);
            widget.WidgetRow(win, vec![
                Widget::Button(Button::new("bruh", font.clone(), None, None))
            ]);
            widget.Slider(win, 0., 100., vec2(win.rect.w-15.0,15f32));
            widget.Checkbox(win, "Auto update", false);
        }
        
        windows.update_windows();
        next_frame().await;
    }
}