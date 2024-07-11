use macroquad::prelude::*;

/// Widget > Text (Renders single-line text).
#[derive(Clone, Debug)]
pub struct Text {
    pub text: String,
    pub rect: Rect,
    pub color: Color,
    pub font: Option<Font>,
    pub queue_free: bool,
    pub uuid: &'static str
}
impl Text {
    pub fn new(text: &str, font: Option<Font>, color: Option<Color>, uuid: Option<&'static str>) -> Self {
        Self {
            text: text.to_owned(),
            uuid: uuid.unwrap_or(""),
            rect: Rect::new(0.,0.,0.,0.),
            color: color.unwrap_or(WHITE),
            font: font,
            queue_free: false
        }
    }

    pub fn set_uuid(&mut self, uuid: &'static str) -> &mut Self {
        self.uuid = uuid;
        self
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
        let dim = measure_text(&self.text.to_string(), None, 16, 1f32);
        let dim_some = measure_text(&self.text.to_string(), self.font.as_ref(), 16, 1f32);

        let height_diff = dim.height/dim_some.height;
        self.rect.w = dim.width * 1.2 + 2.0;
        self.rect.h = dim.height + 3.0;
        
        draw_text_ex(
            self.text.as_str(),
            f32::floor(self.rect.x),
            f32::floor(self.rect.y),
            TextParams {
                font: self.font.as_ref(),
                font_size: 16,
                color: self.color,
                font_scale: height_diff,
                ..Default::default()
            }
        );
    }
}
