use camera::mouse;
use macroquad::prelude::*;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct SliderStyle {
    pub font: Font,
    pub color: Color,
    pub hover_bg_color: Color,
    pub bg_color: Color,
    pub value_color: Color
}

/// Widget > Slider (Simple floating point slider).
#[derive(Clone)]
pub struct Slider {
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
}

impl Slider {
    pub fn new(font: Font, min: f32, max: f32, rect: Rect, uuid: Option<&'static str>) -> Self {
        Self {
            uuid: uuid.unwrap_or(""),
            rect,
            style: SliderStyle {
                font: font,
                color: WHITE,
                hover_bg_color: Color::new(0.3, 0.3, 0.3, 0.5),
                bg_color: Color::new(0.3, 0.3, 0.3, 0.3),
                value_color: GOLD,
            },
            hovering: false,
            pressed: false,
            queue_free: false,
            percentage: 0.0,
            value: min,
            min,
            max
        }
    }
    
    pub fn update(&mut self, selected: bool, mouse_position: &Vec2) {
        if is_mouse_button_released(MouseButton::Left) {
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
            let percentage = (((*mouse_position).x - self.rect.x) / self.rect.w);
            self.value = clamp(percentage * (self.max-self.min), self.min, self.max);
        }

        self.percentage = self.value/self.max;
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
                _ => self.style.bg_color
            }
        );

        // Value
        draw_rectangle(
            self.rect.x,
            self.rect.y,
            self.rect.w * self.value/self.max,
            self.rect.h,
            self.style.value_color
        )
    }

    pub fn style(&mut self, style: SliderStyle) -> &mut Self {
        self.style = style;
        self
    }
}