#include <Arduino.h>
#include <accelerometer.h>
#include <freertos/task.h>

static accelerometer_values_t values;

void setup() {
    Serial.begin(115200);
    while (!Serial);

    printf("Starting up\n");
    init_accelerometer();
}

void loop() {
    read_accelerometer(&values);
    printf("0x");

    printf("%f:%f:%f:%f:%f:%f:%f\n",
           values.delta_yaw, values.delta_pitch, values.delta_roll,
           values.yaw, values.pitch, values.roll,
           values.temperature);
    vTaskDelay(100 / portTICK_PERIOD_MS);

    if (Serial.available()) {
        const char c = static_cast<char>(Serial.read());
        // react to whoami command
        if (c == 'w') {
            printf("raw/accelerometer\n");
        }
    }
}
