use super::super::*;

/// Widget > WindowWidget (Manages widget creation).
pub struct WindowWidget {
    font: Font
}
impl WindowWidget {
    pub async fn new(font_path: &str) -> Self {
        Self {
            font: load_ttf_font(font_path).await.unwrap()
        }
    }
    pub fn Text(&self, text: &'static str, color: Option<Color>, uuid: Option<&'static str>) -> Widget {
        Widget::Text(
            Text::new(text, &self.font, color, uuid)
        )
    }
    pub fn Button(&self, text: &'static str, color: Option<Color>, uuid: Option<&'static str>) -> Widget {
        Widget::Button(
            Button::new(text, &self.font, color, uuid)
        )
    }
    pub fn Slider(&self, min: f32, max: f32, rect: Rect, uuid: Option<&'static str>) -> Widget {
        Widget::Slider(
            Slider::new(&self.font, min, max, rect, uuid)
        )
    }
    pub fn DisplayImage(&self, texture: Option<Texture2D>, size: Vec2, color: Option<Color>, uuid: Option<&'static str>) -> Widget {
        Widget::DisplayImage(
            DisplayImage::new(texture, size, color, uuid)
        )
    }
    pub fn WidgetRow(&self, widgets: Vec<Widget>, uuid: Option<&'static str>) -> Widget {
        Widget::WidgetRow(
            WidgetRow::new(widgets, uuid)
        )
    }
}