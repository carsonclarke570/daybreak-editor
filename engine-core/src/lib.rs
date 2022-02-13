use waiter_di::*;

mod interface;
mod provider;
mod module;

use crate::interface::vulkan::ContextInterface;

pub fn test() {
    let mut container = Container::<profiles::Dev>::new();
    let vulkan_context = Provider::<dyn ContextInterface>::create(&mut container);

    println!("{}", vulkan_context.version());
}