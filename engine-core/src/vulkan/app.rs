use winit::event::{Event, VirtualKeyCode, ElementState, KeyboardInput, WindowEvent};
use winit::event_loop::{EventLoop, ControlFlow};
use winit::window::Fullscreen;

use crate::conf::{WindowConfig, EngineConfig};

pub struct VulkanApp {
    event_loop: EventLoop<()>,
    window: Option<winit::window::Window>
}

impl VulkanApp {
    pub fn new() -> VulkanApp {
        VulkanApp{
            event_loop: EventLoop::new(),
            window: None
        }
    }

    pub fn init(&mut self, config: &EngineConfig) {
        self.window = Some(VulkanApp::init_window(&self.event_loop, &config.window))
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
        log::info!("Successfuly initialized window.");
        result.unwrap()
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