use std::ffi::CString;

use ash::{vk, Entry};

use crate::{conf::VulkanInstanceConfig, util};

use super::ENABLE_VALIDATION_LAYERS;

pub struct VulkanInstance {
    instance: ash::Instance,
}

impl VulkanInstance {
    pub fn new(entry: &Entry, config: &VulkanInstanceConfig) -> Self {
        VulkanInstance {
            instance: create_instance(entry, config),
        }
    }

    pub fn get(&self) -> &ash::Instance {
        &self.instance
    }
}

impl Drop for VulkanInstance {
    fn drop(&mut self) {
        unsafe {
            self.instance.destroy_instance(None);
        }
    }
}

fn create_instance(entry: &Entry, config: &VulkanInstanceConfig) -> ash::Instance {
    let app_name = CString::new(config.app_name.to_string()).unwrap();
    let engine_name = CString::new(config.engine_name.to_string()).unwrap();
    let app_info = vk::ApplicationInfo::builder()
        .application_name(&app_name)
        .application_version(config.app_version)
        .engine_name(&engine_name)
        .engine_version(config.engine_version)
        .api_version(ash::vk::make_api_version(
            0,
            config.api_version.0,
            config.api_version.1,
            config.api_version.2,
        ));

    let extension_names = util::platforms::required_extension_names(ENABLE_VALIDATION_LAYERS);
    let mut create_info = vk::InstanceCreateInfo::builder()
        .application_info(&app_info)
        .enabled_extension_names(&extension_names);

    let layer_names: Vec<std::ffi::CString> =
        vec![std::ffi::CString::new("VK_LAYER_KHRONOS_validation").unwrap()];
    let layer_name_pointers: Vec<*const i8> = layer_names
        .iter()
        .map(|layer_name| layer_name.as_ptr())
        .collect();

    if ENABLE_VALIDATION_LAYERS {
        create_info = create_info.enabled_layer_names(&layer_name_pointers);
    }

    let instance: ash::Instance = unsafe {
        let result = entry.create_instance(&create_info, None);
        if result.is_err() {
            log::error!("Failed to initialize instance.");
            panic!("{:?}", result.err());
        }
        log::info!("Successfuly initialized instance with config: {:?}", config);
        result.unwrap()
    };

    instance
}
