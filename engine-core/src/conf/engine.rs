use serde_derive::Deserialize;

use super::{VulkanConfig, WindowConfig};

#[derive(Debug, Deserialize)]
pub struct EngineConfig {
  pub version: i32,
  pub window: WindowConfig,
  pub vulkan: VulkanConfig
}