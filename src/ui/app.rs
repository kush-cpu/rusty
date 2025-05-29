use super::{editor::EditorPanel, terminal::TerminalPanel, file_tree::FileTree};
use crate::backend::compiler;
use crate::config::CONFIG_FILE;
use crate::backend::project;
use eframe::egui::{CentralPanel, Context, TopBottomPanel};
use eframe::App;

pub struct RustIDE {
    pub editor: EditorPanel,
    pub terminal: TerminalPanel,
    pub file_tree: FileTree,
    pub build_button_clicked: bool,
}

impl Default for RustIDE {
    fn default() -> Self {
        Self {
            editor: EditorPanel::default(),
            terminal: TerminalPanel::default(),
            file_tree: FileTree::default(),
            build_button_clicked: false,
        }
    }
}

impl App for RustIDE {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Build").clicked() {
                    if let Some(config) = project::load_config(CONFIG_FILE) {
                        let parts: Vec<&str> = config.build_cmd.split_whitespace().collect();
                        if let Some((cmd, args)) = parts.split_first() {
                            let output = compiler::compile_shader(cmd, args);
                            self.terminal.log(&output);
                        }
                    }
                }
                self.file_tree.ui(ui, &mut self.editor);
            });
        });

        CentralPanel::default().show(ctx, |ui| {
            self.editor.ui(ui);
        });

        TopBottomPanel::bottom("terminal").show(ctx, |ui| {
            self.terminal.ui(ui);
        });
    }
}
