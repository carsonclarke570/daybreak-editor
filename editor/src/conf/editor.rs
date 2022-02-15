use engine_core::conf::{VulkanConfig, WindowConfig};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct EditorConfig {
  pub version: i32,
  pub window: WindowConfig,
  pub vulkan: VulkanConfig
}