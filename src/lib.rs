//! # Rust Esp
//!
//! `rust_esp` is my collection of embedded tools for the ESP32.

#![no_std]
#![no_main]


mod error;
mod rotary_button_sync;
mod time;

// Re-exports
pub use error::{Error, Result};
pub use rotary_button_sync::{ButtonEvent, RotaryButton, RotationEvent};
pub use time::Ticker;


// TODO Maybe this template is useful for testing: https://github.com/knurling-rs/app-template/

// 26.12 - 30.12
// Franzi wegen 22.12 und 4.1. oder 11.1 hin oder rückfahrt
