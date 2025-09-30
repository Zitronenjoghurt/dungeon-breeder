use crate::app::GameApp;
use crate::{app_save_file_path, meta_save_file_path};
use anyhow::Context;
use dungeon_breeder_core::VERSION_INDEX;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistenceMetadata {
    pub version: u32,
}

impl Default for PersistenceMetadata {
    fn default() -> Self {
        Self {
            version: VERSION_INDEX,
        }
    }
}

impl PersistenceMetadata {
    pub fn save(&self) -> anyhow::Result<()> {
        let path = meta_save_file_path();
        if !path.exists() {
            std::fs::create_dir_all(path.parent().unwrap())?;
            std::fs::File::create(&path)?;
        }

        let data = rmp_serde::to_vec_named(self)?;
        std::fs::write(&path, &data)?;
        Ok(())
    }

    pub fn load() -> anyhow::Result<Self> {
        if !meta_save_file_path().exists() {
            return Ok(Self::default());
        }

        let data = std::fs::read(meta_save_file_path())?;
        Ok(rmp_serde::from_slice(&data)?)
    }
}

// Load app with migrations
impl PersistenceMetadata {
    pub fn load_app(&self) -> anyhow::Result<GameApp> {
        if self.version < VERSION_INDEX {
            return Ok(GameApp::default());
        }

        let mut app = GameApp::default();
        app.restore_from_file(&app_save_file_path())
            .context("Failed to restore save")?;
        app.persistence_metadata = self.clone();
        Ok(app)
    }
}
