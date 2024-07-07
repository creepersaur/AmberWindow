#![allow(unused)]

use std::vec;

use super::widgets::*;
use macroquad::{prelude::*, ui};

/// Style > Custom window styling.
#[derive(Clone)]
pub struct WindowStyle {
    pub font: Option<Font>,
    pub bg_color: Color,
    pub tb_color: Color,
    pub border_color: Color,
    pub selected_border_color: Color,
    pub title_color: Color,
    pub scale_color: Color,
    pub minimize_color: Color,
}

/// # Properties > Properties for windows.
#[derive(Clone)]
pub struct WindowProperties {
    pub wall_collision: bool,
    pub draggable: bool,
    pub scalable: bool,
    pub minimizable: bool,
    pub no_title_bar: bool,
}

/// # The base window class.
#[derive(Clone)]
pub struct Window {
    pub name: String,
    pub id: String,
    pub uuid: String,
    pub rect: Rect,
    tb_rect: Rect,
    pub style: WindowStyle,
    tb_hovered: bool,
    tb_pressed: bool,
    pub dragging: bool,
    drag_mpos: (Vec2, Vec2),
    pub selected: bool,
    pub properties: WindowProperties,
    pub widgets: Vec<Widget>,
    pub queue_free: bool,
    pub scaling: Option<(Vec2, Vec2)>,
    scale_collider: Rect,
    pub scale_triangle_size: f32,
    scale_hover: bool,
    minimize_rect: Rect,
    pub minimized: Option<f32>,
    minimize_hover: bool,
    minimize_pressed: bool,
    button_style: Option<ButtonStyle>,
    close_rect: Rect,
    close_pressed: bool,
    close_hovered: bool,
    pub frame_pushed: Vec<Widget>,
}

// MAIN IMPL
impl Window {
    pub fn new(
        name: &str,
        rect: Rect,
        font: Option<Font>,
        widgets: Option<Vec<Widget>>,
        id: String,
        uuid: Option<String>,
    ) -> Self {
        Self {
            name: name.to_string(),
            id: id,
            uuid: uuid.unwrap_or("".to_owned()),
            rect,
            tb_rect: Rect::new(rect.x, rect.y, rect.w, 20.0),
            style: WindowStyle {
                font: font,
                bg_color: Color::new(0.1, 0.1, 0.1, 1.0),
                tb_color: GOLD, //DARKBLUE,
                border_color: BLANK,
                selected_border_color: ORANGE,
                title_color: BLACK,
                scale_color: Color::new(1.0, 0.7, 0., 0.25),
                minimize_color: BLACK,
            },
            properties: WindowProperties {
                wall_collision: true,
                draggable: true,
                scalable: true,
                minimizable: true,
                no_title_bar: false,
            },
            selected: false,
            tb_hovered: false,
            tb_pressed: false,
            dragging: false,
            drag_mpos: (Vec2::ZERO, Vec2::ZERO),
            queue_free: false,
            scaling: None,
            widgets: widgets.unwrap_or(Vec::new()),
            scale_collider: Rect::new(0., 0., 0., 0.),
            scale_triangle_size: 15f32,
            scale_hover: false,
            minimize_rect: Rect::new(0., 0., 0., 0.),
            close_rect: Rect::new(0., 0., 0., 0.),
            minimized: None,
            minimize_hover: false,
            minimize_pressed: false,
            button_style: None,
            close_pressed: false,
            close_hovered: false,
            frame_pushed: vec![],
        }
    }

    pub fn queue_free(&mut self) {
        self.queue_free = true;
        drop(self)
    }

    pub fn update(&mut self, selected: Option<usize>, mouse_position: &Vec2) {
        self.update_close_button(mouse_position);

        self.update_mouse_released();

        self.update_window_scaling(mouse_position);

        if !self.properties.no_title_bar {
            self.update_dragging(mouse_position);
            self.update_minimise(mouse_position);
            self.update_top_bar(selected, mouse_position);
        }

        self.update_wall_collision();

        self.update_selection(selected, mouse_position);

        self.update_widgets(mouse_position);

        self.update_quit_window();

        self.update_min_size_limit();
    }

