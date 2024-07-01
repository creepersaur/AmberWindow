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
    let font: Font = load_ttf_font("pathToFont").await.unwrap();
    let mut windows = WindowManager::new();

    loop {
        windows.begin("window", &font).unwrap()
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
    let font: Font = load_ttf_font("pathToFont").await.unwrap();
    let mut windows = WindowManager::new();

    loop {
        if let Some(window) = windows.begin("window", &font) {
            window.push(
                WindowWidget::Text("Hello world!", &font, None, None)
            );
        }

        windows.update_windows();
        next_frame().await;
    }
}
```