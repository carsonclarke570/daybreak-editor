use ash::Entry;

use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Fullscreen;

use crate::conf::{EngineConfig, WindowConfig};

use super::{VulkanDebugUtil, VulkanInstance, VulkanPhysicalDevice, REQUIRED_LAYERS, VulkanLogicalDevice, VulkanSurface};

pub struct VulkanApp {
    event_loop: EventLoop<()>,
    _window: winit::window::Window,
    _vk_instance: VulkanInstance,
    _vk_debug: VulkanDebugUtil,
    _vk_phy_device: VulkanPhysicalDevice,
    _vk_log_device: VulkanLogicalDevice,
    _vk_surface: VulkanSurface
}

impl VulkanApp {
    pub fn new(config: &EngineConfig) -> VulkanApp {
        let entry = Entry::linked();
        VulkanDebugUtil::validate_layer_support(&entry, REQUIRED_LAYERS);

        let main_loop = EventLoop::new();
        let main_window = VulkanApp::init_window(&main_loop, &config.window);
        let instance = VulkanInstance::new(&entry, &config.vulkan.instance);
        let debug_util = VulkanDebugUtil::new(&entry, &instance);
        let this_surface = VulkanSurface::new(&entry, &instance, &main_window);
        let this_phys_device = VulkanPhysicalDevice::new(&instance, &this_surface, &config.vulkan.physical_device);
        let this_log_device = VulkanLogicalDevice::new(&instance, &this_phys_device, &this_surface);
        VulkanApp {
            event_loop: main_loop,
            _window: main_window,
            _vk_instance: instance,
            _vk_debug: debug_util,
            _vk_phy_device: this_phys_device,
            _vk_log_device: this_log_device,
            _vk_surface: this_surface
        }
    }

    fn init_window(event_loop: &EventLoop<()>, config: &WindowConfig) -> winit::window::Window {
        let fullscreen: Option<Fullscreen> = if config.fullscreen {
            Some(Fullscreen::Borderless(None))
        } else {
            None
        };
        let result = winit::window::WindowBuilder::new()
            .with_title(config.title.to_string())
            .with_inner_size(winit::dpi::LogicalSize::new(config.width, config.height))
            .with_fullscreen(fullscreen)
            .build(event_loop);

        if result.is_err() {
            log::error!("Failed to initialize window.");
            panic!("{:?}", result);
        }
        log::info!("Successfuly initialized window with config: {:?}", config);
        result.unwrap()
    }

    pub fn run(self) {
        log::info!("Beginning game loop");
        self.event_loop.run(move |event, _, control_flow| {
            if let Event::WindowEvent { event, .. } = event {
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::KeyboardInput { input, .. } => {
                        let KeyboardInput {
                            virtual_keycode,
                            state,
                            ..
                        } = input;
                        if let (Some(VirtualKeyCode::Escape), ElementState::Pressed) =
                            (virtual_keycode, state)
                        {
                            dbg!();
                            *control_flow = ControlFlow::Exit
                        };
                    }
                    _ => {}
                }
            }
        })
    }
}
