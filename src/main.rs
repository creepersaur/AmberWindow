use amberwindow::*;
use macroquad::prelude::*;

#[macroquad::main("Hello")]
async fn main() {
    let mut windows = WindowManager::new();
    loop {
        clear_background(Color::new(0.2,0.2,0.2,1.0));

        if let Some(win) = windows.begin("no_style") {
            win.name("Debug");
            win.Text(format!("FPS: {}", get_fps()).as_str(), None);
            if let Some(row) = win.WidgetRow() {
                row.Text("Style:", None);
                row.Button(">");
                row.Text("Glossy", None);
                row.Button("<");
            }
            if let Some(row) = win.WidgetRow() {
                row.Text("Add:", None);
                row.Button("Cube");
                row.Button("Sphere");
                row.Button("Cylinder");
            }
            if let Some(row) = win.WidgetRow() {
                row.Button("Pyramid");
                row.Button("Torus");
                row.Button("Wedge");
            }
            if let Some(row) = win.WidgetRow() {
                row.Text("Reset: ", None);
                row.Button("Reset Camera");
                row.Button("Delete all");
            }
            if let Some(row) = win.WidgetRow() {
                row.Text("Camera speed:", None);
                row.Slider(0., 5., 0.5, vec2(row.remaining, 16.));
            }
        }
        windows.update_windows();
        next_frame().await;
    }
}
