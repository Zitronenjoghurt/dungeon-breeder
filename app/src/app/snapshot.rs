use crate::app::GameApp;
use anyhow::Context;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameAppSnapshot {
    app: Vec<u8>,
    egui: Vec<u8>,
}

impl GameAppSnapshot {
    pub fn create(app: &GameApp, ctx: &egui::Context) -> anyhow::Result<Self> {
        let app = app.create_save()?;
        let egui =
            ctx.memory(|mem| rmp_serde::to_vec_named(&mem).context("Failed to serialize egui"))?;

        Ok(Self { app, egui })
    }

    pub fn restore(&self, game_app: &mut GameApp, ctx: &egui::Context) -> anyhow::Result<()> {
        game_app.restore_save(&self.app)?;

        if !self.egui.is_empty() {
            ctx.memory_mut(|mem| {
                if let Ok(data) = rmp_serde::from_slice(&self.egui) {
                    *mem = data;
                }
            });
        }

        GameApp::setup_context(ctx);

        Ok(())
    }

    pub fn export_rmp(&self) -> anyhow::Result<Vec<u8>> {
        zstd::encode_all(
            &*rmp_serde::to_vec(&self).context("Failed to serialize snapshot")?,
            22,
        )
        .context("Failed to compress snapshot")
    }

    pub fn import_rmp(data: &[u8]) -> anyhow::Result<Self> {
        rmp_serde::from_slice(&zstd::decode_all(data).context("Failed to decompress snapshot")?)
            .context("Failed to deserialize snapshot")
    }
}
