use amberwindow::*;
use macroquad::prelude::*;

#[macroquad::main("Hello")]
async fn main() {
    let mut windows = WindowManager::new();
    windows.set_font("src\\font.ttf").await;

    let mut widget = WindowWidget::new();
    widget.set_font("src\\font.ttf").await;

    let font = windows.font.clone();

    loop {
        clear_background(Color::from_hex(0x738c99));

        if let Some(win) = windows.begin("") {
            win.name("Debug");
            win.style(WindowStyle{
                font: font.clone(),
                bg_color: Color::from_hex(0x151617),
                tb_color: Color::from_hex(0x294a7a),
                border_color: BLANK,
                selected_border_color: BLANK,
                title_color: WHITE,
                scale_color: Color::from_hex(0x294a7a),
                minimize_color: WHITE
            });
            widget.Text(win, "Hello, world 123", None);
            widget.Button(win, "Save");
            widget.Slider(win, 0., 100., vec2(win.rect.w - 20.0, 15f32));
            widget.Checkbox(win, "Auto update", false);
            widget.DisplayImage(
                win,
                Some(Texture2D::from_image(&load_image("src\\assets\\ferris.png").await.unwrap())),
                vec2(50., 35.),
            );
            win.button_style(ButtonStyle{
                font: font.clone(),
                color: WHITE,
                bg_color: Color::from_hex(0x274972),
                hover_bg_color: Color::from_hex(0x496994),
                pressed_bg_color: Color::from_hex(0x274972)
            });
        }

        windows.update_windows();
        next_frame().await;
    }
}
