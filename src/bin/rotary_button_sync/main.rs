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
    macros::handler,
    peripheral::Peripheral,
    peripherals::Interrupt,
    InterruptConfigurable,
};
use esp_println as _;
use rust_esp::{ButtonEvent, Error, RotaryButton, RotationEvent};


// see https://rtic.rs/2/api/critical_section/struct.Mutex.html#design
//static BUTTON: Mutex<RefCell<Option<Input>>> = Mutex::new(RefCell::new(None));


#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    info!("Hello, ESP!");
    warn!("value: {}", Error::GenericError("hi"));

    let mut io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
    //io.set_interrupt_handler(interrupt_handler);

    /*
    let mut button = Input::new(io.pins.gpio21, Pull::Up);

    critical_section::with(|cs| {
        button.listen(Event::FallingEdge);
        BUTTON.borrow_ref_mut(cs).replace(button);
    });
    */

    let delay = Delay::new();

    let mut rotary = RotaryButton::new(io.pins.gpio22, io.pins.gpio23, io.pins.gpio21);

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

/*
#[handler]
fn interrupt_handler() {
    critical_section::with(|cs| {
        info!("Button pressed");
        BUTTON
        .borrow_ref_mut(cs)
        .as_mut()
        .unwrap()
        .clear_interrupt();
    });
}
*/
