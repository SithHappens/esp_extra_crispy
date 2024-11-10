//! # Rotary Button
//!
//! Implements synchronous support for rotary buttons like the KY-040.

use defmt::Format;
use esp_hal::{
    gpio::{Input, InputPin, Pull},
    peripheral::Peripheral,
    peripherals::GPIO,
};


type RotationState = u8;
type ButtonState = u8;


/// Abstraction for the rotary buttons, e.g. the KY-040 rotary encoder.
///
/// # Example
/// ```rust
/// assert_eq!(true, false);
/// let peripherals = esp_hal::init(esp_hal::Config::default());
/// let mut io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
/// let mut rotary = RotaryButton::new(io.pins.gpio22, io.pins.gpio23, io.pins.gpio21);
///
/// loop {
///     let (rotation, button) = rotary.update();
/// }
/// ```
pub struct RotaryButton<'a> {
    pin_dt: Input<'a>,
    pin_clk: Input<'a>,
    pin_sw: Input<'a>,
    rotation_state: RotationState,
    button_state: ButtonState,
}


impl<'a> RotaryButton<'a> {
    /// Creates a new [`RotaryButton`] instance.
    pub fn new<A, B, C>(pin_dt: A, pin_clk: B, pin_sw: C) -> Self
    where
        A: InputPin + Peripheral<P = A> + 'a,
        B: InputPin + Peripheral<P = B> + 'a,
        C: InputPin + Peripheral<P = C> + 'a,
    {
        Self {
            pin_dt: Input::new(pin_dt, Pull::Up),
            pin_clk: Input::new(pin_clk, Pull::Up),
            pin_sw: Input::new(pin_sw, Pull::Up),
            rotation_state: 0b11_11_11_11,
            button_state: 0b11,
        }
    }

    /// Check the pins for an update synchronously, returns ([`RotationEvent`], [`ButtonEvent`]).
    pub fn update(&mut self) -> (RotationEvent, ButtonEvent) {
        // Rotation
        let mut s = 0b11;

        if self.pin_dt.is_low() {
            s &= 0b01;
        }

        if self.pin_clk.is_low() {
            s &= 0b10;
        }

        if s != (self.rotation_state & 0b11) || s == 0b11 {
            self.rotation_state = (self.rotation_state << 2) | s;
        }

        // Button
        let button_level = self.pin_sw.get_level() as u8;
        self.button_state = (self.button_state << 1) | button_level;

        (self.rotation_state.into(), self.button_state.into())
    }
}


/// Rotation event, returned by `RotaryButton`
#[derive(Format)]
pub enum RotationEvent {
    StepClockwise,
    StepCounterClockwise,
    None,
}

impl From<RotationState> for RotationEvent {
    fn from(state: RotationState) -> Self {
        match state {
            0b01_00_10_11 => RotationEvent::StepClockwise,
            0b10_00_01_11 => RotationEvent::StepCounterClockwise,
            _ => RotationEvent::None,
        }
    }
}


/// Button event, returned by `RotaryButton`
#[derive(Format)]
pub enum ButtonEvent {
    Pressed,
    Released,
    None,
}

impl From<ButtonState> for ButtonEvent {
    fn from(state: ButtonState) -> Self {
        match state & 0b11 {
            0b10 => ButtonEvent::Pressed,
            0b01 => ButtonEvent::Released,
            _ => ButtonEvent::None,
        }
    }
}