    pub fn render(&mut self) {
        // TOP BAR
        let title_padding = self.render_top_bar();

        // MAIN
        if !self.minimized.is_some() {
            draw_rectangle(
                self.rect.x,
                self.rect.y + title_padding,
                self.rect.w,
                self.rect.h - title_padding,
                self.style.bg_color,
            );

            self.render_widgets(title_padding);

            self.render_scale_triangle();
        }

        self.render_outline();
    }
}

// UPDATE
impl Window {
    fn update_mouse_released(&mut self) {
        if !is_mouse_button_down(MouseButton::Left) {
            self.tb_pressed = false;
            self.dragging = false;
            self.scaling = None;
            self.minimize_pressed = false;
            self.close_pressed = false;
        }
    }

    fn update_window_scaling(&mut self, mouse_position: &Vec2) {
        let bottom_right = vec2(self.rect.x + self.rect.w, self.rect.y + self.rect.h);
        self.scale_collider.x = bottom_right.x - self.scale_triangle_size;
        self.scale_collider.y = bottom_right.y - self.scale_triangle_size;
        self.scale_collider.w = self.scale_triangle_size;
        self.scale_collider.h = self.scale_triangle_size;

        if self.properties.scalable && self.scale_collider.contains(*mouse_position) {
            self.scale_hover = true;
            if is_mouse_button_pressed(MouseButton::Left) {
                self.scaling = Some((vec2(self.rect.w, self.rect.h), *mouse_position));
            }
        } else {
            self.scale_hover = false;
        }

        if let Some((size, position)) = self.scaling {
            self.rect.w = size.x + (*mouse_position).x - position.x;
            self.rect.h = size.y + (*mouse_position).y - position.y;
            if self.rect.x + self.rect.w > screen_width() {
                self.rect.w = screen_width() - self.rect.x;
            }
            if self.rect.y + self.rect.h > screen_height() {
                self.rect.h = screen_height() - self.rect.y;
            }
        }
    }

    fn update_dragging(&mut self, mouse_position: &Vec2) {
        if self.properties.draggable && self.dragging {
            let delta = *mouse_position + (self.drag_mpos.1 - self.drag_mpos.0);

            self.rect.x = delta.x;
            self.rect.y = delta.y;
        }
    }

    fn update_wall_collision(&mut self) {
        if self.properties.wall_collision {
            if self.rect.x < 0.0 {
                self.rect.x = 0.0
            }
            if self.rect.y < 0.0 {
                self.rect.y = 0.0
            }

            if self.rect.x > screen_width() - self.rect.w {
                self.rect.x = screen_width() - self.rect.w
            }
            if self.rect.y > screen_height() - self.rect.h {
                self.rect.y = screen_height() - self.rect.h
            }
        }
    }

    fn update_selection(&mut self, selected: Option<usize>, mouse_position: &Vec2) {
        if is_mouse_button_pressed(MouseButton::Left) {
            if self.rect.contains(*mouse_position) {
                if selected == None {
                    self.selected = true
                }
            } else {
                self.selected = false;
            }
        }
    }

    fn update_top_bar(&mut self, selected: Option<usize>, mouse_position: &Vec2) {
        match (
            (self.tb_hovered && !self.minimize_pressed && !self.close_pressed),
            self.tb_pressed,
        ) {
            (true, true) => {
                if selected == None {
                    self.selected = true;
                    self.dragging = true;
                }
                self.drag_mpos = (*mouse_position, vec2(self.rect.x, self.rect.y));
            }
            _ => {}
        };

        if self.tb_rect.contains(*mouse_position) {
            self.tb_hovered = true;
            if is_mouse_button_pressed(MouseButton::Left) {
                self.tb_pressed = true
            }
        } else {
            self.tb_hovered = false;
        }

        self.tb_rect.x = self.rect.x;
        self.tb_rect.y = self.rect.y;
        self.tb_rect.w = self.rect.w;
    }

