use macroquad::prelude::*;

/// Widget > DisplayImage (Displays an image on the window).
#[derive(Clone, PartialEq, Debug)]
pub struct DisplayImage {
    pub rect: Rect,
    pub texture: Texture2D,
    pub color: Color,
    pub queue_free: bool,
    pub uuid: &'static str
}
impl DisplayImage {
    pub fn new(texture: Option<Texture2D>, size: Vec2, color: Option<Color>, uuid: Option<&'static str>) -> Self {
        Self {
            texture: texture.unwrap_or(Texture2D::empty()),
            color: color.unwrap_or(WHITE),
            uuid: uuid.unwrap_or(""),
            rect: Rect::new(0.,0.,size.x, size.y),
            queue_free: false
        }
    }

    pub fn set_uuid(&mut self, uuid: &'static str) -> &mut Self {
        self.uuid = uuid;
        self
    }
    
    pub fn set_size(&mut self, size: Vec2) {
        self.rect.w = size.x;
        self.rect.h = size.y;
    }

    pub fn update(&mut self, _selected: bool) {}

    pub fn render(&self) {
        // texture: &Texture2D, x: f32, y: f32, color: Color, params: DrawTextureParams
        draw_texture_ex(
            &self.texture,
            self.rect.x,
            self.rect.y,
            self.color,
            DrawTextureParams {
                dest_size: Some(vec2(self.rect.w, self.rect.h + 5.0)),
                ..Default::default()
            }
        );
    }
}
