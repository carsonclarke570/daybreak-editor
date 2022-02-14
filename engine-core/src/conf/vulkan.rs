use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct VulkanConfig {
  pub instance: VulkanInstanceConfig
}

#[derive(Debug, Deserialize)]
pub struct VulkanInstanceConfig {
  pub app_name: String,
  pub app_version: u32,
  pub engine_name: String,
  pub engine_version: u32,
  pub api_version: (u32, u32, u32)
}