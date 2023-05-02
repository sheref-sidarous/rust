#![allow(non_upper_case_globals)]

use super::unsupported;
use crate::ffi::CStr;
use crate::io;
use crate::num::NonZeroUsize;
use crate::time::Duration;

pub struct Thread(!);

pub const DEFAULT_MIN_STACK_SIZE: usize = 4096;

#[allow(non_camel_case_types)]
type TickType_t = u32;

extern "C" {
    pub fn vTaskDelay(xTicksToDelay : TickType_t);
}

// from FreeRTOS/FreeRTOS/Demo/CORTEX_MPS2_QEMU_IAR_GCC/FreeRTOSConfig.h
const configTICK_RATE_HZ : TickType_t = 1000;

// from FreeRTOS/FreeRTOS/Source/portable/GCC/ARM_CM3/portmacro.h
const portTICK_PERIOD_MS : TickType_t = 1000 / configTICK_RATE_HZ;

impl Thread {
    // unsafe: see thread::Builder::spawn_unchecked for safety requirements
    pub unsafe fn new(_stack: usize, _p: Box<dyn FnOnce()>) -> io::Result<Thread> {
        unsupported()
    }

    pub fn yield_now() {
        // do nothing
    }

    pub fn set_name(_name: &CStr) {
        // nope
    }

    pub fn sleep(dur: Duration) {
        unsafe { 
            vTaskDelay(dur.as_millis() as u32 * portTICK_PERIOD_MS)
        }
    }

    pub fn join(self) {
        self.0
    }
}

pub fn available_parallelism() -> io::Result<NonZeroUsize> {
    unsupported()
}

pub mod guard {
    pub type Guard = !;
    pub unsafe fn current() -> Option<Guard> {
        None
    }
    pub unsafe fn init() -> Option<Guard> {
        None
    }
}
