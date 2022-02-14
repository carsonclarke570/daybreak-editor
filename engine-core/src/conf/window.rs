use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WindowConfig {
  pub title: String,
  pub width: i32,
  pub height: i32,
  pub fullscreen: bool
}