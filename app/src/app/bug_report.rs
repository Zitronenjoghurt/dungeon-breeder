use crate::app::runtime_info::RuntimeInfo;
use crate::app::snapshot::GameAppSnapshot;
use crate::app::system_info::SystemInfo;
use crate::app::GameApp;
use anyhow::Context;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BugReportMetadata {
    pub title: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BugReport {
    pub metadata: BugReportMetadata,
    pub snapshot: GameAppSnapshot,
    pub runtime: RuntimeInfo,
    pub system: SystemInfo,
}

impl BugReport {
    pub fn create(
        metadata: BugReportMetadata,
        app: &GameApp,
        ctx: &egui::Context,
    ) -> anyhow::Result<Self> {
        let snapshot = GameAppSnapshot::create(app, ctx)?;
        let runtime = RuntimeInfo::collect(ctx);
        let system = SystemInfo::collect();

        Ok(Self {
            metadata,
            snapshot,
            runtime,
            system,
        })
    }

    pub fn export_rmp(&self) -> anyhow::Result<Vec<u8>> {
        zstd::encode_all(
            &*rmp_serde::to_vec(&self).context("Failed to serialize bug report")?,
            22,
        )
        .context("Failed to compress bug report")
    }

    pub fn import_rmp(data: &[u8]) -> anyhow::Result<Self> {
        rmp_serde::from_slice(&zstd::decode_all(data).context("Failed to decompress bug report")?)
            .context("Failed to deserialize bug report")
    }
}
