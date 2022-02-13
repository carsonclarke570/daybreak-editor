use crate::module::VulkanModule;

use waiter_di::*;

#[module]
pub struct RootModule {
    vulakn_module: VulkanModule
}