    fn update_widgets(&mut self, mouse_position: &Vec2) {
        let title_padding = match self.properties.no_title_bar {
            true => 0.0,
            _ => 20.0,
        };

        let mut max_width = 0f32;
        let mut last_y = 17.0 + title_padding;
        let padding = 5.0;
        let padding_left = 7.0;

        for i in self.widgets.iter_mut() {
            if let Widget::Text(i) = i {
                i.rect.x = self.rect.x + padding_left;
                i.rect.y = self.rect.y + last_y;
                i.update(self.selected);

                last_y += i.rect.h + padding;
                if i.rect.w > max_width {
                    max_width = i.rect.w;
                }
            } else if let Widget::Button(i) = i {
                i.rect.x = self.rect.x + padding_left;
                i.rect.y = self.rect.y + last_y;
                i.update(self.selected, *mouse_position);

                last_y += i.rect.h + padding + 4.0;
                if i.button_rect.w + 4.0 > max_width {
                    max_width = i.button_rect.w + 4.0;
                }
            } else if let Widget::Slider(i) = i {
                i.rect.x = self.rect.x + padding_left;
                i.rect.y = self.rect.y + last_y - 10.0;
                i.update(self.selected, mouse_position);

                last_y += i.rect.h + padding + 1.0;
                if i.rect.w + 4.0 > max_width {
                    max_width = i.rect.w + 4.0;
                }
            } else if let Widget::DisplayImage(i) = i {
                i.rect.x = self.rect.x + padding_left;
                i.rect.y = self.rect.y + last_y - 10.0;
                i.update(self.selected);

                last_y += i.rect.h + padding + 1.0;
                if i.rect.w + 4.0 > max_width {
                    max_width = i.rect.w + 4.0;
                }
            } else if let Widget::WidgetRow(i) = i {
                i.rect.x = self.rect.x + padding_left;
                i.rect.y = self.rect.y + last_y;
                i.update(self.selected, *mouse_position);
                i.frame_pushed.clear();

                last_y += i.rect.h + padding;
                if i.rect.w > max_width {
                    max_width = i.rect.w;
                }
            } else if let Widget::Checkbox(i) = i {
                i.rect.x = self.rect.x + padding_left;
                i.rect.y = self.rect.y + last_y;
                i.update(self.selected, *mouse_position);

                last_y += i.rect.h + padding + 4.0;
                if i.rect.w + 4.0 > max_width {
                    max_width = i.rect.w + 4.0;
                }
            }
        }

        if last_y - 10.0 > self.rect.h {
            self.rect.h = last_y - 10.0;
        }
        if max_width + 10.0 > self.rect.w {
            self.rect.w = max_width + 10.0
        }
    }

    fn update_minimise(&mut self, mouse_position: &Vec2) {
        self.minimize_rect.x = self.tb_rect.x + 6.;
        self.minimize_rect.y = self.tb_rect.y + 6.;
        self.minimize_rect.w = 14.0;
        self.minimize_rect.h = self.tb_rect.h - 10.;

        if !self.properties.no_title_bar && self.minimize_rect.contains(*mouse_position) {
            self.minimize_hover = true;
            if is_mouse_button_pressed(MouseButton::Left) {
                self.minimize_pressed = true;
                if let Some(height) = self.minimized {
                    self.rect.h = height;
                    self.minimized = None;
                } else {
                    self.minimized = Some(self.rect.h)
                }
            }
        } else {
            self.minimize_hover = false;
        }

        if self.minimized.is_some() {
            self.rect.h = 20.0;
        }
    }

    fn update_quit_window(&mut self) {
        if self.selected && is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::Q) {
            self.queue_free = true;
        }
    }

    fn update_close_button(&mut self, mouse_position: &Vec2) {
        self.close_rect.x = self.tb_rect.x + self.tb_rect.w - 18.;
        self.close_rect.y = self.tb_rect.y + 3.;
        self.close_rect.w = 14.0;
        self.close_rect.h = self.tb_rect.h - 6.;

        if !self.properties.no_title_bar && self.close_rect.contains(*mouse_position) {
            self.close_hovered = true;
            if is_mouse_button_pressed(MouseButton::Left) {
                self.close_pressed = true;
            }
            if is_mouse_button_released(MouseButton::Left) && self.close_pressed {
                self.queue_free()
            }
        } else {
            self.close_hovered = false;
        }

        if self.minimized.is_some() {
            self.rect.h = 20.0;
        }
    }

    fn update_min_size_limit(&mut self) {
        let dim = measure_text(&self.name, None, 16, 1f32);
        let title_width = dim.width * 1.2 + 4.0;
        if !self.properties.no_title_bar {
            if self.rect.w < title_width + 40.0 {
                self.rect.w = title_width + 40.0;
            }
            if self.rect.h < 40.0 {
                self.rect.h = 40.0;
            }
        } else {
            if self.rect.w < title_width + 20.0 {
                self.rect.w = title_width + 20.0;
            }
            if self.rect.h < 20.0 {
                self.rect.h = 20.0;
            }
        }
    }
}

