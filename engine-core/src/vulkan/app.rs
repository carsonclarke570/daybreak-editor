use ash::Entry;

use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Fullscreen;

use crate::conf::{EngineConfig, WindowConfig};

use super::{VulkanDebugUtil, VulkanInstance, REQUIRED_LAYERS};

pub struct VulkanApp {
    event_loop: EventLoop<()>,
    _window: winit::window::Window,
    _vk_instance: VulkanInstance,
    _vk_debug: VulkanDebugUtil,
}

impl VulkanApp {
    pub fn new(config: &EngineConfig) -> VulkanApp {
        let entry = Entry::linked();
        VulkanDebugUtil::validate_layer_support(&entry, REQUIRED_LAYERS);

        let main_loop = EventLoop::new();
        let main_window = VulkanApp::init_window(&main_loop, &config.window);
        log::info!("HERE");
        let instance = VulkanInstance::new(&entry, &config.vulkan.instance);
        log::info!("HERE1");
        let debug_util = VulkanDebugUtil::new(&entry, &instance);
        log::info!("HERE2");
        VulkanApp {
            event_loop: main_loop,
            _window: main_window,
            _vk_instance: instance,
            _vk_debug: debug_util,
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
