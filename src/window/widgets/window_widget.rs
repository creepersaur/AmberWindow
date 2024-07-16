use super::super::*;

/// Widget > WindowWidget (Manages widget creation).
pub struct WindowWidget {
    pub font: Option<Font>,
}
impl WindowWidget {
    pub fn new() -> Self {
        Self { font: None }
    }

    // Set widget font (for all new widgets)
    pub async fn set_font(&mut self, font_path: &str) -> &mut Self {
        self.font = Some(load_ttf_font(font_path).await.unwrap());
        self
    }

    /// Push a `Text` widget to a window. Returns the index and a CLONE of the object.
    pub fn Text(&self, win: &mut Window, text: &str, color: Option<Color>) -> (usize, Text) {
        let mut x = Widget::Text(Text::new(text, self.font.clone(), color, None));

        win.push(&mut x.clone());
        (win.widgets.len() - 1, x.as_text().clone())
    }

    /// Push a `Button` widget to a window. Returns the index and a CLONE of the object.
    pub fn Button(&self, win: &mut Window, text: &str) -> (usize, Button) {
        let mut x = Widget::Button(Button::new(text, self.font.clone(), None, None));

        win.push(&mut x);
        (win.widgets.len() - 1, x.as_button().clone())
    }

    /// Push a `Slider` widget to a window. Returns the index and a CLONE of the object.
    pub fn Slider(
        &self,
        win: &mut Window,
        min: f32,
        max: f32,
        default: Option<f32>,
        size: Vec2,
    ) -> (usize, Slider) {
        let mut x = Widget::Slider(Slider::new(
            self.font.clone(),
            min,
            max,
            default,
            size,
            None,
        ));

        let idx = win.push(&mut x.clone());
        (win.widgets.len() - 1, win.get(idx).as_slider().clone())
    }

    /// Push a `DisplayImage` widget to a window. Returns the index and a CLONE of the object.
    pub fn DisplayImage(
        &self,
        win: &mut Window,
        texture: Option<Texture2D>,
        size: Vec2,
    ) -> (usize, DisplayImage) {
        let mut x = Widget::DisplayImage(DisplayImage::new(texture, size, None, None));

        win.push(&mut x.clone());
        (win.widgets.len() - 1, x.as_image().clone())
    }

    /// Push a `WidgetRow` widget to a window. Returns the index and a CLONE of the object.
    pub fn WidgetRow(&self, win: &mut Window) -> (usize, Option<WidgetRow>) {
        let mut x = Widget::WidgetRow(WidgetRow::new(self.font.clone(), None, win.rect.w));

        win.push(&mut x.clone());
        (win.widgets.len() - 1, Some(x.as_widget_row().clone()))
    }

    /// Push a `Checkbox` widget to a window. Returns the index and a CLONE of the object.
    pub fn Checkbox(&self, win: &mut Window, text: &str, ticked: bool) -> (usize, Checkbox) {
        let mut x = Widget::Checkbox(Checkbox::new(
            text,
            self.font.clone(),
            Some(ticked),
            None,
            None,
        ));

        win.push(&mut x);
        (win.widgets.len() - 1, x.as_checkbox().clone())
    }
}
