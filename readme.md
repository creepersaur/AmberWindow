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