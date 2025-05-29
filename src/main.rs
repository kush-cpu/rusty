use eframe::egui;
mod ui;
mod backend;
mod config;

fn main() -> Result<(), eframe::Error> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Rust Native IDE",
        native_options,
        Box::new(|_cc| Box::new(ui::app::RustIDE::default())),
    )
}
