// rustarray30 - Array30 Input Method in Rust
//行列 30 輸入法實作

pub mod dict;
pub mod input_engine;
pub mod keymap;
pub mod state;

// 平台特定模組
#[cfg(target_os = "windows")]
pub mod gui;

#[cfg(not(target_os = "windows"))]
pub mod console;

pub use input_engine::InputEngine;
pub use state::InputState;
