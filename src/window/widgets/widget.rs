use std::fmt::Error;
use super::super::*;

#[derive(Clone)]
pub enum Widget
{
    Text(Text),
    Button(Button),
    WidgetRow(WidgetRow),
    Slider(Slider),
    DisplayImage(DisplayImage)
}

impl Widget {
    pub fn as_text(&mut self) -> Result<&mut Text, Error> {
        match self {
            Widget::Text(ref mut obj) => Ok(obj),
            _ => Err(Error)
        }
    }
    pub fn as_button(&mut self) -> Result<&mut Button, Error> {
        match self {
            Widget::Button(ref mut obj) => Ok(obj),
            _ => Err(Error)
        }
    }
    pub fn as_widget_row(&mut self) -> Result<&mut WidgetRow, Error> {
        match self {
            Widget::WidgetRow(ref mut obj) => Ok(obj),
            _ => Err(Error)
        }
    }
    pub fn as_slider(&mut self) -> Result<&mut Slider, Error> {
        match self {
            Widget::Slider(ref mut obj) => Ok(obj),
            _ => Err(Error)
        }
    }
    pub fn as_image(&mut self) -> Result<&mut DisplayImage, Error> {
        match self {
            Widget::DisplayImage(ref mut obj) => Ok(obj),
            _ => Err(Error)
        }
    }
    pub fn is_freeing(&self) -> bool {
        match self {
            Widget::Text(obj) => obj.queue_free,
            Widget::Button(obj) => obj.queue_free,
            Widget::WidgetRow(obj) => obj.queue_free,
            Widget::Slider(obj) => obj.queue_free,
            _ => false
        }
    }
}