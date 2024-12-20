use core::{
    future::Future,
    pin::Pin,
    task::{Context, RawWaker, RawWakerVTable, Waker},
};

use defmt::{trace, warn};
use heapless::mpmc::Q32;

use crate::Rtc;

pub type TaskId = usize;

static TASK_QUEUE: Q32<TaskId> = Q32::new();


// region:    --- Waker

/* Idea
#[interrupt]
fn some_interrupt_handler() {
    WAKER.wake();
}
*/

/// Extension trait to extract the task_id from the Waker
pub trait ExtWaker {
    fn task_id(&self) -> TaskId;
}

impl ExtWaker for Waker {
    fn task_id(&self) -> TaskId {
        // uses unstable feature `waker-getters`
        return self.as_raw().data() as TaskId;
    }
}

fn get_waker(task_id: TaskId) -> Waker {
    // Safety: Data argument interpreted as TaskId, not dereferenced.
    unsafe { Waker::from_raw(RawWaker::new(task_id as *const (), &VTABLE)) }
}

static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);

unsafe fn clone(p: *const ()) -> RawWaker {
    RawWaker::new(p, &VTABLE)
}

// `p` is essentially free 32 bits and we can fill it with whatever we want.
// So we choose it to be the `task_id`
unsafe fn wake(p: *const ()) {
    wake_task(p as TaskId);
}

unsafe fn wake_by_ref(p: *const ()) {
    wake_task(p as TaskId);
}

unsafe fn drop(_p: *const ()) {}

pub fn wake_task(task_id: TaskId) {
    trace!("Waking task {}", task_id);
    if TASK_QUEUE.enqueue(task_id).is_err() {
        panic!("Task queue full: can't add task {}", task_id);
    }
}

// endregion: --- Waker


pub struct Executor {
    //heapless
}

impl Executor {
    pub fn new() -> Self {
        Self {}
    }

    //pub fn spawn_task() {}

    pub fn run(&mut self, tasks: &mut [Pin<&mut dyn Future<Output = ()>>]) -> ! {
        // make sure every task is ran at least once
        for task_id in 0..tasks.len() {
            TASK_QUEUE.enqueue(task_id).ok();
        }

        loop {
            while let Some(task_id) = TASK_QUEUE.dequeue() {
                if task_id >= tasks.len() {
                    warn!("Bad task id: {}", task_id);
                    continue;
                }
                trace!("Running task {}", task_id);
                let waker = get_waker(task_id);
                let _ = tasks[task_id]
                    .as_mut()
                    .poll(&mut Context::from_waker(&waker));
            }
            trace!("No tasks ready, going to sleep");

            Rtc::with_guard(|rtc| {
                // TODO specify WakeSource
                rtc.sleep_light(&[]);
            });

            trace!("Waking up");
        }
    }
}
