#![allow(non_upper_case_globals)]

use super::unsupported;
use crate::ffi::CStr;
use crate::io;
use crate::num::NonZeroUsize;
use crate::time::Duration;
use crate::sys::freertos::freertos_api;
use crate::ffi::{c_void, c_char};

pub struct Thread(freertos_api::TaskHandle_t);
struct ThreadDescriptor {
    entry : Box<dyn FnOnce()>,
    is_running : bool,
    // a join handle
}

pub const DEFAULT_MIN_STACK_SIZE: usize = 4096;

#[allow(non_camel_case_types)]
type TickType_t = u32;


extern "C" fn thread_entry (arg : *mut c_void) {


    let mut thread_descriptor = unsafe { Box::from_raw(arg as *mut ThreadDescriptor) };


    thread_descriptor.is_running = true;
    (thread_descriptor.entry)();
    thread_descriptor.is_running = false;
}

impl Thread {
    // unsafe: see thread::Builder::spawn_unchecked for safety requirements
    pub unsafe fn new(stack: usize, p: Box<dyn FnOnce()>) -> io::Result<Thread> {

        let arg : *mut ThreadDescriptor= Box::into_raw(Box::new(ThreadDescriptor {
            entry: p,
            is_running : false,
        }));

        let mut thread_handle : freertos_api::TaskHandle_t = core::ptr::null_mut();

        let r = freertos_api::xTaskCreate(
            thread_entry,  /* entry point */
            core::ptr::null(),  /* Task name */
            stack as freertos_api::configSTACK_DEPTH_TYPE, /*  */
            arg as *mut c_void, freertos_api::DefaultTaskPriority, /* */
            &mut thread_handle as *mut freertos_api::TaskHandle_t); /* get the handle back here */

        if r == freertos_api::pdPASS {
            // Success !
            io::Result::Ok(Thread (thread_handle))
        } else {
            io::Result::Err(io::Error::from_raw_os_error(r))
        }
        
    }

    pub fn yield_now() {
        unsafe { freertos_api::rust_std_taskYIELD(); }
    }

    pub fn set_name(_name: &CStr) {
        panic!("implement me!");
    }

    pub fn sleep(dur: Duration) {
        unsafe { 
            freertos_api::vTaskDelay(dur.as_millis() as u32 * freertos_api::portTICK_PERIOD_MS)
        }
    }

    pub fn join(self) {
        panic!("implement me!");
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
