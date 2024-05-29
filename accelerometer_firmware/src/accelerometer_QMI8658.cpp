//
// Created by chris on 5/29/24.
//


#include "accelerometer.h"

#if !CONFIG_USE_LSM6DSR

#include <SensorQMI8658.hpp>

#define SDA GPIO_NUM_6
#define SCL GPIO_NUM_7
#define INT1 GPIO_NUM_4
#define INT2 GPIO_NUM_3

SensorQMI8658 qmi;

void init_accelerometer() {
    printf("Initializing QMI8658\n");
    if (!qmi.begin(Wire, 0x6b, SDA, SCL)) {
        printf("QMI8658 not found\n");
        return;
    }

    printf("QMI8658 found\n");
    printf("Chip ID: %d\n", qmi.getChipID());

    qmi.configAccelerometer(
        SensorQMI8658::ACC_RANGE_2G,
        SensorQMI8658::ACC_ODR_1000Hz,
        SensorQMI8658::LPF_MODE_0,
        true,
        true
    );
    qmi.configGyroscope(
        SensorQMI8658::GYR_RANGE_512DPS,
        SensorQMI8658::GYR_ODR_7174_4Hz,
        SensorQMI8658::LPF_MODE_2,
        true,
        true
    );

    qmi.enableAccelerometer();
    qmi.enableGyroscope();

    printf("Accelerometer enabled\n");
}

void read_accelerometer(accelerometer_values_t *values) {
    if (qmi.getDataReady()) {
        qmi.getAccelerometer(values->delta_yaw, values->delta_pitch, values->delta_roll);
        qmi.getGyroscope(values->yaw, values->pitch, values->roll);
        values->temperature = qmi.getTemperature_C();
    }
}

#endif