// RENDER
impl Window {
    fn render_top_bar(&mut self) -> f32 {
        self.tb_rect.x = self.rect.x;
        self.tb_rect.y = self.rect.y;
        self.tb_rect.w = self.rect.w;

        let title_padding = match self.properties.no_title_bar {
            true => 0.0,
            _ => 20.0,
        };

        self.tb_rect.w = self.rect.w;

        self.close_rect.x = self.tb_rect.x + self.tb_rect.w - 18.;
        self.close_rect.y = self.tb_rect.y + 3.;
        self.close_rect.w = 14.0;
        self.close_rect.h = self.tb_rect.h - 6.;

        if !self.properties.no_title_bar {
            self.render_topbar_and_title();
            self.render_minimise_button();
            self.render_close_button();
        }

        title_padding
    }

    fn render_topbar_and_title(&mut self) {
        let dim = measure_text(&self.name.to_string(), None, 16, 1f32);
        let dim_some = measure_text(&self.name.to_string(), self.style.font.as_ref(), 16, 1f32);

        let height_diff = dim.height/dim_some.height;

        // TOP BAR
        draw_rectangle(
            self.tb_rect.x,
            self.tb_rect.y,
            self.tb_rect.w,
            self.tb_rect.h,
            self.style.tb_color,
        );

        // WINDOW TITLE
        draw_text_ex(
            &self.name,
            self.tb_rect.x
                + 5f32
                + match self.properties.minimizable {
                    true => 20.0,
                    _ => 0.0,
                },
            self.tb_rect.y + 15f32,
            TextParams {
                font: self.style.font.as_ref(),
                font_size: 16,
                color: self.style.title_color,
                font_scale: height_diff,
                ..Default::default()
            },
        );
    }

    fn render_minimise_button(&mut self) {
        self.minimize_rect.x = self.tb_rect.x + 6.;
        self.minimize_rect.y = self.tb_rect.y + 6.;
        self.minimize_rect.w = 14.0;
        self.minimize_rect.h = self.tb_rect.h - 10.;

        // MINIMIZE TRIANGLE
        match (self.minimized.is_none(), self.properties.minimizable) {
            (true, true) => {
                draw_triangle(
                    vec2(self.minimize_rect.x, self.minimize_rect.y),
                    vec2(
                        self.minimize_rect.x + self.minimize_rect.w,
                        self.minimize_rect.y,
                    ),
                    vec2(
                        self.minimize_rect.x + self.minimize_rect.w / 2.0,
                        self.minimize_rect.y + self.minimize_rect.h,
                    ),
                    match self.minimize_hover {
                        true => Color::from_vec(
                            self.style.minimize_color.to_vec() - vec4(0.0, 0.0, 0.0, 0.2),
                        ),
                        _ => self.style.minimize_color,
                    },
                );
            }
            (_, true) => {
                draw_triangle(
                    vec2(
                        self.minimize_rect.x,
                        self.minimize_rect.y + self.minimize_rect.h - 2.0,
                    ),
                    vec2(
                        self.minimize_rect.x + self.minimize_rect.w,
                        self.minimize_rect.y + self.minimize_rect.h - 2.0,
                    ),
                    vec2(
                        self.minimize_rect.x + self.minimize_rect.w / 2.0,
                        self.minimize_rect.y - 2.0,
                    ),
                    match self.minimize_hover {
                        true => Color::from_vec(
                            self.style.minimize_color.to_vec() - vec4(0.0, 0.0, 0.0, 0.2),
                        ),
                        _ => self.style.minimize_color,
                    },
                );
            }
            (_, _) => {}
        }
    }

