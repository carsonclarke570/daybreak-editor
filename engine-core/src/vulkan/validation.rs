use std::{
    ffi::{c_void, CStr},
    ptr, 
};

use ash::{
    extensions::ext::DebugUtils,
    vk::{self},
    Entry,
};

use super::VulkanInstance;

#[cfg(debug_assertions)]
pub const ENABLE_VALIDATION_LAYERS: bool = true;
#[cfg(not(debug_assertions))]
pub const ENABLE_VALIDATION_LAYERS: bool = false;

pub const REQUIRED_LAYERS: [&str; 1] = ["VK_LAYER_KHRONOS_validation"];

unsafe extern "system" fn vulkan_debug_callback(
    flag: vk::DebugUtilsMessageSeverityFlagsEXT,
    typ: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _: *mut c_void,
) -> vk::Bool32 {
    use vk::DebugUtilsMessageSeverityFlagsEXT as Flag;

    let message = CStr::from_ptr((*p_callback_data).p_message);
    match flag {
        Flag::VERBOSE => log::debug!("{:?} - {:?}", typ, message),
        Flag::INFO => log::info!("{:?} - {:?}", typ, message),
        Flag::WARNING => log::warn!("{:?} - {:?}", typ, message),
        _ => log::error!("{:?} - {:?}", typ, message),
    }
    vk::FALSE
}

pub struct VulkanDebugUtil {
    util: Option<(DebugUtils, vk::DebugUtilsMessengerEXT)>,
}

impl VulkanDebugUtil {
    pub fn new(entry: &Entry, instance: &VulkanInstance) -> Self {
        VulkanDebugUtil {
            util: create_debug_messenger(entry, instance),
        }
    }

    pub fn get(&self) -> &Option<(DebugUtils, vk::DebugUtilsMessengerEXT)> {
        &self.util
    }

    pub fn validate_layer_support(entry: &Entry, layers: [&str; 1]) {
        for required in layers.iter() {
            let found = entry
                .enumerate_instance_layer_properties()
                .unwrap()
                .iter()
                .any(|layer| {
                    let name = unsafe { CStr::from_ptr(layer.layer_name.as_ptr()) };
                    let name = name.to_str().expect("Failed to get layer name pointer");
                    required == &name
                });

            if !found {
                panic!("Validation layer not supported: {}", required);
            }
        }
    }
}

impl Drop for VulkanDebugUtil {
    fn drop(&mut self) {
        unsafe {
            if let Some((utils, messenger)) = self.util.take() {
                utils.destroy_debug_utils_messenger(messenger, None);
            }
        }
        log::debug!("Sucessfully destroyed debug utils");
    }
}

pub fn create_debug_messenger(
    entry: &Entry,
    instance: &VulkanInstance,
) -> Option<(DebugUtils, vk::DebugUtilsMessengerEXT)> {
    if !ENABLE_VALIDATION_LAYERS {
        return None;
    }
    let create_info = vk::DebugUtilsMessengerCreateInfoEXT {
        s_type: vk::StructureType::DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT,
        p_next: ptr::null(),
        flags: vk::DebugUtilsMessengerCreateFlagsEXT::empty(),
        message_severity: vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE
            | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
            | vk::DebugUtilsMessageSeverityFlagsEXT::ERROR,
        message_type: vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
            | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
            | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
        pfn_user_callback: Some(vulkan_debug_callback),
        p_user_data: ptr::null_mut(),
    };

    let debug_utils = DebugUtils::new(entry, instance.get());
    let debug_utils_messenger = unsafe {
        debug_utils
            .create_debug_utils_messenger(&create_info, None)
            .unwrap()
    };

    Some((debug_utils, debug_utils_messenger))
}
