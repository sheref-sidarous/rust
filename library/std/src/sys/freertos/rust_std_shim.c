
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
#include "semphr.h"

void rust_std_taskYIELD(void)
{
  taskYIELD();
}

SemaphoreHandle_t rust_std_xSemaphoreCreateCounting(UBaseType_t uxMaxCount,
                                                    UBaseType_t uxInitialCount)
{
  return xSemaphoreCreateCounting(uxMaxCount, uxInitialCount);
}

SemaphoreHandle_t rust_std_xSemaphoreCreateMutex(void)
{
  return xSemaphoreCreateMutex();
}

void rust_std_xSemaphoreTake(SemaphoreHandle_t xSemaphore,
                    TickType_t xTicksToWait)
{
  xSemaphoreTake(xSemaphore, xTicksToWait);
}

void rust_std_xSemaphoreGive(SemaphoreHandle_t xSemaphore)
{
  xSemaphoreGive(xSemaphore);
}

void rust_std_vSemaphoreDelete(SemaphoreHandle_t xSemaphore)
{
  vSemaphoreDelete(xSemaphore);
}

TickType_t rust_std_msec_to_ticks (uint32_t millis) {
  return millis * portTICK_PERIOD_MS;
}
