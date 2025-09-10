use egui::ViewportInfo;
use serde::{Deserialize, Serialize};
use std::env::var;

#[derive(Debug, Serialize, Deserialize)]
pub struct RuntimeInfo {
    pub viewport: ViewportInfo,
    pub rust_version: Option<String>,
    pub target_triple: Option<String>,
    pub target_arch: Option<String>,
    pub target_os: Option<String>,
    pub target_family: Option<String>,
    pub target_pointer_width: Option<String>,
    pub target_endian: Option<String>,
    pub debug_assertions: bool,
}

impl RuntimeInfo {
    pub fn collect(ctx: &egui::Context) -> Self {
        let viewport = ctx.input(|i| i.viewport().clone());

        Self {
            viewport,
            rust_version: var("RUSTC_VERSION").ok(),
            target_triple: var("TARGET").ok(),
            target_arch: var("CARGO_CFG_TARGET_ARCH").ok(),
            target_os: var("CARGO_CFG_TARGET_OS").ok(),
            target_family: var("CARGO_CFG_TARGET_FAMILY").ok(),
            target_pointer_width: var("CARGO_CFG_TARGET_POINTER_WIDTH").ok(),
            target_endian: var("CARGO_CFG_TARGET_ENDIAN").ok(),
            debug_assertions: cfg!(debug_assertions),
        }
    }
}
