use std::fmt::Display;

use ash::vk::{self, QueueFamilyProperties};
use serde_derive::Deserialize;

use crate::{conf::VulkanPhysicalDeviceConfig, util};

use super::{VulkanInstance, VulkanSurface};

#[derive(Debug, Deserialize)]
pub struct QueueFlagSupportMatrix {
    pub graphics: bool,
    pub compute: bool,
    pub transfer: bool,
    pub sparse: bool,
}

impl QueueFlagSupportMatrix {
    pub fn new(queue_family: &QueueFamilyProperties) -> Self {
        QueueFlagSupportMatrix {
            graphics: queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS),
            compute: queue_family.queue_flags.contains(vk::QueueFlags::COMPUTE),
            transfer: queue_family.queue_flags.contains(vk::QueueFlags::TRANSFER),
            sparse: queue_family
                .queue_flags
                .contains(vk::QueueFlags::SPARSE_BINDING),
        }
    }
}

impl Display for QueueFlagSupportMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "graphics: {}, compute: {}, transfer: {}, sparse: {}",
            self.graphics, self.compute, self.transfer, self.sparse
        )
    }
}

#[derive(Debug, Deserialize)]
pub struct DeviceFeatureSupportMatrix {
    pub geometry_shader: bool,
}

impl DeviceFeatureSupportMatrix {
    pub fn new(features: vk::PhysicalDeviceFeatures) -> Self {
        DeviceFeatureSupportMatrix {
            geometry_shader: features.geometry_shader == 1,
        }
    }
}

impl Display for DeviceFeatureSupportMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "geometry_shader: {}", self.geometry_shader)
    }
}

#[derive(Default)]
pub struct QueueFamilyIndices {
    pub graphics_family: Option<u32>,
    pub compute_family: Option<u32>,
    pub transfer_family: Option<u32>,
    pub sparse_family: Option<u32>,
    pub present_family: Option<u32>,
}

impl QueueFamilyIndices {
    pub fn new(
        instance: &VulkanInstance,
        surface: &VulkanSurface,
        vk_physical_device: vk::PhysicalDevice,
    ) -> Self {
        Self::create(instance, surface, vk_physical_device)
    }

    pub fn is_complete(&self, required_matrix: &QueueFlagSupportMatrix) -> bool {
        self.is_part_complete(required_matrix.graphics, self.graphics_family)
            && self.is_part_complete(required_matrix.transfer, self.transfer_family)
            && self.is_part_complete(required_matrix.sparse, self.sparse_family)
            && self.is_part_complete(required_matrix.compute, self.compute_family)
            && self.is_part_complete(true, self.present_family)
    }

    fn is_part_complete(&self, required: bool, optional: Option<u32>) -> bool {
        if required {
            optional.is_some()
        } else {
            true
        }
    }

    pub fn create(
        instance: &VulkanInstance,
        surface: &VulkanSurface,
        vk_physical_device: vk::PhysicalDevice,
    ) -> Self {
        let queue_families = unsafe {
            instance
                .get()
                .get_physical_device_queue_family_properties(vk_physical_device)
        };
        let mut indices = Self::default();
        for (index, queue_family) in queue_families.iter().enumerate() {
            let support_matrix = QueueFlagSupportMatrix::new(queue_family);
            let idx = index as u32;
            indices.graphics_family =
                indices.mark_one(support_matrix.graphics, indices.graphics_family, idx);
            indices.compute_family =
                indices.mark_one(support_matrix.compute, indices.compute_family, idx);
            indices.transfer_family =
                indices.mark_one(support_matrix.transfer, indices.transfer_family, idx);
            indices.sparse_family =
                indices.mark_one(support_matrix.sparse, indices.sparse_family, idx);

            let present_support = unsafe {
                let result = surface.get_loader().get_physical_device_surface_support(
                    vk_physical_device,
                    idx,
                    *surface.get_surface(),
                );
                result.unwrap()
            };
            indices.present_family = indices.mark_one(present_support, indices.present_family, idx);
        }

        indices
    }

