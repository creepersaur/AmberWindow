use amberwindow::WindowManager;
use macroquad::prelude::*;

#[macroquad::main("Hello")]
async fn main() {
    let mut windows = WindowManager::new();

    loop {
        clear_background(DARKGRAY);

        if let Some(win) = windows.begin("id") {
            if let Some(row) = win.WidgetRow().1 {
                row.Text("This text is:", Some(RED));
                row.Button("RED");
                row.Button("Blue");
            }
            if let Some(row) = win.WidgetRow().1 {
                row.Text("This text is:", Some(GREEN));
                row.Button("GREEN");
                row.Button("YELLOW");
            }
            if let Some(row) = win.WidgetRow().1 {
                row.Text("This text is:", Some(PINK));
                row.Button("PURPLE");
                row.Button("PINK");
            }
        }
        windows.update_windows();

        next_frame().await;
    }
}
