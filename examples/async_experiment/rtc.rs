use core::cell::RefCell;

use critical_section::Mutex;
use esp_hal::{peripherals::LPWR, rtc_cntl::Rtc};


static RTC: RtcRef = RtcRef {
    rtc: Mutex::new(RefCell::new(None)),
};


pub struct RtcRef<'a> {
    rtc: Mutex<RefCell<Option<Rtc<'a>>>>,
}

impl<'a> RtcRef<'a> {
    pub fn init(lpwr: LPWR) {
        let rtc = Rtc::new(lpwr);
        critical_section::with(|cs| {
            RTC.rtc.replace(cs, Some(rtc));
        });
    }

    pub fn with_guard<R>(f: impl FnOnce(&mut Rtc<'_>) -> R) -> R {
        critical_section::with(|cs| {
            let mut rtc = RTC.rtc.borrow_ref_mut(cs);
            let rtc = rtc.as_mut().unwrap();
            f(rtc)
        })
    }
}
