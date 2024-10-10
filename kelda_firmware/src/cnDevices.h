#pragma once

#include <VL53L0X.h>

VL53L0X throttleSensor;
TaskHandle_t measurementTaskHandle;
volatile uint16_t throttleValue = 0;

void throttleMeasurementTask(void *pvParameters)
{
    while (1)
    {
        if (throttleSensor.timeoutOccurred())
        {
            throttleValue = 0xFFFF;
        }
        else
        {
            throttleValue = throttleSensor.readRangeContinuousMillimeters();
        }
        vTaskDelay(100 / portTICK_PERIOD_MS);
    }
}

void beginThrottle()
{
    throttleSensor.setTimeout(500);
    if (!throttleSensor.init())
    {
        Serial.println("Failed to detect and initialize sensor!");
        while (1);
    }

    throttleSensor.startContinuous(100);

    xTaskCreate(
        throttleMeasurementTask,
        "THROTTLE",
        4096,
        NULL,
        1,
        &measurementTaskHandle
    );
}

