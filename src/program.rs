use std::collections::HashMap;

use crate::{UUID, Window, editor::Editor};

pub struct Program {
    editor: Editor,
    windows: HashMap<UUID, Box<dyn Window>>
}

impl Program {
    pub fn new() -> Self {
        Self {
            editor: Editor::new(),
            windows: HashMap::new()
        }
    }

    pub fn add_window(&mut self, window: Box<dyn Window>) {
        self.windows.insert(window.uuid(), window);
    }

    pub fn get_window(&self, window_uuid: UUID) -> Option<&dyn Window> {
        match self.windows.get(&window_uuid) {
            Some(w) => Some(w.as_ref()),
            None => None
        }
    }

    pub fn get_windows(&self) -> Vec<&dyn Window> {
        self.windows.values().map(|w| w.as_ref()).collect()
    }

    pub fn get_windows_mut(&mut self) -> Vec<&mut dyn Window> {
        self.windows.values_mut().map(|w| -> &mut dyn Window { w.as_mut() } ).collect()
    }

    pub fn remove_window(&mut self, window_uuid: UUID) {
        self.windows.remove(&window_uuid);
    }

    pub fn update(&mut self) {
        self.editor.update();

        for (_, window) in &mut self.windows {
            window.update();
        }
    }

    pub fn render(&self) {
        self.editor.render();

        for (_, window) in &self.windows {
            window.render();
        }
    }
}