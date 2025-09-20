use crate::app::GameApp;

mod app;
mod components;
mod modals;
mod systems;
mod theme;
mod types;
mod utils;
mod views;
mod windows;

fn main() {
    #[cfg(feature = "tracy")]
    init_tracing();

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
        Box::new(|cc| Ok(Box::new(GameApp::new(cc).unwrap()))),
    )
    .expect("Failed to run egui application.");
}

#[cfg(feature = "tracy")]
fn init_tracing() {
    use tracing_subscriber::EnvFilter;
    use tracing_subscriber::Layer;
    use tracing_subscriber::prelude::*;

    let filter = EnvFilter::new("")
        .add_directive("app=trace".parse().unwrap())
        .add_directive("game=trace".parse().unwrap());
    tracing_subscriber::registry()
        .with(tracing_tracy::TracyLayer::default().with_filter(filter))
        .init();
}
