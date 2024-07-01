use super::super::*;

/// Widget > WindowWidget (Manages widget creation).
pub struct WindowWidget {}
impl WindowWidget {
    pub fn Text(text: &'static str, font: &Font, color: Option<Color>, uuid: Option<&'static str>) -> Widget {
        Widget::Text(
            Text::new(text, font, color, uuid)
        )
    }
    pub fn Button(text: &'static str, font: &Font, color: Option<Color>, uuid: Option<&'static str>) -> Widget {
        Widget::Button(
            Button::new(text, font, color, uuid)
        )
    }
    pub fn Slider(font: Font, min: f32, max: f32, rect: Rect, uuid: Option<&'static str>) -> Widget {
        Widget::Slider(
            Slider::new(font, min, max, rect, uuid)
        )
    }
    pub fn DisplayImage(texture: Option<Texture2D>, size: Vec2, color: Option<Color>, uuid: Option<&'static str>) -> Widget {
        Widget::DisplayImage(
            DisplayImage::new(texture, size, color, uuid)
        )
    }
    pub fn WidgetRow(widgets: Vec<Widget>, uuid: Option<&'static str>) -> Widget {
        Widget::WidgetRow(
            WidgetRow::new(widgets, uuid)
        )
    }
}