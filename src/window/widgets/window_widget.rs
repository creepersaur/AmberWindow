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

    /// Push a `Text` widget to a window. Returns the index and a CLONE of the object.
    pub fn Text(&self, win: &mut Window, text: &str, color: Option<Color>) -> (usize, Text) {
        let mut x = Widget::Text(
            Text::new(text, &self.font, color, None)
        );

        win.push(&mut x.clone());
        (win.widgets.len()-1, x.as_text().clone())
    }

    /// Push a `Button` widget to a window. Returns the index and a CLONE of the object.
    pub fn Button(&self, win: &mut Window, text: &str) -> (usize, Button) {
        let mut x = Widget::Button(
            Button::new(text, &self.font, None, None)
        );

        win.push(&mut x);
        (win.widgets.len()-1, x.as_button().clone())
    }

    /// Push a `Slider` widget to a window. Returns the index and a CLONE of the object.
    pub fn Slider(&self, win: &mut Window, min: f32, max: f32, rect: Rect) -> (usize, Slider) {
        let mut x = Widget::Slider(
            Slider::new(&self.font, min, max, rect, None)
        );

        win.push(&mut x.clone());
        (win.widgets.len()-1, x.as_slider().clone())
    }

    /// Push a `DisplayImage` widget to a window. Returns the index and a CLONE of the object.
    pub fn DisplayImage(&self, win: &mut Window, texture: Option<Texture2D>, size: Vec2) -> (usize, DisplayImage) {
        let mut x = Widget::DisplayImage(
            DisplayImage::new(texture, size, None, None)
        );

        win.push(&mut x.clone());
        (win.widgets.len()-1, x.as_image().clone())
    }

    /// Push a `WidgetRow` widget to a window. Returns the index and a CLONE of the object.
    pub fn WidgetRow(&self, win: &mut Window, widgets: Vec<Widget>) -> (usize, WidgetRow) {
        let mut x = Widget::WidgetRow(
            WidgetRow::new(widgets, None)
        );

        win.push(&mut x.clone());
        (win.widgets.len()-1, x.as_widget_row().clone())
    }
    
    /// Push a `Checkbox` widget to a window. Returns the index and a CLONE of the object.
    pub fn Checkbox(&self, win: &mut Window, text: &str, ticked: bool) -> (usize, Checkbox) {
        let mut x = Widget::Checkbox(
            Checkbox::new(text, &self.font, Some(ticked), None, None)
        );

        win.push(&mut x);
        (win.widgets.len()-1, x.as_checkbox().clone())
    }
}