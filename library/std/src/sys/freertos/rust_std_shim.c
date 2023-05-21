
/*
Why this file is needed ?
* To capture Freertos configurations that are expressed as #defines in its headers
  for example: configTICK_RATE_HZ
* Many Freertos APIs resolve to other APIs via macros, for example  
  xSemaphoreCreateMutex is a macro to xQueueCreateMutex( queueQUEUE_TYPE_MUTEX )



*/

/* Standard includes. */
#include <stdio.h>

/* Kernel includes. */
#include "FreeRTOS.h"
#include "task.h"
#include "timers.h"
#include "queue.h"

void rust_std_taskYIELD(void) {
    taskYIELD();
}