    fn mark_one(&self, is_available: bool, optional: Option<u32>, index: u32) -> Option<u32> {
        if is_available {
            Some(index)
        } else {
            optional
        }
    }
}

pub struct VulkanPhysicalDevice {
    physical_device: vk::PhysicalDevice,
}

impl VulkanPhysicalDevice {
    pub fn new(
        instance: &VulkanInstance,
        surface: &VulkanSurface,
        config: &VulkanPhysicalDeviceConfig,
    ) -> Self {
        VulkanPhysicalDevice {
            physical_device: Self::create_physical_device(instance, surface, config),
        }
    }

    pub fn get(&self) -> &vk::PhysicalDevice {
        &self.physical_device
    }

    fn create_physical_device(
        instance: &VulkanInstance,
        surface: &VulkanSurface,
        config: &VulkanPhysicalDeviceConfig,
    ) -> vk::PhysicalDevice {
        let physical_devices = unsafe {
            let result = instance.get().enumerate_physical_devices();
            if result.is_err() {
                log::error!("Failed to enumerate physical devices.");
                panic!("{:?}", result.err());
            }
            result.unwrap()
        };
        log::debug!(
            "Discovered {} devices (GPU) with Vulkan support",
            physical_devices.len()
        );

        let result = physical_devices.iter().find(|device| {
            Self::is_device_suitable(instance, **device, surface, &config.desired_queue_flags)
        });

        match result {
            None => panic!("Failed to find a suitable GPU!"),
            Some(physical_device) => {
                log::info!(
                    "Successfully initialized physical device with config: {:?}",
                    config
                );
                *physical_device
            }
        }
    }

    fn is_device_suitable(
        instance: &VulkanInstance,
        vk_physical_device: vk::PhysicalDevice,
        surface: &VulkanSurface,
        required_flags: &QueueFlagSupportMatrix,
    ) -> bool {
        let properties = unsafe {
            instance
                .get()
                .get_physical_device_properties(vk_physical_device)
        };
        let features = unsafe {
            instance
                .get()
                .get_physical_device_features(vk_physical_device)
        };
        let queue_families = unsafe {
            instance
                .get()
                .get_physical_device_queue_family_properties(vk_physical_device)
        };

        let device_type = match properties.device_type {
            vk::PhysicalDeviceType::CPU => "Cpu",
            vk::PhysicalDeviceType::INTEGRATED_GPU => "Integrated GPU",
            vk::PhysicalDeviceType::DISCRETE_GPU => "Discrete GPU",
            vk::PhysicalDeviceType::VIRTUAL_GPU => "Virtual GPU",
            vk::PhysicalDeviceType::OTHER => "Unknown",
            _ => panic!("Unable to determine GPU type"),
        };

        let device_name = util::string::c_char_arr_to_string(&properties.device_name);
        log::debug!(
            "Device Name: {}, id: {}, type: {}",
            device_name,
            properties.device_id,
            device_type
        );

        let major_version = vk::api_version_major(properties.api_version);
        let minor_version = vk::api_version_minor(properties.api_version);
        let patch_version = vk::api_version_patch(properties.api_version);
        log::debug!(
            "API Version: {}.{}.{}",
            major_version,
            minor_version,
            patch_version
        );
        log::debug!("Num Queue Families: {}", queue_families.len());

        for queue_family in queue_families.iter() {
            let queue_matrix = QueueFlagSupportMatrix::new(queue_family);
            log::debug!(
                "Queue Flag Support {} - {}",
                queue_family.queue_count,
                queue_matrix
            );
        }

        let device_matrix = DeviceFeatureSupportMatrix::new(features);
        log::debug!("Device Feature Support - {}", device_matrix);

        log::debug!("Required - {:?}", required_flags);
        QueueFamilyIndices::new(instance, surface, vk_physical_device).is_complete(required_flags)
    }
}