    fn render_close_button(&mut self) {
        // CLOSE BUTTON
        if self.close_hovered {
            self.close_rect.x -= 0.5;
            self.close_rect.y -= 0.5;
            self.close_rect.w += 1.;
            self.close_rect.h += 1.;
        }

        // CLOSE RECTANGLE
        draw_rectangle(
            self.close_rect.x,
            self.close_rect.y,
            self.close_rect.w,
            self.close_rect.h,
            match self.close_hovered {
                true => Color::new(0., 0., 0., 0.33),
                _ => Color::new(0., 0., 0., 0.2),
            },
        );

        // CLOSE 'X'
        let x_thickness = 2f32;
        draw_line(
            self.close_rect.x + 4.,
            self.close_rect.y + 4.,
            self.close_rect.x + self.close_rect.w - 4.,
            self.close_rect.y + self.close_rect.h - 4.,
            x_thickness,
            BLACK,
        );

        draw_line(
            self.close_rect.x + self.close_rect.w - 4.,
            self.close_rect.y + 4.,
            self.close_rect.x + 4.,
            self.close_rect.y + self.close_rect.h - 4.,
            x_thickness,
            BLACK,
        )
    }

    fn render_scale_triangle(&self) {
        // SCALE TRIANGLE
        let bottom_right = vec2(self.rect.x + self.rect.w, self.rect.y + self.rect.h);

        if self.properties.scalable {
            draw_triangle(
                bottom_right,
                bottom_right - Vec2::X * (self.scale_triangle_size + 2.),
                bottom_right - Vec2::Y * (self.scale_triangle_size + 2.),
                match (self.scale_hover, self.scaling.is_some()) {
                    (true, true) => {
                        Color::from_vec(self.style.scale_color.to_vec() + vec4(0.1, 0.1, 0.1, 0.3))
                    },
                    (true, _) => {
                        Color::from_vec(self.style.scale_color.to_vec() + vec4(0.1, 0.1, 0.1, 0.4))
                    }
                    _ => self.style.scale_color,
                },
            );
        }
    }

    fn render_widgets(&mut self, title_padding: f32) {
        if let Some(style) = &self.button_style {
            self.button_style(style.clone());
        }

        // WIDGETS
        let mut last_y = 17.0 + title_padding;
        let padding = 5.0;
        let padding_left = 7.0;

        for i in self.widgets.iter_mut() {
            if let Widget::Text(i) = i {
                i.rect.x = self.rect.x + padding_left;
                i.rect.y = self.rect.y + last_y;
                i.render();

                last_y += i.rect.h + padding;
            } else if let Widget::Button(i) = i {
                i.rect.x = self.rect.x + padding_left;
                i.rect.y = self.rect.y + last_y;
                i.render();

                last_y += i.rect.h + padding + 4.0;
            } else if let Widget::Slider(i) = i {
                i.rect.x = self.rect.x + padding_left;
                i.rect.y = self.rect.y + last_y - 10.0;
                i.render();

                last_y += i.rect.h + padding;
            } else if let Widget::DisplayImage(i) = i {
                i.rect.x = self.rect.x + padding_left;
                i.rect.y = self.rect.y + last_y - 10.0;
                i.render();

                last_y += i.rect.h + padding;
            } else if let Widget::WidgetRow(i) = i {
                i.rect.x = self.rect.x + padding_left;
                i.rect.y = self.rect.y + last_y;
                i.render();

                last_y += i.rect.h + padding;
            } else if let Widget::Checkbox(i) = i {
                i.rect.x = self.rect.x + padding_left;
                i.rect.y = self.rect.y + last_y - 10.0;
                i.render();

                last_y += i.rect.h + padding;
            }
        }
    }

    fn render_outline(&self) {
        draw_rectangle_lines(
            self.rect.x,
            self.rect.y,
            self.rect.w,
            match self.minimized.is_some() {
                true => 20.0,
                _ => self.rect.h,
            },
            2f32,
            match self.selected {
                true => self.style.selected_border_color,
                _ => self.style.border_color,
            },
        );
    }
}

