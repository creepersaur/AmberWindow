use super::super::*;

/// Widget > WindowWidget (Manages widget creation).
pub struct WindowWidget {
    pub font: Font
}
impl WindowWidget {
    pub async fn new(font_path: &str) -> Self {
        Self {
            font: load_ttf_font(font_path).await.unwrap(),
        }
    }
    pub fn Text(&self, window: &mut Window, text: &str, color: Option<Color>) -> Text {
        let mut x = Widget::Text(
            Text::new(text, &self.font, color, None)
        );

        window.push(x.clone());
        x.as_text().clone()
    }
    pub fn Button(&self, window: &mut Window, text: &str) -> Button {
        let mut x = Widget::Button(
            Button::new(text, &self.font, None, None)
        );

        window.push(x.clone());
        x.as_button().clone()
    }
    pub fn Slider(&self, window: &mut Window, min: f32, max: f32, rect: Rect) -> Slider {
        let mut x = Widget::Slider(
            Slider::new(&self.font, min, max, rect, None)
        );

        window.push(x.clone());
        x.as_slider().clone()
    }
    pub fn DisplayImage(&self, window: &mut Window, texture: Option<Texture2D>, size: Vec2) -> DisplayImage {
        let mut x = Widget::DisplayImage(
            DisplayImage::new(texture, size, None, None)
        );

        window.push(x.clone());
        x.as_image().clone()
    }
    pub fn WidgetRow(&self, window: &mut Window, widgets: Vec<Widget>) -> WidgetRow {
        let mut x = Widget::WidgetRow(
            WidgetRow::new(widgets, None)
        );

        window.push(x.clone());
        x.as_widget_row().clone()
    }
}