#![no_std]
#![no_main]

use defmt::{info, warn};
use esp_backtrace as _;
use esp_hal::{delay::Delay, entry};
use esp_println as _;
use fugit::ExtU64;
use rotary_button_sync::{ButtonEvent, RotaryButton, RotationEvent};
use rust_esp::Error;
use time::Ticker;

mod rotary_button_sync;
mod time;


#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    info!("Hello, ESP!");
    warn!("value: {}", Error::Generic("hi"));

    let mut rotary = RotaryButton::new(peripherals.GPIO22, peripherals.GPIO23, peripherals.GPIO21);


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
