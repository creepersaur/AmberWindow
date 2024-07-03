use macroquad::prelude::*;
use std::sync::{Arc, Mutex};

/// Style > Custom Button styling.
#[derive(Clone)]
pub struct ButtonStyle {
    pub font: Font,
    pub color: Color,
    pub bg_color: Color,
    pub hover_bg_color: Color,
    pub pressed_bg_color: Color,
}

/// Widget > Button (Simple text button).
/// 
/// # Examples
/// 
/// ## Render a button
/// ```
/// loop {
///     if let Some(win) = windows.begin("my_window", &font) {
///         win.push(
///             WindowWidget::Button("press me", &font, None, None)
///         );
///     }
/// }
/// ```
/// 
/// ## Detecting button presses
///  ```
/// loop {
///     if let Some(win) = windows.begin("my_window", &font) {
///         win.push(
///             WindowWidget::Button("press me", &font, None, None)
///         );
/// 
///         if win.get_widget(0).as_button().is_just_pressed {
///             println!("BUTTON WAS JUST PRESSED!");
///         }
///     }
/// }
/// ```
/// 
/// ## Check if button is held
///  ```
/// loop {
///     if let Some(win) = windows.begin("my_window", &font) {
///         win.push(
///             WindowWidget::Button("press me", &font, None, None)
///         );
/// 
///         if win.get_widget(0).as_button().pressed {
///             println!("BUTTON IS BEING HELD DOWN!");
///         }
///     }
/// }
/// ```
#[derive(Clone)]
pub struct Button {
    pub text: &'static str,
    pub rect: Rect,
    pub hovering: bool,
    pub button_rect: Rect,
    pub pressed: bool,
    pub style: ButtonStyle,
    pub uuid: &'static str,
    pub is_just_pressed: bool,
    pub queue_free: bool,
}

impl Button {
    pub fn new(text: &'static str, font: &Font, color: Option<Color>, uuid: Option<&'static str>) -> Self {
        let mut x = Self {
            text,
            uuid: uuid.unwrap_or(""),
            rect: {
                let dim = measure_text(text, None, 16, 1.0);
                Rect::new(0.0, 0.0, dim.width * 1.2 + 2.0, dim.height + 4.0)
            },
            style: ButtonStyle {
                color: color.unwrap_or(WHITE),
                font: font.clone(),
                bg_color: DARKGRAY,
                hover_bg_color: Color::new(0.2, 0.2, 0.2, 1.0),
                pressed_bg_color: Color::new(0.6, 0.6, 0.6, 1.0),
            },
            hovering: false,
            button_rect: Rect::new(0., 0., 50., 50.),
            pressed: false,
            is_just_pressed: false,
            queue_free: false
        };
        
        x.button_rect.w = x.rect.w + 14.;
        x.button_rect.h = x.rect.h + 7.;
        
        x
    }
    
    pub fn update(&mut self, selected: bool) {
        let dim = measure_text(self.text, None, 16, 1f32);
        self.rect.w = dim.width * 1.2 + 2.0;
        self.rect.h = dim.height + 4.0;

        self.is_just_pressed = false;

        self.button_rect.x = self.rect.x - 2.;
        self.button_rect.y = self.rect.y - 12.;
        self.button_rect.w = self.rect.w + 14.;
        self.button_rect.h = self.rect.h + 7.;

        if is_mouse_button_released(MouseButton::Left)
            && self.hovering
            && self.pressed
            && selected
        {
            self.is_just_pressed = true;
        }

        if !is_mouse_button_down(MouseButton::Left) {
            self.pressed = false;
        }

        if self.button_rect.contains(vec2(mouse_position().0, mouse_position().1)) {
            self.hovering = true;
            if is_mouse_button_pressed(MouseButton::Left) && selected {
                self.pressed = true;
            }
        } else {
            self.hovering = false;
        }
    }

    pub fn render(&mut self) {
        draw_rectangle(
            self.button_rect.x,
            self.button_rect.y,
            self.button_rect.w + 4.,
            self.button_rect.h,
            match (self.hovering, self.pressed) {
                (true, false) => self.style.hover_bg_color,
                (_, true) => self.style.pressed_bg_color,
                _ => self.style.bg_color,
            },
        );
        draw_text_ex(
            self.text,
            self.button_rect.x + self.button_rect.w/2.0 - self.rect.w/2.0,
            self.button_rect.y + self.button_rect.h/2.0 + 4.0,
            TextParams {
                font: Some(&self.style.font),
                color: self.style.color,
                font_size: 14,
                ..Default::default()
            },
        );
    }

    pub fn style(&mut self, style: ButtonStyle) -> &mut Self {
        self.style = style;
        self
    }
}