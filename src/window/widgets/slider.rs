use std::default;

use camera::mouse;
use macroquad::prelude::*;

/// Style > Custom Slider Styling.
#[derive(Clone, Debug)]
pub struct SliderStyle {
    pub color: Color,
    pub hover_bg_color: Color,
    pub bg_color: Color,
    pub value_color: Color,
}

/// Widget > Slider (Simple floating point slider).
#[derive(Clone, Debug)]
pub struct Slider {
    pub font: Option<Font>,
    pub rect: Rect,
    pub hovering: bool,
    pub pressed: bool,
    pub style: SliderStyle,
    pub uuid: &'static str,
    pub min: f32,
    pub max: f32,
    pub queue_free: bool,
    pub value: f32,
    pub percentage: f32,
    integer_only: bool
}

impl Slider {
    /// Create a new slider.
    pub fn new(
        font: Option<Font>,
        min: f32,
        max: f32,
        default: Option<f32>,
        size: Vec2,
        integer_only: bool,
        uuid: Option<&'static str>,
    ) -> Self {
        Self {
            integer_only,
            font: font.clone(),
            uuid: uuid.unwrap_or(""),
            rect: Rect::new(0., 0., size.x, size.y),
            style: SliderStyle {
                color: WHITE,
                hover_bg_color: Color::new(0.3, 0.3, 0.3, 0.5),
                bg_color: Color::new(0.3, 0.3, 0.3, 0.3),
                value_color: GOLD,
            },
            hovering: false,
            pressed: false,
            queue_free: false,
            percentage: 0.0,
            value: default.unwrap_or(min),
            min,
            max,
        }
    }

    pub fn set_uuid(&mut self, uuid: &'static str) -> &mut Self {
        self.uuid = uuid;
        self
    }

    pub fn update(&mut self, selected: bool, mouse_position: &Vec2, mouse_released: bool) {
        if mouse_released {
            self.pressed = false;
        }

        if self.rect.contains(*mouse_position) {
            self.hovering = true;
            if is_mouse_button_pressed(MouseButton::Left) && selected {
                self.pressed = true;
            }
        } else {
            self.hovering = false;
        }

        if self.pressed {
            let slider_button_width = self.rect.w / 10.0;
            let percentage = (((*mouse_position).x - self.rect.x) / self.rect.w);
            self.value = clamp(percentage * (self.max - self.min), self.min, self.max);
            if self.integer_only {
                self.value = self.value
            }
        }

        self.percentage = self.value / self.max;
    }

    pub fn render(&mut self) {
        // BG
        draw_rectangle(
            self.rect.x,
            self.rect.y,
            self.rect.w,
            self.rect.h,
            match self.hovering {
                true => self.style.hover_bg_color,
                _ => self.style.bg_color,
            },
        );

        // Value
        draw_rectangle(
            self.rect.x,
            self.rect.y,
            f32::clamp(self.rect.w * self.value / self.max, 0., self.rect.w),
            self.rect.h,
            match self.pressed {
                false => {
                    Color::from_vec(self.style.value_color.to_vec() - vec4(0., 0.13, 0., 0.82))
                }
                true => {
                    Color::from_vec(self.style.value_color.to_vec() - vec4(0.13, 0.3, 0.13, 0.82))
                }
            },
        );

        // SliderButton
        let slider_button_width = self.rect.w / 30.0 + 2.0;

        draw_rectangle(
            clamp(
                self.rect.x + self.rect.w * self.value / self.max - slider_button_width / 2.0,
                self.rect.x,
                self.rect.x + self.rect.w - slider_button_width,
            ),
            self.rect.y,
            slider_button_width,
            self.rect.h,
            match self.pressed {
                false => {
                    Color::from_vec(self.style.value_color.to_vec() - vec4(0.1, 0.1, 0.1, 0.22))
                }
                true => {
                    Color::from_vec(self.style.value_color.to_vec() - vec4(0.1, 0.1, 0.1, 0.42))
                }
            },
        );

        // Text
        let mut text = self.value.to_string();
        if self.integer_only {
            text = (self.value as i32).to_string();
        }
        text = format!("{:.4}", text);

        let font_size = 16;
        let dim = measure_text(&text.to_string(), None, 16, 1f32);
        let dim_some = measure_text(&text.to_string(), self.font.as_ref(), 16, 1f32);

        let height_diff = dim.height / dim_some.height;

        draw_text_ex(
            &text,
            f32::floor(self.rect.x + self.rect.w / 2.0 - dim.width / 2.0),
            f32::floor(self.rect.y + self.rect.h / 2.0 + dim.height / 2.0 + 1.),
            TextParams {
                font: self.font.as_ref(),
                font_size: 16,
                color: self.style.color,
                font_scale: height_diff,
                ..Default::default()
            },
        );
    }

    pub fn style(&mut self, style: SliderStyle) -> &mut Self {
        self.style = style;
        self
    }
}
