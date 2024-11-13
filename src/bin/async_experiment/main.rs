#![no_std]
#![no_main]

use core::{
    future::Future,
    pin::{pin, Pin},
    task::{Context, Poll},
    time::Duration,
};

use defmt::{info, trace, Format};
use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    entry,
    reset::SleepSource,
    rtc_cntl::{get_wakeup_cause, sleep::TimerWakeupSource, Rtc},
};
use esp_println as _;
use rust_esp::executor::{self, Executor, ExtWaker, TaskId};


// region:    --- Delay

enum TimerState {
    Init,
    Wait,
}

struct Timer<'a> {
    rtc: &'a Rtc<'a>,
    state: TimerState,
}

impl<'a> Timer<'a> {
    // TODO actually implement to wait for duration
    fn new(rtc: &'a Rtc<'_>) -> Self {
        Self {
            rtc: rtc,
            state: TimerState::Init,
        }
    }

    fn register(&self, task_id: TaskId) {
        // schedule_wakeup rtc.borrow_ref_mut(cs)
        // TODO how do I schedule the interrupt with rtc.sleep_light(...)?
    }
}

impl<'a> Future for Timer<'a> {
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

pub async fn delay_millis<'a>(rtc: &Rtc<'_>, duration: u16) {
    Timer::new(rtc).await;
}

// endregion: --- Delay


async fn async_number() -> u32 {
    42
}

async fn async_task() {
    let number = async_number().await;
    info!("Async number: {}", number);
}

async fn dot_task(rtc: &Rtc<'_>) {
    loop {
        info!(".");
        delay_millis(rtc, 1000).await;
    }
}


#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let mut rtc = Rtc::new(peripherals.LPWR);

    info!("Going to sleep for 5s");
    let wake_source = TimerWakeupSource::new(Duration::new(5, 0));
    rtc.sleep_light(&[&wake_source]);
    info!("Hello, ESP!");

    //let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);

    let task = pin!(async_task());
    let dot_task = pin!(dot_task(&rtc));

    //let mut tasks: &mut [&mut dyn Future<Output = ()>; 1] = &mut [&mut async_task()];

    let executor = Executor::new(&rtc);
    executor.run(&mut [dot_task, task])
}