// WINDOW METHODS
impl Window {
    /// Set the name/title of the window.
    pub fn name(&mut self, name: &str) -> &mut Self {
        self.name = name.to_owned();
        self
    }

    /// Set the position of the window.
    pub fn position(&mut self, position: Vec2) -> &mut Self {
        self.rect.x = position.x;
        self.rect.y = position.y;
        self
    }

    /// Set the size of the window.
    pub fn size(&mut self, size: Vec2) -> &mut Self {
        self.rect.w = size.x;
        self.rect.h = size.y;
        self
    }

    /// Set the style of the window.
    pub fn style(&mut self, style: WindowStyle) -> &mut Self {
        self.style = style;
        self
    }

    /// Set the properties of the window.
    pub fn properties(&mut self, properties: WindowProperties) -> &mut Self {
        self.properties = properties;
        self
    }

    /// Push multiple widgets to the window.
    pub fn push_widgets(&mut self, widgets: &mut Vec<Widget>) -> &mut Self {
        if widgets.len() < 1 {
            return self;
        }

        for i in widgets.iter_mut() {
            self.push(i);
        }

        self
    }

    // Push a single widget to the window.
    pub fn push(&mut self, widget: &mut Widget) -> &mut Self {
        let mut idx = self.frame_pushed.len();

        if self.widgets.len() < 1 || self.widgets.len() - 1 < idx {
            self.widgets.push(widget.clone())
        } else if widget.equate(&mut self.get_widget(idx)) {
            if let Widget::Text(ref mut widget) = widget {
                self.get_widget(idx).as_text().text = widget.text.clone();
            } else if let Widget::Button(ref mut widget) = widget {
                widget.pressed = self.get_widget(idx).as_button().pressed;
                widget.hovering = self.get_widget(idx).as_button().hovering;
                widget.is_just_pressed = self.get_widget(idx).as_button().is_just_pressed;
            } else if let Widget::Slider(ref mut widget) = widget {
                widget.pressed = self.get_widget(idx).as_slider().pressed;
                widget.hovering = self.get_widget(idx).as_slider().hovering;
                widget.value = self.get_widget(idx).as_slider().value;
            } else if let Widget::Checkbox(ref mut widget) = widget {
                widget.pressed = self.get_widget(idx).as_checkbox().pressed;
                widget.hovering = self.get_widget(idx).as_checkbox().hovering;
                widget.value = self.get_widget(idx).as_checkbox().value;
                widget.is_just_pressed = self.get_widget(idx).as_checkbox().is_just_pressed;
            }

            self.widgets[idx] = widget.clone();
        } else {
            self.widgets[idx] = widget.clone();
        }
        self.frame_pushed.push(widget.clone());

        self
    }
    
    /// Set the window's buttons' styles.
    pub fn button_style(&mut self, style: ButtonStyle) -> &mut Self {
        for i in self.widgets.iter_mut() {
            if let Widget::Button(i) = i {
                i.style = style.clone();
            } else if let Widget::WidgetRow(i) = i {
                i.button_style(&style);
            }
        }
        self
    }

    /// Get a widget by its index (usize/int).
    pub fn get_widget(&mut self, idx: usize) -> &mut Widget {
        &mut self.widgets[idx]
    }

    /// Get a widget by its index (usize/int). (`get_widget()` shorthand).
    pub fn get(&mut self, idx: usize) -> &mut Widget {
        &mut self.widgets[idx]
    }

    /// Get a widget using its uuid (str).
    pub fn get_widget_from_uuid(&mut self, uuid: &str) -> Option<&mut Widget> {
        for i in self.widgets.iter_mut() {
            if i.as_text().uuid == uuid {
                return Some(i);
            }
            if i.as_button().uuid == uuid {
                return Some(i);
            }
            if i.as_slider().uuid == uuid {
                return Some(i);
            }
            if i.as_image().uuid == uuid {
                return Some(i);
            }
            if i.as_widget_row().uuid == uuid {
                return Some(i);
            }
        }
        return None;
    }
}
