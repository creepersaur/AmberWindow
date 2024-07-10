use amberwindow::*;
use macroquad::prelude::*;

#[macroquad::main("Hello")]
async fn main() {
    let mut windows = WindowManager::new();
    let mut widgets = WindowWidget::new();

    let mut rect = Rect::new(0.,0.,0.,0.);
    let mut angle = 0.;
    let dist = 75.;

    loop {
        clear_background(DARKGRAY);

        if let Some(win) = windows.begin("") {
            win.name("Debug");
            if widgets.Checkbox(win, "Spin", false).1.value {
                let value = widgets.Slider(win, 0., 1.0, vec2(200., 15.)).1.value;
                println!("{value}");
                angle += value;
            }
        }

        rect.x = screen_width()/2.0 + dist * f32::sin(angle);
        rect.y = screen_height()/2.0 + dist * f32::cos(angle);
        draw_rectangle(
            rect.x,
            rect.y,
            25.,
            25.,
            RED
        );

        windows.update_windows();


        next_frame().await;
    }
}
