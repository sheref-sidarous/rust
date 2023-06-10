#![allow(non_upper_case_globals)]

use super::unsupported;
use crate::ffi::CStr;
use crate::io;
use crate::num::NonZeroUsize;
use crate::time::Duration;
use crate::sys::freertos::freertos_api;
use crate::ffi::{c_void, c_char};
use crate::collections::HashMap;
use crate::boxed::Box;

pub struct Thread {
    handle : freertos_api::TaskHandle_t,
    join_semaphore : freertos_api::SemaphoreHandle_t,
}
struct ThreadDescriptor {
    entry : Box<dyn FnOnce()>,
    is_running : bool,
    join_semaphore : freertos_api::SemaphoreHandle_t,
}

pub const DEFAULT_MIN_STACK_SIZE: usize = 4096;

#[allow(non_camel_case_types)]
type TickType_t = u32;


extern "C" fn thread_entry (arg : *mut c_void) {


    let mut thread_descriptor = unsafe { Box::from_raw(arg as *mut ThreadDescriptor) };


    thread_descriptor.is_running = true;
    (thread_descriptor.entry)();
    thread_descriptor.is_running = false;

    unsafe {
        freertos_api::rust_std_xSemaphoreGive(thread_descriptor.join_semaphore);
        freertos_api::rust_std_vTaskDelete( core::ptr::null_mut() );
    }
}

impl Thread {
    // unsafe: see thread::Builder::spawn_unchecked for safety requirements
    pub unsafe fn new(stack: usize, p: Box<dyn FnOnce()>) -> io::Result<Thread> {

        let join_semaphore = freertos_api::rust_std_xSemaphoreCreateMutex();

        let arg : *mut ThreadDescriptor= Box::into_raw(Box::new(ThreadDescriptor {
            entry: p,
            is_running : false,
            join_semaphore : join_semaphore.clone(),
        }));

        let mut thread_handle : freertos_api::TaskHandle_t = core::ptr::null_mut();

        let r = freertos_api::xTaskCreate(
            thread_entry,  /* entry point */
            core::ptr::null(),  /* Task name */
            stack as freertos_api::configSTACK_DEPTH_TYPE, /*  */
            arg as *mut c_void, freertos_api::DefaultTaskPriority, /* */
            &mut thread_handle as *mut freertos_api::TaskHandle_t); /* get the handle back here */

        if r == freertos_api::pdPASS {

            // create the Hashmap for TLS
            let list : Box<Vec<*mut u8>> = Box::new(Vec::new());
            freertos_api::rust_std_vTaskSetThreadLocalStoragePointer(
                thread_handle, 0, Box::into_raw(list) as *mut c_void);
            // Success !
            io::Result::Ok(Thread {
                handle : thread_handle,
                join_semaphore : join_semaphore,
            })
        } else {
            io::Result::Err(io::Error::from_raw_os_error(r))
        }

    }

    pub fn yield_now() {
        unsafe { freertos_api::rust_std_taskYIELD(); }
    }

    pub fn set_name(_name: &CStr) {
        // ignore name setting
    }

    pub fn sleep(dur: Duration) {
        unsafe {
            freertos_api::vTaskDelay(
                freertos_api::rust_std_msec_to_ticks(dur.as_millis() as u32))
        }
    }

    pub fn join(self) {
        unsafe {
            freertos_api::rust_std_xSemaphoreTake(self.join_semaphore, freertos_api::portMAX_DELAY);
        }
    }
}

pub fn available_parallelism() -> io::Result<NonZeroUsize> {
    unsafe { io::Result::Ok(NonZeroUsize::new_unchecked(1)) }
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
