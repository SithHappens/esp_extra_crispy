//! # Rust Esp
//!
//! `rust_esp` is my collection of embedded tools for the ESP32.

#![no_std]
#![no_main]


/// Error module
pub mod error;

/// Module for the rotary button
pub mod rotary_button_sync;

// Re-exports
pub use error::{Error, Result};
pub use rotary_button_sync::{ButtonEvent, RotaryButton, RotationEvent};


// TODO Maybe this template is useful for testing: https://github.com/knurling-rs/app-template/

// 26.12 - 30.12
