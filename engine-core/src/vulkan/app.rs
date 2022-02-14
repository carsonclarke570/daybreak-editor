use std::ffi::CString;
use std::ptr;

use ash::{vk, Entry};

use winit::event::{Event, VirtualKeyCode, ElementState, KeyboardInput, WindowEvent};
use winit::event_loop::{EventLoop, ControlFlow};
use winit::window::Fullscreen;

use crate::conf::{WindowConfig, EngineConfig, VulkanInstanceConfig};
use crate::util;

pub struct VulkanApp {
    event_loop: EventLoop<()>,
    entry: Option<Entry>,
    window: Option<winit::window::Window>,
    instance: Option<ash::Instance>
}

impl VulkanApp {
    pub fn new() -> VulkanApp {
        VulkanApp {
            event_loop: EventLoop::new(),
            entry: None,
            window: None,
            instance: None
        }
    }

    pub fn init(&mut self, config: &EngineConfig) {
        self.window = Some(VulkanApp::init_window(&self.event_loop, &config.window));
        self.entry = Some(Entry::linked());
        self.instance = Some(VulkanApp::init_instance(&self.entry.as_ref().unwrap(), &config.vulkan.instance))
    }

    fn init_window(event_loop: &EventLoop<()>, config: &WindowConfig) -> winit::window::Window {
        let fullscreen: Option<Fullscreen> = if config.fullscreen { Some(Fullscreen::Borderless(None)) } else { None }; 
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

    fn init_instance(entry: &Entry, config: &VulkanInstanceConfig) -> ash::Instance {
        let app_name = CString::new(config.app_name.to_string()).unwrap();
        let engine_name = CString::new(config.engine_name.to_string()).unwrap();
        let app_info = ash::vk::ApplicationInfo {
            s_type: ash::vk::StructureType::APPLICATION_INFO,
            p_next: ptr::null(),
            p_application_name: app_name.as_ptr(),
            application_version: config.app_version,
            p_engine_name: engine_name.as_ptr(),
            engine_version: config.engine_version,
            api_version: ash::vk::make_api_version(0, 
                config.api_version.0, 
                config.api_version.1, 
                config.api_version.2),
        };

        let extension_names = util::platforms::required_extension_names();
        let create_info = vk::InstanceCreateInfo {
            s_type: vk::StructureType::INSTANCE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::InstanceCreateFlags::empty(),
            p_application_info: &app_info,
            pp_enabled_layer_names: ptr::null(),
            enabled_layer_count: 0,
            pp_enabled_extension_names: extension_names.as_ptr(),
            enabled_extension_count: extension_names.len() as u32,
        };

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

    pub fn run(self) {
        log::info!("Beginning game loop");
        self.event_loop.run(move |event, _, control_flow| {
            match event {
                | Event::WindowEvent { event, .. } => {
                    match event {
                        | WindowEvent::CloseRequested => {
                            *control_flow = ControlFlow::Exit
                        },
                        | WindowEvent::KeyboardInput { input, .. } => {
                            match input {
                                | KeyboardInput { virtual_keycode, state, .. } => {
                                    match (virtual_keycode, state) {
                                        | (Some(VirtualKeyCode::Escape), ElementState::Pressed) => {
                                            dbg!();
                                            *control_flow = ControlFlow::Exit
                                        },
                                        | _ => {},
                                    }
                                },
                            }
                        },
                        | _ => {},
                    }
                },
                _ => (),
            }

        })
    }
}

// impl Drop for VulkanApp {
//     fn drop(&mut self) {
//         unsafe {
//             self.instance.unwrap().destroy_instance(None);
//         }
//     }
// }