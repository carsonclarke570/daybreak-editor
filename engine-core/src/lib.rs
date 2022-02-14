mod conf;
mod vulkan;

use std::path::Path;

use crate::{conf::EngineConfig, vulkan::VulkanApp};
use figment::{Figment, providers::{Format, Yaml}};

pub struct Engine {
    pub config: EngineConfig
}

impl Engine {
    
    pub fn new(config_dir: String) -> Self {
        let config_path = Path::new(config_dir.as_str()).join("default.yaml");
        let config = Figment::new()
            .merge(Yaml::file(config_path))
            .extract::<EngineConfig>();

        if config.is_err() {
            log::error!("Failed to read config directory '{:?}': {:?}", config_dir, config);
            panic!("{:?}", config);
        }
        Self {
            config: config.unwrap()
        }
    }

    pub fn info(&self) {
        log::info!("Using default config of version: {}", self.config.version);
    }
}

pub fn test() {
    let engine: Engine = Engine::new("engine-core/config/".to_string());
    let mut app = VulkanApp::new();
    app.init(&engine.config);
    app.run();
}
