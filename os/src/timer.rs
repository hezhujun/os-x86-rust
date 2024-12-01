use alloc::sync::Arc;
use alloc::collections::BinaryHeap;
use core::cmp::PartialOrd;
use core::cmp::Ordering;
use spin::Mutex;

use crate::{arch::x86::outb, drivers::{rdtsc, read_hour, read_minute, read_second}, process::TaskControlBlock, schedule::wakeup_task};


const INPUT_FREQUENCY: usize = 1193180;
const IRQ0_FREQUENCY: usize = 100;
const COUNTER0_VALUE: usize = INPUT_FREQUENCY / IRQ0_FREQUENCY;
const COUNTER0_PORT: u16 = 0x40;
const PIT_CONTROL_PORT: u16 = 0x43;

pub fn init() {
    outb(0u8 << 6 | 3 << 4 | 2 << 1, PIT_CONTROL_PORT);
    outb((COUNTER0_VALUE & 0xff) as u8, COUNTER0_PORT);
    outb(((COUNTER0_VALUE >> 8) & 0xff) as u8, COUNTER0_PORT);
    _ = START_TIME.lock();
}

lazy_static! {
    static ref ELAPSED_TIME: Arc<Mutex<u64>> = Arc::new(Mutex::new(0));
    static ref START_TIME: Arc<Mutex<u64>> = Arc::new(Mutex::new(
        current_timestamp()
    ));
}

fn current_timestamp() -> u64 {
    let hours = read_hour() as u64;
    let minutes = read_minute() as u64;
    let seconds = read_second() as u64;
    hours * 60 * 60 + minutes * 60 + seconds
}

pub fn update_time() {
    let tick = 1_000_000 / IRQ0_FREQUENCY as u64;
    let mut elasped_time = ELAPSED_TIME.lock();
    let start_time = START_TIME.lock();
    let elasped_time_real = (current_timestamp() - *start_time) * 1_000_000;
    if (*elasped_time + tick) < elasped_time_real {
        *elasped_time = elasped_time_real;
    } else {
        *elasped_time += tick;
    }
}

pub fn get_time_in_millisecond() -> u64 {
    get_time_in_microsecond() / 1000
}

pub fn get_time_in_microsecond() -> u64 {
    let elasped_time = ELAPSED_TIME.lock();
    *elasped_time
}

pub struct TimerCondVar {
    pub expire_ms: u64,
    pub task: Arc<TaskControlBlock>,
}

impl PartialEq for TimerCondVar {
    fn eq(&self, other: &Self) -> bool {
        self.expire_ms == other.expire_ms
    }
}
impl Eq for TimerCondVar {}
impl PartialOrd for TimerCondVar {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let a = -(self.expire_ms as i64);
        let b = -(other.expire_ms as i64);
        Some(a.cmp(&b))
    }
}

impl Ord for TimerCondVar {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

lazy_static! {
    static ref TIMERS: Arc<Mutex<BinaryHeap<TimerCondVar>>> = Arc::new(Mutex::new(BinaryHeap::<TimerCondVar>::new()));
}

pub fn add_timer(expire_ms: u64, task: Arc<TaskControlBlock>) {
    let mut timers = TIMERS.lock();
    timers.push(TimerCondVar { expire_ms, task });
}

pub fn check_timer() {
    let current_ms = get_time_in_millisecond();
    let mut timers = TIMERS.lock();
    while let Some(timer) = timers.peek() {
        if timer.expire_ms <= current_ms {
            wakeup_task(timer.task.clone());
            timers.pop();
        } else {
            break;
        }
    }
}
