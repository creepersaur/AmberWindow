use macroquad::prelude::*;

/// Widget > Checkbox (Toggled bool value).
#[derive(Clone)]
pub struct Checkbox {
    pub text: String,
    pub rect: Rect,
    pub color: Color,
    pub bg_color: Color,
    pub font: Font,
    pub queue_free: bool,
    pub uuid: &'static str,
    pub box_rect: Rect,
    pub value: bool,
    pub hovering: bool,
    pub pressed: bool,
}
impl Checkbox {
    pub fn new(
        text: &str,
        font: &Font,
        ticked: Option<bool>,
        color: Option<Color>,
        uuid: Option<&'static str>,
    ) -> Self {
        let mut x = Self {
            text: text.to_owned(),
            uuid: uuid.unwrap_or(""),
            rect: Rect::new(0., 0., 0., 0.),
            color: color.unwrap_or(WHITE),
            font: font.clone(),
            queue_free: false,
            box_rect: Rect::new(0., 0., 15., 15.),
            value: ticked.unwrap_or(false),
            hovering: false,
            pressed: false,
            bg_color: Color::new(1.0, 0.7, 0., 1.0),
        };

        let dim = measure_text(&x.text, None, 16, 1f32);
        x.rect.w = dim.width * 1.2 + 7.0 + x.box_rect.w;
        x.rect.h = x.box_rect.h + 3.0;

        x
    }

    pub fn set_uuid(&mut self, uuid: &'static str) -> &mut Self {
        self.uuid = uuid;
        self
    }

    pub fn set_text(&mut self, text: String) -> &mut Self {
        self.text = text;
        self
    }

    pub fn update(&mut self, selected: bool, mouse_position: Vec2) {
        let dim = measure_text(&self.text, None, 16, 1f32);
        self.rect.w = dim.width * 1.2 + 7.0 + self.box_rect.w;
        self.rect.h = self.box_rect.h + 3.0;

        self.rect.y -= self.box_rect.h;

        if is_mouse_button_released(MouseButton::Left) && self.hovering && self.pressed && selected
        {
            self.value = !self.value;
        }

        if !is_mouse_button_down(MouseButton::Left) {
            self.pressed = false;
        }

        if self.rect.contains(mouse_position) {
            self.hovering = true;
            if is_mouse_button_pressed(MouseButton::Left) {
                self.pressed = true;
            }
        } else {
            self.hovering = false;
        }
    }

    pub fn render(&mut self) {
        let dim = measure_text(&self.text, None, 16, 1f32);
        self.rect.w = dim.width * 1.2 + 7.0 + self.box_rect.w;
        self.rect.h = self.box_rect.h + 3.0;

        let bg_color = match self.value {
            true => self.bg_color,
            false => Color::from_vec(self.bg_color.to_vec() - vec4(0., 0., 0., 0.4))
        };

        draw_rectangle(
            self.rect.x,
            self.rect.y,
            self.box_rect.w,
            self.box_rect.h,
            match (self.hovering, self.pressed) {
                (false, false) => match self.value {
                    true => self.bg_color,
                    false => Color::from_vec(bg_color.to_vec() - vec4(0., 0., 0., 0.5)),
                },
                (_, true) => Color::from_vec(bg_color.to_vec() - vec4(0., 0., 0., 0.5)),
                (true, _) => Color::from_vec(bg_color.to_vec() - vec4(0., 0., 0., 0.4)),
            },
        );

        draw_text_ex(
            self.text.as_str(),
            self.rect.x + self.box_rect.w + 5.0,
            self.rect.y + self.rect.h / 1.5,
            TextParams {
                font: Some(&self.font),
                color: self.color,
                font_size: 14,
                ..Default::default()
            },
        );
    }
}
