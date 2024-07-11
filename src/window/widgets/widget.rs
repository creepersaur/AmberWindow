#![allow(non_snake_case)]
use super::super::*;
use std::fmt::Error;

/// Widget > Widget (Base enum for all widgets).
#[derive(Clone, Debug)]
pub enum Widget {
    Text(Text),
    Button(Button),
    WidgetRow(WidgetRow),
    Slider(Slider),
    DisplayImage(DisplayImage),
    Checkbox(Checkbox),
}

impl Widget {
    pub fn equate(&self, other: &mut Self) -> bool {
        match self {
            Widget::Text(i) => i.equate(other.as_text()),
            Widget::Button(i) => i.equate(other.as_button()),
            Widget::Slider(i) => {
                let x = i.equate(other.as_slider());
                return x
            },
            Widget::DisplayImage(i) => i.equate(other.as_image()),
            Widget::WidgetRow(i) => i.equate(other.as_widget_row()),
            Widget::Checkbox(i) => i.equate(other.as_checkbox()),
            _ => false,
        }
    }
    pub fn as_text(&mut self) -> &mut Text {
        match self {
            Widget::Text(ref mut obj) => Ok(obj),
            _ => Err(Error),
        }
        .unwrap()
    }
    pub fn as_button(&mut self) -> &mut Button {
        match self {
            Widget::Button(ref mut obj) => Ok(obj),
            _ => Err(Error),
        }
        .unwrap()
    }
    pub fn as_widget_row(&mut self) -> &mut WidgetRow {
        match self {
            Widget::WidgetRow(ref mut obj) => Ok(obj),
            _ => Err(Error),
        }
        .unwrap()
    }
    pub fn as_slider(&mut self) -> &mut Slider {
        match self {
            Widget::Slider(ref mut obj) => Ok(obj),
            _ => Err(Error),
        }
        .unwrap()
    }
    pub fn as_image(&mut self) -> &mut DisplayImage {
        match self {
            Widget::DisplayImage(ref mut obj) => Ok(obj),
            _ => Err(Error),
        }
        .unwrap()
    }
    pub fn as_checkbox (&mut self) -> &mut Checkbox {
        match self {
            Widget::Checkbox(ref mut obj) => Ok(obj),
            _ => Err(Error),
        }
        .unwrap()
    }
}

trait Equate {
    fn equate(&self, other: &mut Self) -> bool;
}

impl Equate for Text {
    fn equate(&self, other: &mut Self) -> bool {
        self.rect == other.rect
            && self.color == other.color
            && self.queue_free == other.queue_free
    }
}
impl Equate for Button {
    fn equate(&self, other: &mut Self) -> bool {
        self.rect.w == other.rect.w
            && self.rect.h == other.rect.h
            && self.button_rect.w == other.button_rect.w
            && self.button_rect.h == other.button_rect.h
    }
}
impl Equate for Slider {
    fn equate(&self, other: &mut Self) -> bool {
        self.min == other.min
            && self.max == other.max
    }
}
impl Equate for DisplayImage {
    fn equate(&self, other: &mut Self) -> bool {
        self.texture == other.texture
            && { self.rect.w == other.rect.w && self.rect.h == other.rect.h }
            && self.color == other.color
    }
}
impl Equate for WidgetRow {
    fn equate(&self, other: &mut Self) -> bool {
        let mut idx = 0;
        for i in self.widgets.iter() {
            let equal = match i {
                Widget::Text(i) => i.equate(other.widgets[idx].as_text()),
                Widget::Button(i) => i.equate(other.widgets[idx].as_button()),
                Widget::Slider(i) => i.equate(other.widgets[idx].as_slider()),
                Widget::DisplayImage(i) => i.equate(other.widgets[idx].as_image()),
                _ => true,
            };
            idx += 1;
        }
        return true;
    }
}
impl Equate for Checkbox {
    fn equate(&self, other: &mut Self) -> bool {
        self.rect.w == other.rect.w
            && self.rect.h == other.rect.h
    }
}
