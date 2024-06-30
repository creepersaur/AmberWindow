use macroquad::prelude::*;
mod window;
use window::*;

#[macroquad::main("Window")]
async fn main() {
    let font: Font = load_ttf_font("src\\assets\\JetBrainsMono-Medium.ttf")
        .await
        .unwrap();

    let ferris_image = Texture2D::from_image(&load_image("src\\assets\\ferris.png").await.unwrap());
    let ferris_size = ferris_image.size() / 2.0;

    let mut win = begin(Some("amogus"), &font.clone());
    win.push_widgets(vec![
        Widget::Slider(Slider::new(
            "",
            font,
            0.0,
            200.0,
            Rect::new(0., 0., 200., 20.),
            None,
        )),
        Widget::DisplayImage(DisplayImage::new(
            Some(ferris_image),
            ferris_size,
            None,
            None,
        )),
    ]);

    let mut windows = vec![win];

    loop {
        clear_background(Color::new(0.2, 0.2, 0.2, 1.0));
        update_windows(&mut windows);

        let mut windows_clone = windows[0].clone();
        let value = windows_clone.widgets[0].as_slider().unwrap().value;

        windows[0].widgets[1]
            .as_image()
            .unwrap()
            .set_size(vec2(value * 1.7, value));

        next_frame().await;
    }
}
