use macroquad::prelude::*;

/// Style > Custom Button styling.
#[derive(Clone, Debug)]
pub struct ButtonStyle {
    pub font: Option<Font>,
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
///         widgets.Button(win, "hello");
///     }
/// }
/// ```
///
/// ## Detecting button presses
///  ```
/// loop {
///     if let Some(win) = windows.begin("my_window", &font) {
///         if widgets.Button(win, "hello").1.is_just_pressed {
///             println!("PRESSED");
///         }
///     }
/// }
/// ```
///
/// ## Check if button is held
///  ```
/// loop {
///     if let Some(win) = windows.begin("my_window", &font) {
///         if widgets.Button(win, "hello").1.pressed {
///             println!("BUTTON HELD");
///         }
///     }
/// }
/// ```
#[derive(Clone, Debug)]
pub struct Button {
    pub text: String,
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
    pub fn new(
        text: &str,
        font: Option<Font>,
        color: Option<Color>,
        uuid: Option<&'static str>,
    ) -> Self {
        let mut x = Self {
            text: text.to_string(),
            uuid: uuid.unwrap_or(""),
            rect: {
                let dim = measure_text(text, None, 16, 1.0);
                Rect::new(0.0, 0.0, dim.width * 1.2 + 2.0, dim.height + 4.0)
            },
            style: ButtonStyle {
                font: font.clone(),
                color: color.unwrap_or(WHITE),
                bg_color: Color::new(0.3, 0.3, 0.3, 0.3),
                hover_bg_color: Color::new(0.2, 0.2, 0.2, 0.3),
                pressed_bg_color: Color::new(0.4, 0.4, 0.4, 0.4),
            },
            hovering: false,
            button_rect: Rect::new(0., 0., 50., 50.),
            pressed: false,
            is_just_pressed: false,
            queue_free: false,
        };

        x.button_rect.w = x.rect.w + 14.;
        x.button_rect.h = x.rect.h + 7.;

        x
    }

    pub fn set_uuid(&mut self, uuid: &'static str) -> &mut Self {
        self.uuid = uuid;
        self
    }

    pub fn update(&mut self, selected: bool, mouse_position: Vec2, mouse_released: bool) {
        let dim = measure_text(&self.text.to_string(), None, 16, 1f32);
        self.rect.w = dim.width * 1.2 + 2.0;
        self.rect.h = dim.height + 4.0;

        self.is_just_pressed = false;

        self.button_rect.x = self.rect.x - 2.;
        self.button_rect.y = self.rect.y - 12.;
        self.button_rect.w = self.rect.w + 14.;
        self.button_rect.h = self.rect.h + 7.;

        if mouse_released && self.hovering && self.pressed && selected {
            self.is_just_pressed = true;
        }

        if !is_mouse_button_down(MouseButton::Left) {
            self.pressed = false;
        }

        if self.button_rect.contains(mouse_position) {
            self.hovering = true;
            if is_mouse_button_pressed(MouseButton::Left) && selected {
                self.pressed = true;
            }
        } else {
            self.hovering = false;
        }
    }

    pub fn render(&mut self) {
        let dim = measure_text(&self.text.to_string(), None, 16, 1f32);
        let dim_some = measure_text(&self.text.to_string(), self.style.font.as_ref(), 16, 1f32);

        let height_diff = dim.height / dim_some.height;

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
            self.text.as_str(),
            f32::floor(self.button_rect.x + self.button_rect.w / 2.0 - dim.width / 2.0),
            f32::floor(self.button_rect.y + self.button_rect.h / 2.0 + 4.0),
            TextParams {
                font: self.style.font.as_ref(),
                font_size: 16,
                color: self.style.color,
                font_scale: height_diff,
                ..Default::default()
            },
        );
    }

    pub fn style(&mut self, style: ButtonStyle) -> &mut Self {
        self.style = style;
        self
    }
}
