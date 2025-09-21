use crate::app::GameApp;
use directories::ProjectDirs;
use std::path::PathBuf;

mod app;
mod components;
mod modals;
mod systems;
mod theme;
mod types;
mod utils;
mod views;
mod windows;

pub const VERSION: &str = "v0.1.0-alpha";

fn main() {
    #[cfg(feature = "tracy")]
    init_tracing();

    let native_options = eframe::NativeOptions {
        renderer: eframe::Renderer::Wgpu,
        viewport: egui::ViewportBuilder::default()
            .with_maximized(true)
            .with_title(app_name())
            .with_app_id("io.github.zitronenjoghurt.dungeon-breeder"),
        persist_window: true,
        persistence_path: Some(save_file_path()),
        ..Default::default()
    };

    eframe::run_native(
        &app_name(),
        native_options,
        Box::new(|cc| Ok(Box::new(GameApp::new(cc).unwrap()))),
    )
    .expect("Failed to run egui application.");
}

pub fn app_name() -> String {
    format!("Dungeon Breeder {}", VERSION)
}

pub fn project_dirs() -> ProjectDirs {
    ProjectDirs::from("io.github", "zitronenjoghurt", "dungeon-breeder").unwrap()
}

pub fn save_dir_path() -> PathBuf {
    project_dirs().data_dir().to_path_buf()
}

pub fn save_file_path() -> PathBuf {
    save_dir_path().join("save.ron")
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
