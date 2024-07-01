use macroquad::prelude::*;
use super::*;

/// # Manages window update and creation.
/// 
/// Handles all windows and creates new windows.
/// 
/// ## Examples
/// 
/// ### Creating and updating windows
/// 
/// ```
/// let font = load_font_ttf("path").await.unwrap(); // Load font
/// let windows = WindowManager::new(); // Create new window manager
/// 
/// loop {
///     windows.begin("window", &font); // returns `Option<Window>`
///     windows.update_windows();       // Update + Render all windows.
/// }
/// ```
/// 
/// ### Getting window index from id
/// 
/// ```
/// // Get window with id = "window"
/// let idx = windows.get_window_index("window");
/// 
/// windows[idx].queue_free(); // Kill the window
/// ```
pub struct WindowManager {
    pub windows: Vec<Window>,
    freed: Vec<String>,
    font: Font,
}
impl WindowManager {
    /// Create a new WindowManager
    pub async fn new(font_path: &str) -> Self {
        Self {
            windows: vec![],
            freed: vec![],
            font: load_ttf_font(font_path).await.unwrap()
        }
    }

    /// Create a new immediate window with a ***unique*** `id`.
    /// 
    /// Returns an Option<&mut Window> which can have methods called to change itself.
    /// # Example
    /// 
    /// ```
    /// if let Some(win) = windows.begin("my_window") {
    ///     win.name("window_name")
    ///         .push(
    ///             WindowWidget::Text("hello", &font, None, None)
    ///         );
    /// }
    /// ```
    pub fn begin(&mut self, id: &str) -> Option<&mut Window> {
        if let Some(idx) = self.get_window_index(id) {
            if !self.check_freed(id) {
                self.windows[idx].frame_pushed.clear();
        
                return Some(&mut self.windows[idx]);
            }
        } else if !self.check_freed(id) {
            let uuid = RandomRange::gen_range(0, 999999999);
            let mut win = Window::new(
                "Window",
                Rect::new(0., 0., 200., 200.),
                &self.font,
                None,
                id.to_string(),
                Some(uuid.to_string()),
            );
            self.windows.push(win);
            let last = self.windows.len();
            return Some(&mut self.windows[last-1]);
        }
        None
    }

    /// Check if the a window with `id` has been freed.
    pub fn check_freed(&mut self, id: &str) -> bool {
        for i in self.windows.iter() {
            if i.id != id.to_owned() { continue }
            if i.queue_free {
                self.freed.push(i.uuid.clone());
                return true;
            } else if self.freed.contains(&i.uuid) {
                return true;
            }
        }
        false
    }

    /// Updates and renders all windows.
    /// ## MUST BE CALLED AT THE END
    /// *(After calling begin and changing them)*
    pub fn update_windows(&mut self) {
        let mouse_position = vec2(mouse_position().0, mouse_position().1);

        let mut idx: usize = 0;
        let mut selected: Option<usize> = None;

        let mut windows = &mut self.windows;

        for win in windows.iter_mut() {
            if self.freed.contains(&win.uuid) { continue }

            let mut idx = 0;
            win.widgets.retain(|_| {
                if idx > win.frame_pushed.len()-1 {
                    return false;
                }
                idx += 1;
                true
            });
            win.update(selected, &mouse_position);

            if win.selected {
                selected = Some(idx);
            }

            idx += 1;
        }
        if let Some(idx) = selected {
            if windows.len() > 1 {
                windows.insert(0, windows[idx-1].clone());
                windows.remove((idx).into());
            }
        }

        let mut reversed = windows.clone();
        reversed.reverse();
        for win in reversed.iter_mut() {
            if self.freed.contains(&win.uuid) { continue }

            win.render();
        }
    }

    /// Get the index of a window using its `id`.
    pub fn get_window_index(&mut self, id: &str) -> Option<usize> {
        let mut idx = 0;
        for i in self.windows.iter() {
            if i.id == id.to_owned() {
                return Some(idx);
            }
            idx += 1;
        }
        None
    }
}
