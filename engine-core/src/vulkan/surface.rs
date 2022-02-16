use ash::vk;

use crate::util;

use super::VulkanInstance;

pub struct VulkanSurface {
    surface_loader: ash::extensions::khr::Surface,
    surface: vk::SurfaceKHR,
}

impl VulkanSurface {
    pub fn new(
        entry: &ash::Entry,
        instance: &VulkanInstance,
        window: &winit::window::Window,
    ) -> Self {
        let (this_loader, this_surface) = VulkanSurface::create_surface(entry, instance, window);
        VulkanSurface {
            surface_loader: this_loader,
            surface: this_surface,
        }
    }

    pub fn get_loader(&self) -> &ash::extensions::khr::Surface {
        &self.surface_loader
    }

    pub fn get_surface(&self) -> &vk::SurfaceKHR {
        &self.surface
    }

    fn create_surface(
        entry: &ash::Entry,
        instance: &VulkanInstance,
        window: &winit::window::Window,
    ) -> (ash::extensions::khr::Surface, vk::SurfaceKHR) {
        let surface = unsafe {
            let result = util::platforms::create_surface(entry, instance.get(), window);
            if result.is_err() {
                log::error!("Failed to create surface.");
                panic!("{:?}", result.err());
            }
            result.unwrap()
        };
        let surface_loader = ash::extensions::khr::Surface::new(entry, instance.get());
        log::info!("Successfully initialized surface");
        (surface_loader, surface)
    }
}

impl Drop for VulkanSurface {
    fn drop(&mut self) {
        unsafe {
            self.surface_loader.destroy_surface(self.surface, None);
        }
        log::debug!("Sucessfully destroyed surface");
    }
}
