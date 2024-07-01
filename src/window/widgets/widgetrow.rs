use macroquad::prelude::*;

use super::super::*;

/// Widget > WidgetRow (Horizontally places widgets).
/// # WidgetRow
/// 
/// Allows for placing widgets into horizontally, as opposed to only in rows.
/// **You cannot have WidgetRows inside of WidgetRows**.
#[derive(Clone)]
pub struct WidgetRow {
    pub widgets: Vec<Widget>,
    pub rect: Rect,
    pub uuid: &'static str,
} impl WidgetRow {
    pub fn new(widgets: Vec<Widget>, uuid: Option<&'static str>) -> Self{
        Self {
            widgets,
            rect: Rect::new(0., 0., 0., 0.),
            uuid: uuid.unwrap_or(""),
        }
    }

    pub fn push(&mut self, value: Widget) {
        self.widgets.push(value);
    }

    pub fn update(&mut self, selected: bool) {
        let mut max_height = 0.0;

        let mut last_y = 0.0;
        let padding = 7.0;
        let padding_left = 0.0;

        let mut button_amount = 0.0;

        for i in self.widgets.iter_mut() {
            if let Widget::Text(i) = i {
                i.rect.x = self.rect.x + padding_left + last_y;
                i.update(selected);

                if i.rect.h > max_height {
                    max_height = i.rect.h;
                }
                i.rect.y = self.rect.y + self.rect.h/4.0;

                last_y += i.rect.w + padding;
            } else if let Widget::Button(i) = i {
                i.rect.x = self.rect.x + padding_left + last_y;
                i.rect.y = self.rect.y;
                i.update(selected);

                if i.button_rect.h > max_height {
                    max_height = i.button_rect.h;
                }

                button_amount += 1.0;

                last_y += i.button_rect.w + 2.0 + padding;
            }
        }

        self.rect.h = max_height;
        if last_y > self.rect.w {
            self.rect.w = last_y - 5.0
        }
        //self.rect.h = max_height + 4.0;
    }

    pub fn render(&mut self) {
        let mut max_height = 0.0;

        let mut last_y = 0.0;
        let padding = 7.0;
        let padding_left = 0.0;

        for i in self.widgets.iter_mut() {
            if let Widget::Text(i) = i {
                i.rect.x = self.rect.x + padding_left + last_y;
                //i.rect.y = self.rect.y;
                i.render();
                
                if i.rect.h > max_height {
                    max_height = i.rect.h;
                    self.rect.h = max_height + 4.0;
                }

                last_y += i.rect.w + padding;
            } else if let Widget::Button(i) = i {
                i.rect.x = self.rect.x + padding_left + last_y;
                //i.rect.y = self.rect.y;
                i.render();
                
                if i.rect.h > max_height {
                    max_height = i.rect.h;
                    self.rect.h = max_height + 4.0;
                }

                last_y += i.button_rect.w + padding;
            }
        }
        
        self.rect.h = max_height + 4.0;
    }

    pub fn button_style(&mut self, style: &ButtonStyle) {
        for i in self.widgets.iter_mut() {
            if let Widget::Button(i) = i {
                i.style = style.clone();
            }
        }
    }

    pub fn get_widget(&mut self, uuid: &str) -> Option<&mut Widget> {
        for i in self.widgets.iter_mut() {
            if let Widget::Text(obj) = i {
                if obj.uuid == uuid { return Some(i) }
            } else if let Widget::Button(obj) = i {
                if obj.uuid == uuid { return Some(i) }
            } else if let Widget::Slider(obj) = i {
                if obj.uuid == uuid { return Some(i) }
            } else if let Widget::WidgetRow(obj) = i {
                if let Some(obj) = obj.get_widget(uuid) {
                   return Some(obj)
                }
            }
        }
        return None
    }
}