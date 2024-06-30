use super::super::*;
use std::fmt::Error;

#[derive(Clone)]
pub enum Widget {
    Text(Text),
    Button(Button),
    WidgetRow(WidgetRow),
    Slider(Slider),
    DisplayImage(DisplayImage),
}

impl Widget {
    pub fn equate(&self, other: &mut Self) -> bool {
        match self {
            Widget::Text(i) => i.equate(other.as_text()),
            Widget::Button(i) => i.equate(other.as_button()),
            Widget::Slider(i) => i.equate(other.as_slider()),
            Widget::DisplayImage(i) => i.equate(other.as_image()),
            Widget::WidgetRow(i) => i.equate(other.as_widget_row()),
            _ => false,
        }
    }
    pub fn as_text(&mut self) -> &mut Text {
        match self {
            Widget::Text(ref mut obj) => Ok(obj),
            _ => Err(Error),
        }.unwrap()
    }
    pub fn as_button(&mut self) -> &mut Button {
        match self {
            Widget::Button(ref mut obj) => Ok(obj),
            _ => Err(Error),
        }.unwrap()
    }
    pub fn as_widget_row(&mut self) -> &mut WidgetRow {
        match self {
            Widget::WidgetRow(ref mut obj) => Ok(obj),
            _ => Err(Error),
        }.unwrap()
    }
    pub fn as_slider(&mut self) -> &mut Slider {
        match self {
            Widget::Slider(ref mut obj) => Ok(obj),
            _ => Err(Error),
        }.unwrap()
    }
    pub fn as_image(&mut self) -> &mut DisplayImage {
        match self {
            Widget::DisplayImage(ref mut obj) => Ok(obj),
            _ => Err(Error),
        }.unwrap()
    }
    pub fn is_freeing(&self) -> bool {
        match self {
            Widget::Text(obj) => obj.queue_free,
            Widget::Button(obj) => obj.queue_free,
            Widget::WidgetRow(obj) => obj.queue_free,
            Widget::Slider(obj) => obj.queue_free,
            _ => false,
        }
    }
}

trait Equate {
    fn equate(&self, other: &mut Self) -> bool;
}

impl Equate for Text {
    fn equate(&self, other: &mut Self) -> bool {
        self.text == other.text
            && self.rect == other.rect
            && self.color == other.color
            && self.queue_free == other.queue_free
    }
}
impl Equate for Button {
    fn equate(&self, other: &mut Self) -> bool {
        self.text == other.text && self.rect == other.rect && self.button_rect == other.button_rect
    }
}
impl Equate for Slider {
    fn equate(&self, other: &mut Self) -> bool {
        self.text == other.text && self.rect == other.rect
    }
}
impl Equate for DisplayImage {
    fn equate(&self, other: &mut Self) -> bool {
        self.texture == other.texture && self.rect == other.rect && self.color == other.color
    }
}
impl Equate for WidgetRow {
    fn equate(&self, other: &mut Self) -> bool {
        ({
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
        } && self.queue_free == other.queue_free)
    }
}
