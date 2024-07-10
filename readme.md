# AmberWindow

'amberwindow' is an easy to use ImmediateMode gui library for Rust.

Uses macroquad as a backend. Inspired by libraries like DearImgui.

## Supported Platforms

- Windows / PC
- Linux (untested)
- MacOS (untested)

## Features

* Easy to setup
* Getting windows working is easy
* Many pre-made widgets to use

# Examples

## Hello Window
```rs
use amberwindow::WindowManager;
use macroquad::prelude::*;

#[macroquad::main("Hello")]
async fn main() {
    let mut windows = WindowManager::new();
    loop {
        windows.begin("");
        windows.update_windows();
        next_frame().await;
    }
}
```

## Hello World

```rs
use amberwindow::{WindowManager, WindowWidget};
use macroquad::prelude::*;

#[macroquad::main("Hello")]
async fn main() {
    let mut windows = WindowManager::new();
    let widget = WindowWidget::new();

    loop {
        if let Some(win) = windows.begin("") {
            widget.Text( win, "Hello world");
        }

        windows.update_windows();
        next_frame().await;
    }
}
```

# Custom Styling

For all of you who love dearimgui's styling, using the custom styling features in AmberWindow can let you "remake" dearimgui.

```rs
if let Some(win) = windows.begin("") {
    win.name("Debug");
    win.style(WindowStyle{
        font: font.clone(),
        bg_color: Color::from_hex(0x151617),
        tb_color: Color::from_hex(0x294a7a),
        deselected_tb_color: BLACK,
        border_color: BLANK,
        selected_border_color: Color::new(1.,1.,1., 0.7),
        title_color: WHITE,
        scale_color: Color::from_hex(0x294a7a),
        minimize_color: WHITE,
        close_color: WHITE
    });
    widget.Text(win, "Hello, world 123", None);
    widget.Button(win, "Save");
    widget.Slider(win, 0., 100., vec2(win.rect.w - 20.0, 15f32));
    widget.Checkbox(win, "Auto update", false);
    win.button_style(ButtonStyle{
        font: font.clone(),
        color: WHITE,
        bg_color: Color::from_hex(0x274972),
        hover_bg_color: Color::from_hex(0x496994),
        pressed_bg_color: Color::from_hex(0x274972)
    });
    win.slider_style(SliderStyle{
        color: WHITE,
        bg_color: Color::from_hex(0x163861),
        hover_bg_color: Color::from_hex(0x274972),
        value_color: SKYBLUE,
    });
    win.get(3).as_checkbox().bg_color = Color::from_hex(0x274972);
}
```

It will make this: https://i.imgur.com/du1M7wV.png
(Without the crab image.)