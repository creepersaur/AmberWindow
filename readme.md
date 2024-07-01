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
use macroquad::prelude::*;
use amberwindow::WindowManager;

#[macroquad::main("Window")]
async fn main() {
    let mut windows = WindowManager::new("src\\font.ttf").await;

    loop {
        windows.begin("window")
            .unwrap()
            .name("Hello Window!");

        windows.update_windows();
        next_frame().await;
    }
}
```

## Hello World

```rs
use macroquad::prelude::*;
use amberwindow::{WindowManager, WindowWidget};

#[macroquad::main("Window")]
async fn main() {
    let font_path = "src\\font.ttf";
    let widget = WindowWidget::new(font_path).await;
    let mut windows = WindowManager::new(font_path).await;

    loop {
        if let Some(window) = windows.begin("window") {
            window.push(
                widget.Text("Hello world!", None, None)
            );
        }

        windows.update_windows();
        next_frame().await;
    }
}
```