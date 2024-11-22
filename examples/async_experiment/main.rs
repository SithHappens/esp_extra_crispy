#![no_std]
#![no_main]
#![feature(waker_getters)] // used in `ExtWaker`

use core::{
    future::Future,
    pin::{pin, Pin},
    task::{Context, Poll},
    time::Duration,
};

use defmt::info;
use esp_backtrace as _;
use esp_hal::{entry, rtc_cntl::sleep::TimerWakeupSource};
use esp_println as _;
use executor::{Executor, ExtWaker, TaskId};


pub mod channel;
mod executor;
mod rtc;

use rtc::RtcRef as Rtc;


// region:    --- Delay

enum TimerState {
    Init,
    Wait,
}

struct MyTimer {
    state: TimerState,
}

impl MyTimer {
    // TODO actually implement to wait for duration
    fn new() -> Self {
        Self {
            state: TimerState::Init,
        }
    }

    fn register(&self, _task_id: TaskId) {
        // schedule_wakeup rtc.borrow_ref_mut(cs)
        // TODO how do I schedule the interrupt with rtc.sleep_light(...)?
    }
}

impl Future for MyTimer {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.state {
            TimerState::Init => {
                self.register(cx.waker().task_id());
                self.state = TimerState::Wait;
                Poll::Pending
            }
            TimerState::Wait => {
                if false
                /*time::now() >= self.end_time*/
                {
                    Poll::Ready(())
                } else {
                    Poll::Pending
                }
            }
        }
    }
}

pub async fn delay_millis(_duration: u16) {
    MyTimer::new().await;
}

// endregion: --- Delay


async fn async_number() -> u32 {
    42
}

async fn num_task() {
    let number = async_number().await;
    info!("Async number: {}", number);
}
/*
async fn dot_task(rtc: &Rtc<'_>) {
    loop {
        info!(".");
        delay_millis(rtc, 1000).await;
    }
}
*/

//Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    Rtc::init(peripherals.LPWR);

    info!("Going to sleep for 5s");
    let wake_source = TimerWakeupSource::new(Duration::new(5, 0));
    Rtc::with_guard(|rtc| rtc.sleep_light(&[&wake_source]));
    info!("Hello, ESP!");

    //let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);

    let task = pin!(num_task());
    //let dot_task = pin!(dot_task(&rtc));

    //let mut tasks: &mut [&mut dyn Future<Output = ()>; 1] = &mut [&mut async_task()];

    let mut executor = Executor::new();
    executor.run(&mut [task])
}
