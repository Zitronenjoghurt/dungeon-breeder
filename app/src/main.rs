use crate::app::GameApp;
use directories::ProjectDirs;
use dungeon_breeder_core::VERSION_NAME;
use std::path::PathBuf;

mod app;
mod components;
mod data;
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
            .with_title(app_name())
            .with_app_id("io.github.zitronenjoghurt.dungeon-breeder")
            .with_icon(load_icon()),
        persist_window: true,
        persistence_path: Some(eframe_save_file_path()),
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
    format!("Dungeon Breeder {}", VERSION_NAME)
}

pub fn project_dirs() -> ProjectDirs {
    ProjectDirs::from("io.github", "zitronenjoghurt", "dungeon-breeder").unwrap()
}

pub fn data_dir_path() -> PathBuf {
    project_dirs().data_dir().to_path_buf()
}

pub fn save_dir_path() -> PathBuf {
    data_dir_path().join("save")
}

pub fn eframe_save_file_path() -> PathBuf {
    save_dir_path().join("eframe.dat")
}

pub fn app_save_file_path() -> PathBuf {
    save_dir_path().join("app.dat")
}

pub fn meta_save_file_path() -> PathBuf {
    save_dir_path().join("meta.dat")
}

fn load_icon() -> egui::IconData {
    let icon_bytes = include_bytes!("../../assets/icons/icon_512x512.png");
    let image = image::load_from_memory(icon_bytes)
        .expect("Failed to load icon")
        .into_rgba8();
    let (width, height) = image.dimensions();

    egui::IconData {
        rgba: image.into_raw(),
        width,
        height,
    }
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
