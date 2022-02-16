use ash::vk;

use std::ffi::CString;
use std::os::raw::c_char;

use super::{VulkanInstance, VulkanPhysicalDevice, ENABLE_VALIDATION_LAYERS, REQUIRED_LAYERS};

pub struct VulkanLogicalDevice {
    logical_device: ash::Device,
    queue: vk::Queue,
}

impl VulkanLogicalDevice {
    pub fn new(instance: &VulkanInstance, physical_device: &VulkanPhysicalDevice) -> Self {
        let (this_device, this_queue) = Self::create_logical_device(instance, physical_device);
        VulkanLogicalDevice {
            logical_device: this_device,
            queue: this_queue,
        }
    }

    pub fn get_device(&self) -> &ash::Device {
        &self.logical_device
    }

    pub fn get_queue(&self) -> &vk::Queue {
        &self.queue
    }

    fn create_logical_device(
        instance: &VulkanInstance,
        physical_device: &VulkanPhysicalDevice,
    ) -> (ash::Device, vk::Queue) {
        let indices = VulkanPhysicalDevice::find_queue_index(instance, *physical_device.get());

        let queue_priorities = [1.0_f32];
        let queue_create_info = [vk::DeviceQueueCreateInfo::builder()
            .flags(vk::DeviceQueueCreateFlags::empty())
            .queue_family_index(indices.graphics_family.unwrap())
            .queue_priorities(&queue_priorities)
            .build()];

        let physical_device_features = vk::PhysicalDeviceFeatures {
            ..Default::default()
        };

        let requred_validation_layer_raw_names: Vec<CString> = REQUIRED_LAYERS
            .iter()
            .map(|layer_name| CString::new(*layer_name).unwrap())
            .collect();
        let enable_layer_names: Vec<*const c_char> = requred_validation_layer_raw_names
            .iter()
            .map(|layer_name| layer_name.as_ptr())
            .collect();

        let mut device_create_info_builder = vk::DeviceCreateInfo::builder()
            .flags(vk::DeviceCreateFlags::empty())
            .queue_create_infos(&queue_create_info)
            .enabled_features(&physical_device_features);

        if ENABLE_VALIDATION_LAYERS {
            device_create_info_builder =
                device_create_info_builder.enabled_layer_names(&enable_layer_names);
        }
        let device_create_info = device_create_info_builder.build();

        let device: ash::Device = unsafe {
            let result =
                instance
                    .get()
                    .create_device(*physical_device.get(), &device_create_info, None);
            if result.is_err() {
                log::error!("Failed to create logical devices.");
                panic!("{:?}", result.err());
            }
            result.unwrap()
        };

        let graphics_queue =
            unsafe { device.get_device_queue(indices.graphics_family.unwrap(), 0) };

        log::info!("Successfully initialized logical device and graphics queue");
        (device, graphics_queue)
    }
}

impl Drop for VulkanLogicalDevice {
    fn drop(&mut self) {
        unsafe {
            self.logical_device.destroy_device(None);
        }
        log::debug!("Sucessfully destroyed logical device");
    }
}
