use crate::app::GameApp;
use anyhow::Context;
use std::path::Path;

pub mod migration;

impl GameApp {
    pub fn create_save(&self) -> anyhow::Result<Vec<u8>> {
        let uncompressed = rmp_serde::to_vec_named(self).context("Failed to serialize app")?;
        Ok(zstd::encode_all(&*uncompressed, 22)?)
    }

    pub fn restore_save(&mut self, save: &[u8]) -> anyhow::Result<()> {
        let uncompressed = zstd::decode_all(save).context("Failed to decompress save")?;
        *self = rmp_serde::from_slice(&uncompressed).context("Failed to deserialize save")?;
        Ok(())
    }

    pub fn save_to_file(&self, path: &Path) -> anyhow::Result<()> {
        if !path.exists() {
            std::fs::create_dir_all(path.parent().unwrap())?;
            std::fs::File::create(path)?;
        }

        let data = self.create_save()?;
        std::fs::write(path, data)?;
        Ok(())
    }

    pub fn restore_from_file(&mut self, path: &Path) -> anyhow::Result<()> {
        if !path.exists() {
            return Ok(());
        }

        let data = std::fs::read(path)?;
        self.restore_save(&data)?;
        Ok(())
    }
}
