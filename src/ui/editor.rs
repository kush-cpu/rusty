use egui::Ui;
use std::collections::HashMap;

#[derive(Default)]
pub struct EditorPanel {
    tabs: HashMap<String, String>,
    current: Option<String>,
}

impl EditorPanel {
    pub fn ui(&mut self, ui: &mut Ui) {
        if !self.tabs.is_empty() {
            ui.horizontal(|ui| {
                for (file, _) in self.tabs.iter() {
                    if ui.selectable_label(self.current.as_deref() == Some(file), file).clicked() {
                        self.current = Some(file.clone());
                    }
                }
            });

            if let Some(file) = &self.current {
                if let Some(text) = self.tabs.get_mut(file) {
                    ui.separator();
                    ui.text_edit_multiline(text);
                }
            }
        } else {
            ui.label("No file open");
        }
    }

    pub fn open_file(&mut self, path: &str) {
        if !self.tabs.contains_key(path) {
            if let Ok(contents) = std::fs::read_to_string(path) {
                self.tabs.insert(path.to_string(), contents);
            }
        }
        self.current = Some(path.to_string());
    }
}