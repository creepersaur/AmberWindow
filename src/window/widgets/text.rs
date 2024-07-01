use macroquad::prelude::*;

/// Widget > Text (Renders single-line text).
#[derive(Clone)]
pub struct Text {
    pub text: String,
    pub rect: Rect,
    pub color: Color,
    pub font: Font,
    pub queue_free: bool,
    pub uuid: &'static str
}
impl Text {
    pub fn new(text: &'static str, font: &Font, color: Option<Color>, uuid: Option<&'static str>) -> Self {
        Self {
            text: text.to_owned(),
            uuid: uuid.unwrap_or(""),
            rect: Rect::new(0.,0.,0.,0.),
            color: color.unwrap_or(WHITE),
            font: font.clone(),
            queue_free: false
        }
    }
    
    pub fn set_text(&mut self, text: String) -> &mut Self {
        self.text = text;
        self
    }

    pub fn update(&mut self, _selected: bool) {
        let dim = measure_text(&self.text, None, 16, 1f32);
        self.rect.w = dim.width * 1.2 + 2.0;
        self.rect.h = dim.height + 3.0;
    }

    pub fn render(&mut self) {
        let dim = measure_text(&self.text, None, 16, 1f32);
        self.rect.w = dim.width * 1.2 + 2.0;
        self.rect.h = dim.height + 3.0;
        
        draw_text_ex(
            self.text.as_str(),
            self.rect.x,
            self.rect.y,
            TextParams {
                font: Some(&self.font),
                color: self.color,
                font_size: 14,
                ..Default::default()
            }
        );
    }
}
