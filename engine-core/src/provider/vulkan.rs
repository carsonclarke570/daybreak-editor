use crate::interface::*;
use waiter_di::*;

#[component]
pub struct Context {}

#[provides]
impl ContextInterface for Context {
  fn version(&self) -> &str {
    "Version 1"
  }
}