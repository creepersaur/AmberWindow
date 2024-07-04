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
    let mut windows = WindowManager::new("src\\font.ttf").await;
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
    let font_path = "src\\font.ttf";
    let mut windows = WindowManager::new(font_path).await;
    let widget = WindowWidget::new(font_path).await;

    loop {
        if let Some(win) = windows.begin("") {
            widget.Text( win, "Hello world", None);
        }

        windows.update_windows();
        next_frame().await;
    }
}
```