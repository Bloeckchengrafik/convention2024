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
    // print as long hex string
    printf("0x");
    for (int j = 0; j < sizeof(values); j++) {
        printf("%02x", reinterpret_cast<uint8_t *>(&values)[j]);
    }
    printf("\n");
    vTaskDelay(100 / portTICK_PERIOD_MS);

    if (Serial.available()) {
        const char c = static_cast<char>(Serial.read());
        // react to whoami command
        if (c == 'w') {
            printf("raw/accelerometer\n");
        }
    }
}
