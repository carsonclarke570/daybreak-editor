use serde_derive::Deserialize;

use crate::vulkan::{QueueFlagSupportMatrix, DeviceFeatureSupportMatrix};

#[derive(Debug, Deserialize)]
pub struct VulkanConfig {
  pub instance: VulkanInstanceConfig,
  pub physical_device: VulkanPhysicalDeviceConfig
}

#[derive(Debug, Deserialize)]
pub struct VulkanInstanceConfig {
  pub app_name: String,
  pub app_version: u32,
  pub engine_name: String,
  pub engine_version: u32,
  pub api_version: (u32, u32, u32)
}

#[derive(Debug, Deserialize)]
pub struct VulkanPhysicalDeviceConfig {
  pub desired_queue_flags: QueueFlagSupportMatrix,
  pub desired_device_features: DeviceFeatureSupportMatrix
}