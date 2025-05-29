use egui::Ui;

#[derive(Default)]
pub struct TerminalPanel {
    logs: String,
}

impl TerminalPanel {
    pub fn ui(&mut self, ui: &mut Ui) {
        ui.group(|ui| {
            ui.label("Terminal Output:");
            ui.text_edit_multiline(&mut self.logs);
        });
    }

    pub fn log(&mut self, output: &str) {
        self.logs.push_str(output);
        self.logs.push('\n');
    }
}
