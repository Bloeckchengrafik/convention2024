use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum VrMessage {
    GyroscopeReading {
        yaw: f32,
        pitch: f32,
        roll: f32,
        temperature: f32,
    },
    SetGyroscopeZero {},
    VrDistanceConfiguration {
        distance_between: i32,
        v_offset: i32,
    }
}