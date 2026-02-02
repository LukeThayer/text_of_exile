pub mod combat;
pub mod enemy;
pub mod inventory;
pub mod player;

use anyhow::Result;
use loot_core::{Config as LootConfig, Generator};
use std::path::Path;
use tables_core::DropTableRegistry;

/// Central game configuration holding all Obelisk crate configs
pub struct GameConfig {
    pub generator: Generator,
    pub tables: DropTableRegistry,
}

impl GameConfig {
    pub fn load() -> Result<Self> {
        let config_path = Path::new("config");

        // Load loot_core config
        let loot_config = LootConfig::load_from_dir(config_path)
            .map_err(|e| anyhow::anyhow!("Failed to load loot config: {}", e))?;
        let generator = Generator::new(loot_config);

        // Load tables_core config
        let tables = DropTableRegistry::load(&config_path.join("tables"))
            .map_err(|e| anyhow::anyhow!("Failed to load drop tables: {}", e))?;

        Ok(Self { generator, tables })
    }
}
