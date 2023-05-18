
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::ffi::{c_void, c_char};
use crate::ptr;

pub type TickType_t = u32;
pub type BaseType_t = i32;
pub type configSTACK_DEPTH_TYPE = u16;
pub type UBaseType_t = u32;
pub type TaskFunction_t = extern "C" fn (*mut c_void);
pub type SemaphoreHandle_t = *mut c_void;
pub type TaskHandle_t = *mut c_void;

pub const portMAX_DELAY : TickType_t = 0xffffffffu32;
pub const queueQUEUE_TYPE_MUTEX : u8 = 1;
pub const semGIVE_BLOCK_TIME : TickType_t = 0;
pub const queueSEND_TO_BACK : BaseType_t = 0;

// TODO: better Freertos task priority handling
pub const DefaultTaskPriority : UBaseType_t = 5u32;

// from FreeRTOS/FreeRTOS/Demo/CORTEX_MPS2_QEMU_IAR_GCC/FreeRTOSConfig.h
pub const configTICK_RATE_HZ : TickType_t = 1000;

// from FreeRTOS/FreeRTOS/Source/portable/GCC/ARM_CM3/portmacro.h
pub const portTICK_PERIOD_MS : TickType_t = 1000 / configTICK_RATE_HZ;

extern "C" {

    //pub fn xSemaphoreCreateMutex() -> SemaphoreHandle_t;
    pub fn xQueueCreateMutex( mx_type : u8 ) -> SemaphoreHandle_t;

    //pub fn xSemaphoreTake(xSemaphore : SemaphoreHandle_t, xTicksToWait : TickType_t) -> bool;
    pub fn xQueueSemaphoreTake(xSemaphore : SemaphoreHandle_t, xTicksToWait : TickType_t) -> bool;

    //pub fn xSemaphoreGive(xSemaphore : SemaphoreHandle_t);
    pub fn xQueueGenericSend(xSemaphore : SemaphoreHandle_t, pvItemToQueue : *const c_void ,
        xTicksToWait : TickType_t,
        xCopyPosition : BaseType_t);

    //pub fn vSemaphoreDelete(xSemaphore : SemaphoreHandle_t);
    pub fn vQueueDelete(xSemaphore : SemaphoreHandle_t);

    pub fn vTaskDelay(xTicksToDelay : TickType_t);
    pub fn uart_write ( buff : *const u8, buff_len : usize);

    pub fn xTaskCreate (pxTaskCode : TaskFunction_t,
        pcName : *const c_char,
        usStackDepth : configSTACK_DEPTH_TYPE,
        pvParameters : *const c_void,
        uxPriority : UBaseType_t,
        pxCreatedTask : *mut TaskHandle_t ) -> BaseType_t;

}

pub unsafe fn xSemaphoreCreateMutex() -> SemaphoreHandle_t {
    xQueueCreateMutex( queueQUEUE_TYPE_MUTEX )
}

pub unsafe fn xSemaphoreTake(xSemaphore : SemaphoreHandle_t, xTicksToWait : TickType_t) -> bool {
    xQueueSemaphoreTake(xSemaphore, xTicksToWait)
}

pub unsafe fn xSemaphoreGive(xSemaphore : SemaphoreHandle_t) {
    xQueueGenericSend( xSemaphore, ptr::null(), semGIVE_BLOCK_TIME, queueSEND_TO_BACK )
}

pub unsafe fn vSemaphoreDelete(xSemaphore : SemaphoreHandle_t) {
    vQueueDelete(xSemaphore)

}
