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
    pub font: Option<Font>,
    pub remaining: f32,
    pub window_width: f32,
}

// BASE IMPL
impl WidgetRow {
    pub fn new(font: Option<Font>, uuid: Option<&'static str>, window_width: f32) -> Self {
        Self {
            font,
            frame_pushed: vec![],
            widgets: vec![],
            rect: Rect::new(0., 0., 0., 0.),
            uuid: uuid.unwrap_or(""),
            remaining: 0f32,
            window_width
        }
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

    /// Get a widget by its index (usize/int)
    pub fn get(&mut self, idx: usize) -> &mut Widget {
        &mut self.widgets[idx]
    }

    // Push a single widget to the window.
    pub fn push(&mut self, widget: &mut Widget) -> usize {
        let mut idx = self.frame_pushed.len();

        if self.widgets.len() < 1 || self.widgets.len() - 1 < idx {
            self.widgets.push(widget.clone());
        } else if matches!(widget.clone(), self_Clone) && widget.equate(&mut self.get_widget(idx)) {
            if let Widget::Text(ref mut widget) = widget {
                self.get_widget(idx).as_text().text = widget.text.clone();
            } else if let Widget::Button(ref mut widget) = widget {
                let obj = self.get_widget(idx).as_button();
                widget.pressed = obj.pressed;
                widget.hovering = obj.hovering;
                widget.is_just_pressed = obj.is_just_pressed;
            } else if let Widget::Slider(ref mut widget) = widget {
                let obj = self.get_widget(idx).as_slider();
                widget.pressed = obj.pressed;
                widget.hovering = obj.hovering;
                widget.value = obj.value;
            } else if let Widget::Checkbox(ref mut widget) = widget {
                let obj = self.get_widget(idx).as_checkbox();
                widget.pressed = obj.pressed;
                widget.hovering = obj.hovering;
                widget.value = obj.value;
                widget.is_just_pressed = obj.is_just_pressed;
            }

            self.widgets[idx] = widget.clone();
        } else {
            self.widgets.push(widget.clone());
        }
        self.frame_pushed.push(widget.clone());

        self.update(true, vec2(-100.,-100.), false);
        self.remaining = self.window_width - self.rect.w - 5.;

        idx
    }

    pub fn update(&mut self, selected: bool, mouse_position: Vec2, mouse_released: bool) {
        let mut max_height = 5.0;

        let mut last_y = 0.0;
        let padding = 5.0;
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
                i.update(selected, mouse_position, mouse_released);

                if i.button_rect.h > max_height {
                    max_height = i.button_rect.h;
                }

                button_amount += 1.0;

                last_y += i.button_rect.w + 2.0 + padding;
            } else if let Widget::Slider(i) = i {
                i.rect.x = self.rect.x + last_y + padding_left;
                i.rect.y = self.rect.y - i.rect.h / 2.8;
                i.update(selected, &mouse_position, mouse_released);

                last_y += i.rect.w;
            } else if let Widget::Checkbox(i) = i {
                i.rect.x = self.rect.x + padding_left;
                i.rect.y = self.rect.y + last_y;
                i.update(selected, mouse_position, mouse_released);

                last_y += i.rect.w + padding + 4.0;
                if i.rect.h > max_height {
                    max_height = i.rect.h;
                }
            }
        }

        self.rect.h = max_height;
        if last_y > self.rect.w {
            self.rect.w = last_y + 5.0;
        }
        //self.rect.h = max_height + 4.0;
    }

    pub fn render(&mut self) {
        let mut max_height = 0.0;

        let mut last_y = 0.0;
        let padding = -2.0;
        let padding_left = 0.0;

        for i in self.widgets.iter_mut() {
            if let Widget::Text(i) = i {
                i.rect.x = self.rect.x + padding_left + last_y;
                i.rect.y = self.rect.y + i.rect.h/2.;
                i.render();
                i.rect.y -= i.rect.h/2.;

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
                i.rect.x = self.rect.x + last_y + padding_left;
                i.rect.y = self.rect.y - i.rect.h / 2.8;
                i.rect.w += 3.0;
                i.render();
                i.rect.w -= 3.0;

                last_y += i.rect.w + padding;
            } else if let Widget::Checkbox(i) = i {
                i.rect.x = self.rect.x + padding_left;
                i.rect.y = self.rect.y + last_y - 10.0;
                i.render();

                last_y += i.rect.w + padding;
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

// WIDGETS
impl WidgetRow {
    /// Push a `Text` widget to a window. Returns the index and a CLONE of the object.
    pub fn Text(&mut self, text: &str, color: Option<Color>) -> (usize, Text) {
        let mut x = Widget::Text(Text::new(text, self.font.clone(), color, None));

        self.push(&mut x.clone());
        (self.widgets.len() - 1, x.as_text().clone())
    }

    /// Push a `Button` widget to a window. Returns the index and a CLONE of the object.
    pub fn Button(&mut self, text: &str) -> (usize, Button) {
        let mut x = Widget::Button(Button::new(text, self.font.clone(), None, None));

        self.push(&mut x.clone());
        (self.widgets.len() - 1, x.as_button().clone())
    }

    
    /// Push a `Slider_float` widget to the window. Returns the index and a CLONE of the object.
    pub fn Slider_float(&mut self, min: f32, max: f32, default: Option<f32>, size: Vec2) -> (f32, f32) {
        //&mut Slider {
        let mut x = Widget::Slider(Slider::new(
            self.font.clone(),
            min,
            max,
            default,
            size,
            false,
            None,
        ));

        let idx = self.push(&mut x.clone());
        let slider = self.get(idx).as_slider();
        (
            slider.value,
            slider.percentage
        )
    }

    /// Push a `Slider_int` widget to the window. Returns the index and a CLONE of the object.
    pub fn Slider_int(&mut self, min: i32, max: i32, default: Option<i32>, size: Vec2) -> (i32, f32) {
        let mut default_value: Option<f32> = None;
        if default_value.is_some() {
            default_value = Some(default_value.unwrap_or(0.0) as f32);
        }
        
        let mut x = Widget::Slider(Slider::new(
            self.font.clone(),
            min as f32,
            max as f32,
            default_value,
            size,
            true,
            None,
        ));

        let idx = self.push(&mut x.clone());
        let slider = self.get(idx).as_slider();
        (
            slider.value.round() as i32,
            slider.percentage
        )
    }

    /// Push a `DisplayImage` widget to a window. Returns the index and a CLONE of the object.
    pub fn DisplayImage(
        &mut self,
        win: &mut Window,
        texture: Option<Texture2D>,
        size: Vec2,
    ) -> (usize, DisplayImage) {
        let mut x = Widget::DisplayImage(DisplayImage::new(texture, size, None, None));

        self.push(&mut x.clone());
        (self.widgets.len() - 1, x.as_image().clone())
    }

    /// Push a `Checkbox` widget to a window. Returns the index and a CLONE of the object.
    pub fn Checkbox(&mut self, text: &str, ticked: bool) -> (usize, Checkbox) {
        let mut x = Widget::Checkbox(Checkbox::new(
            text,
            self.font.clone(),
            Some(ticked),
            None,
            None,
        ));

        self.push(&mut x.clone());
        (self.widgets.len() - 1, x.as_checkbox().clone())
    }
}
