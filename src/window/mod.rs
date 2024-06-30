#![allow(unused)]
#![allow(dropping_references)]

mod display;
use std::ops::Deref;

pub use display::*;
mod widgets;
use std::mem::drop;
use macroquad::prelude::*;
pub use widgets::*;

impl Window {
    pub fn name(&mut self, name: &str) -> &mut Self {
        self.name = name.to_owned();
        self
    }

    pub fn position(&mut self, position: Vec2) -> &mut Self {
        self.rect.x = position.x;
        self.rect.y = position.y;
        self
    }

    pub fn size(&mut self, size: Vec2) -> &mut Self {
        self.rect.w = size.x;
        self.rect.h = size.y;
        self
    }

    pub fn style(&mut self, style: WindowStyle) -> &mut Self {
        self.style = style;
        self
    }

    pub fn properties(&mut self, properties: WindowProperties) -> &mut Self {
        self.properties = properties;
        self
    }

    pub fn push_widgets(&mut self, widgets: Vec<Widget>) -> &mut Self {
        for i in widgets {
            self.widgets.push(i);
        }
        self
    }

    pub fn push(&mut self, widget: Widget) -> &mut Self {
        self.widgets.push(widget);
        self
    }

    pub fn button_style(&mut self, style: ButtonStyle) -> &mut Self {
        for i in self.widgets.iter_mut() {
            if let Widget::Button(i) = i {
                i.style = style.clone();
            } else if let Widget::WidgetRow(i) = i {
                i.button_style(&style);
            }
        }
        self
    }
}

pub fn begin(uuid: Option<&str>, font: &Font) -> Window {
    Window::new("Window", Rect::new(0., 0., 200., 200.), &font, None, uuid)
}

pub fn update_single_window(win: &mut Window) -> bool {
    let mouse_position = Vec2::from_tuple(mouse_position());

    if win.queue_free {
        drop(win);
    } else {
        win.update(None, &mouse_position);
        win.render();

        return win.selected
    }
    false
}

pub fn update_windows(windows: &mut Vec<Window>) {
    let mouse_position = Vec2::from_tuple(mouse_position());

    let mut idx: u16 = 0;
    let mut selected: Option<usize> = None;

    windows.retain(|x| !x.queue_free);

    for win in windows.iter_mut() {
        win.update(selected, &mouse_position);

        if win.selected {
            selected = Some(idx.try_into().unwrap());
        }

        idx += 1;
    }
    if let Some(idx) = selected {
        windows.insert(0, windows[idx].clone());
        windows.remove((idx + 1).into());
    }

    let mut reversed = windows.clone();
    reversed.reverse();
    for win in reversed.iter_mut() {
        win.render();
    }
}

pub fn get_window_index(windows: &Vec<Window>, uuid: &str) -> Option<usize> {
    let mut idx = 0;
    for i in windows.iter() {
        if i.uuid == uuid.to_owned() {
            return Some(idx)
        }
        idx += 1;
    }
    None
}