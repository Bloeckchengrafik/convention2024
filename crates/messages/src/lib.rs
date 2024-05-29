use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum VrMessage {
    GyroscopeReading {
        x: f32,
        y: f32,
        z: f32,
        temperature: f32,
    },
    VrDistanceConfiguration {
        distance_between: i32,
        v_offset: i32,
    }
}