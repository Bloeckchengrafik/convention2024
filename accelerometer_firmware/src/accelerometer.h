//
// Created by chris on 5/29/24.
//

#ifndef ACCELEROMETER_H
#define ACCELEROMETER_H

typedef struct {
    float yaw;
    float pitch;
    float roll;

    float delta_yaw;
    float delta_pitch;
    float delta_roll;

    float temperature;
} accelerometer_values_t;

void init_accelerometer();

void read_accelerometer(accelerometer_values_t *values);

#endif //ACCELEROMETER_H
