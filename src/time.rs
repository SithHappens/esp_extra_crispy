use core::cell::RefCell;

use critical_section::Mutex;
use defmt::info;
use esp_hal::{
    delay::MicrosDurationU64,
    macros::handler,
    peripheral::Peripheral,
    peripherals::LPWR,
    rtc_cntl::{Rtc, Rwdt},
    InterruptConfigurable,
};


type CallbackFn = fn();


static TICKER: Ticker = Ticker {
    rtc: Mutex::new(RefCell::new(None)),
    callback: Mutex::new(RefCell::new(None)),
};


// see https://rtic.rs/2/api/critical_section/struct.Mutex.html#design
pub struct Ticker<'a> {
    rtc: Mutex<RefCell<Option<Rtc<'a>>>>,
    callback: Mutex<RefCell<Option<CallbackFn>>>,
}

impl<'a> Ticker<'a> {
    pub fn init(lpwr: LPWR, timeout: MicrosDurationU64) {
        let mut rtc = Rtc::new(lpwr);
        rtc.set_interrupt_handler(interrupt_handler);
        rtc.rwdt.set_timeout(timeout);
        rtc.rwdt.listen();
        critical_section::with(|cs| {
            TICKER.rtc.replace(cs, Some(rtc));
        });
    }

    pub fn register_callback(cb: CallbackFn) {
        critical_section::with(|cs| TICKER.callback.replace(cs, Some(cb)));
    }

    /// Timestamp in seconds, with milliseconds precision
    pub fn now() -> f64 {
        critical_section::with(|cs| {
            let mut rtc = TICKER.rtc.borrow_ref_mut(cs);
            let rtc = rtc.as_mut().unwrap();
            rtc.current_time().and_utc().timestamp_millis() as f64 / 1000f64
        })
    }
}


#[handler]
fn interrupt_handler() {
    critical_section::with(|cs| {
        let mut rtc = TICKER.rtc.borrow_ref_mut(cs);
        let rtc = rtc.as_mut().unwrap();
        rtc.rwdt.clear_interrupt();
        //rwdt.disable();  // call disable to let the watchdog expire only once
    });

    critical_section::with(|cs| {
        let mut cb = TICKER.callback.borrow_ref_mut(cs);
        if let Some(cb) = cb.as_mut() {
            cb();
        }
    });
}
