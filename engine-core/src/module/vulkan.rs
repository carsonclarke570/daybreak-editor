use crate::interface::ContextInterface;

use waiter_di::*;

#[module]
pub struct VulkanModule {
  context: Box<dyn ContextInterface>
}