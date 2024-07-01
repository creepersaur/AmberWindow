#![allow(non_snake_case)]
/*!
'amberwindow' is an easy to use ImmediateMode gui library for Rust.

Uses macroquad as a backend.

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
```
use macroquad::prelude::*;
use amberwindow::WindowManager;

#[macroquad::main("Window")]
async fn main() {
    let font: Font = load_ttf_font("pathToFont")
        .await
        .unwrap();

    let mut windows = WindowManager::new();

    loop {
        clear_background(Color::new(0.2, 0.2, 0.2, 1.0));

        windows.begin("window", &font);
        windows.update_windows();

        next_frame().await;
    }
}
```

## Hello World

```
use macroquad::prelude::*;
use amberwindow::{WindowManager, WindowWidget};

#[macroquad::main("Window")]
async fn main() {
    let font: Font = load_ttf_font("pathToFont")
        .await
        .unwrap();

    let mut windows = WindowManager::new();

    loop {
        clear_background(Color::new(0.2, 0.2, 0.2, 1.0));

        if let Some(window) = windows.begin("window", &font) {
            window.push(
                WindowWidget::Text("hello", &font, None, None)
            );
        }

        windows.update_windows();
        next_frame().await;
    }
}
```
*/

#![allow(unused)]
#![allow(dropping_references)]

use macroquad::prelude::*;
mod window;
pub use window::*;
// {Widget, WindowManager, WindowWidget, Window, WindowStyle, ButtonStyle, SliderStyle}