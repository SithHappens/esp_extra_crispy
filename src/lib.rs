//! # Rust Esp
//!
//! `rust_esp` is my collection of embedded tools for the ESP32.
//!
//! TODO Maybe this template is useful for testing: https://github.com/knurling-rs/app-template/

#![no_std]
#![no_main]
#![feature(waker_getters)] // used in `ExtWaker`


mod channel;

pub mod error;
pub mod executor;

// Re-exports
pub use error::{Error, Result};
