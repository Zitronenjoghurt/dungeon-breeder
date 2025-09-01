use crate::app::DBApp;

mod app;
mod components;
mod modals;
mod state;
mod views;
mod windows;

fn main() {
    let native_options = eframe::NativeOptions {
        renderer: eframe::Renderer::Wgpu,
        viewport: egui::ViewportBuilder::default()
            .with_maximized(true)
            .with_title("Dungeon Breeder"),
        persist_window: true,
        ..Default::default()
    };

    eframe::run_native(
        "Dungeon Breeder",
        native_options,
        Box::new(|cc| Ok(Box::new(DBApp::new(cc).unwrap()))),
    )
    .expect("Failed to run egui application.");
}
