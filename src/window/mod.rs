#![allow(unused)]
#![allow(dropping_references)]

use macroquad::prelude::*;
use rand::RandomRange;
mod widgets;
pub use widgets::*;
mod display;
pub use display::*;
mod window_manager;
pub use window_manager::*;