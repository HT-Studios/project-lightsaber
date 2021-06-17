use core::time::Duration;

use lightsaber_util::libcore_ext::io;

#[cfg(target_arch = "x86_64")]
use crate::architecture::amd64::interrupts;

pub const DEFAULT_PROGRAMMABLE_INTERVAL_TIMER_DIVISOR: u64 = 65535;
pub const PROGRAMMABLE_INTERVAL_TIMER_BASE_FREQUENCY: u64 = 1193182;
pub const PROGRAMMABLE_INTERVAL_TIMER_DIVISOR: u64 = 1193180;

pub static mut PROGRAMMABLE_INTERVAL_TIMER: ProgrammableIntervalTimerDescriptor = ProgrammableIntervalTimerDescriptor::new();

pub struct ProgrammableIntervalTimerDescriptor {
    ticks_since_epoch: u64,
    divisor: u64
}

impl ProgrammableIntervalTimerDescriptor {
    #[inline]
    const fn new() -> Self {
        Self {
            ticks_since_epoch: 0,
            divisor: DEFAULT_PROGRAMMABLE_INTERVAL_TIMER_DIVISOR
        }
    }

    #[inline(always)]
    pub fn get_frequency(&self) -> u64 {
        PROGRAMMABLE_INTERVAL_TIMER_BASE_FREQUENCY / self.divisor
    }

    pub unsafe fn set_divisor(&mut self, divisor: u64) {
        io::outb(0x40, (divisor & 0x00FF) as u8);
        io::wait();

        io::outb(0x40, ((divisor & 0xFF00) >> 8) as u8);
        io::wait();
    }

    pub fn sleep(&mut self, duration: Duration) {
        let start_time = self.ticks_since_epoch;
        let seconds = duration.as_secs();

        while self.ticks_since_epoch < start_time + seconds {
            interrupts::pause();
        }
    }

    #[inline(always)]
    pub fn tick(&mut self) {
        self.ticks_since_epoch += 1 / self.get_frequency();
    }
}

pub fn initialize_programmable_interval_timer() {
    unsafe {
        PROGRAMMABLE_INTERVAL_TIMER.set_divisor(PROGRAMMABLE_INTERVAL_TIMER_DIVISOR);
    }
}
