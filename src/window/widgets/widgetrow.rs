use macroquad::prelude::*;

use super::super::*;

/// Widget > WidgetRow (Horizontally places widgets).
/// # WidgetRow
///
/// Allows for placing widgets into horizontally, as opposed to only in rows.
/// **You cannot have WidgetRows inside of WidgetRows**.
#[derive(Clone, Debug)]
pub struct WidgetRow {
    pub widgets: Vec<Widget>,
    pub rect: Rect,
    pub uuid: &'static str,
    pub frame_pushed: Vec<Widget>,
}
impl WidgetRow {
    pub fn new(widgets: &mut Vec<Widget>, uuid: Option<&'static str>) -> Self {
        let mut x = Self {
            frame_pushed: vec![],
            widgets: vec![],
            rect: Rect::new(0., 0., 0., 0.),
            uuid: uuid.unwrap_or(""),
        };

        x.push_widgets(widgets);
        x
    }

    pub fn set_uuid(&mut self, uuid: &'static str) -> &mut Self {
        self.uuid = uuid;
        self
    }

    /// Push multiple widgets to the window.
    pub fn push_widgets(&mut self, widgets: &mut Vec<Widget>) -> &mut Self {
        if widgets.len() < 1 {
            return self;
        }

        for i in widgets.iter_mut() {
            self.push(i);
        }

        self
    }

    /// Get a widget by its index (usize/int).
    pub fn get_widget(&mut self, idx: usize) -> &mut Widget {
        &mut self.widgets[idx]
    }

    // Push a single widget to the window.
    pub fn push(&mut self, widget: &mut Widget) -> &mut Self {
        let mut idx = self.frame_pushed.len();

        if self.widgets.len() < 1 || self.widgets.len() - 1 < idx {
            self.widgets.push(widget.clone())
        } else if widget.equate(&mut self.get_widget(idx)) {
            if let Widget::Text(ref mut widget) = widget {
                self.get_widget(idx).as_text().text = widget.text.clone();
            } else if let Widget::Button(ref mut widget) = widget {
                widget.pressed = self.get_widget(idx).as_button().pressed;
                widget.hovering = self.get_widget(idx).as_button().hovering;
                widget.is_just_pressed = self.get_widget(idx).as_button().is_just_pressed;
            } else if let Widget::Slider(ref mut widget) = widget {
                widget.pressed = self.get_widget(idx).as_slider().pressed;
                widget.hovering = self.get_widget(idx).as_slider().hovering;
                widget.value = self.get_widget(idx).as_slider().value;
            } else if let Widget::Checkbox(ref mut widget) = widget {
                widget.pressed = self.get_widget(idx).as_checkbox().pressed;
                widget.hovering = self.get_widget(idx).as_checkbox().hovering;
                widget.value = self.get_widget(idx).as_checkbox().value;
                widget.is_just_pressed = self.get_widget(idx).as_checkbox().is_just_pressed;
            }

            self.widgets[idx] = widget.clone();
        } else {
            self.widgets[idx] = widget.clone();
        }
        self.frame_pushed.push(widget.clone());

        self
    }

    pub fn update(&mut self, selected: bool, mouse_position: Vec2) {
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
                i.rect.y = self.rect.y + self.rect.h / 4.0;

                last_y += i.rect.w + padding;
            } else if let Widget::Button(i) = i {
                i.rect.x = self.rect.x + padding_left + last_y;
                i.rect.y = self.rect.y;
                i.update(selected, mouse_position);

                if i.button_rect.h > max_height {
                    max_height = i.button_rect.h;
                }

                button_amount += 1.0;

                last_y += i.button_rect.w + 2.0 + padding;
            } else if let Widget::Slider(i) = i {
                i.rect.x = self.rect.x + padding_left;
                i.rect.y = self.rect.y + last_y - 10.0;
                i.update(selected, &mouse_position);

                last_y += i.rect.h + padding + 1.0;
                if i.rect.h > max_height {
                    max_height = i.rect.h;
                }
            } else if let Widget::Checkbox(i) = i {
                i.rect.x = self.rect.x + padding_left;
                i.rect.y = self.rect.y + last_y;
                i.update(selected, mouse_position);

                last_y += i.rect.h + padding + 4.0;
                if i.rect.h > max_height {
                    max_height = i.rect.h;
                }
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
            } else if let Widget::Slider(i) = i {
                i.rect.x = self.rect.x + padding_left;
                i.rect.y = self.rect.y + last_y - 10.0;
                i.render();

                last_y += i.rect.h + padding;
            } else if let Widget::Checkbox(i) = i {
                i.rect.x = self.rect.x + padding_left;
                i.rect.y = self.rect.y + last_y - 10.0;
                i.render();

                last_y += i.rect.h + padding;
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

    /// Set the window's slider' styles.
    pub fn slider_style(&mut self, style: &SliderStyle) -> &mut Self {
        for i in self.widgets.iter_mut() {
            if let Widget::Slider(i) = i {
                i.style = style.clone();
            }
        }
        self
    }
}
