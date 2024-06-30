use super::*;

pub struct WindowManager {
    pub windows: Vec<Window>,
    freed: Vec<String>,
}

impl WindowManager {
    pub fn new() -> Self {
        Self {
            windows: vec![],
            freed: vec![],
        }
    }

    pub fn begin(&mut self, id: &str, font: &Font) -> Option<&mut Window> {
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
                &font,
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

    pub fn update_windows(&mut self) {
        let mouse_position = Vec2::from_tuple(mouse_position());

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
