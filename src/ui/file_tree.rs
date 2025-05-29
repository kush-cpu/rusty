use egui::Ui;
use crate::ui::editor::EditorPanel;
use std::fs;

#[derive(Default)]
pub struct FileTree;

impl FileTree {
    pub fn ui(&self, ui: &mut Ui, editor: &mut EditorPanel) {
        if let Ok(entries) = fs::read_dir(".") {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        if ui.button(name).clicked() {
                            editor.open_file(name);
                        }
                    }
                }
            }
        }
    }
}
