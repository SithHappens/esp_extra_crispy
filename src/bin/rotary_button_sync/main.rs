#![no_std]
#![no_main]

use core::cell::RefCell;

use critical_section::Mutex;
use defmt::{info, warn, Format};
use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    entry,
    gpio::{self, Event, Input, InputPin, Io, Level, Output, Pull},
    interrupt,
    peripheral::Peripheral,
    peripherals::Interrupt,
    rtc_cntl::Rtc,
    InterruptConfigurable,
};
use esp_println as _;
use fugit::ExtU64;
use rust_esp::{ButtonEvent, Error, RotaryButton, RotationEvent, Ticker};


#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    info!("Hello, ESP!");
    warn!("value: {}", Error::GenericError("hi"));

    let mut io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut rotary = RotaryButton::new(io.pins.gpio22, io.pins.gpio23, io.pins.gpio21);

    let a = 5;

    Ticker::init(peripherals.LPWR, 1000u64.millis());
    Ticker::register_callback(|| {
        info!("Hello, {} s", Ticker::now());
    });

    let delay = Delay::new();

    loop {
        let (rotation, button) = rotary.update();
        match rotation {
            RotationEvent::StepClockwise | RotationEvent::StepCounterClockwise => {
                info!("{}", rotation)
            }
            _ => {}
        }
        match button {
            ButtonEvent::Pressed | ButtonEvent::Released => info!("{}", button),
            _ => {}
        }

        delay.delay_millis(1u32);
    }
}
