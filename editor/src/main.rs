mod gui;
mod conf;

use std::path::Path;

use conf::EditorConfig;
use figment::{Figment, providers::{Yaml, Format}};
use engine_core::vulkan::VulkanApp;
use gui::EditorUI;
use iced::{Settings, Application};
use simple_logger::SimpleLogger;


struct Editor {
    config: EditorConfig,
    //vulkan_app: VulkanApp
}

impl Editor {
    pub fn new(config_dir: String) -> Self {
        let config_path = Path::new(config_dir.as_str()).join("default.yaml");
        let config = Figment::new()
            .merge(Yaml::file(config_path))
            .extract::<EditorConfig>();

        if config.is_err() {
            log::error!("Failed to read config directory '{:?}': {:?}", config_dir, config);
            panic!("{:?}", config);
        }
        Self {
            config: config.unwrap(),
            //vulkan_app: VulkanApp::default()
        }
    }

    pub fn info(&self) {
        log::info!("Using default config of version: {}", self.config.version);
    }

    pub fn run(&self) -> iced::Result {
        EditorUI::run(Settings::default())
    }
}

fn main() {
    SimpleLogger::new().init().unwrap();
    engine_core::test();
    // let editor: Editor = Editor::new("editor/config/".to_string());
    // editor.info();
    // editor.run()
}
