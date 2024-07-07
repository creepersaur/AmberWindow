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

```
use amberwindow::{WindowManager, WindowWidget};
use macroquad::prelude::*;

#[macroquad::main("Hello")]
async fn main() {
    let mut windows = WindowManager::new();
    let widget = WindowWidget::new();

    loop {
        if let Some(win) = windows.begin("") {
            widget.Text(win, "Hello world");
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