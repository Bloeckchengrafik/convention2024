use serde::{Deserialize, Serialize};

pub mod file_config;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EyeSettings {
    pub image_width: u32,
    pub image_height: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RenderSettingsData {
    pub left_eye: EyeSettings,
    pub right_eye: EyeSettings,
    pub v_offset: i32,
    pub space_between: i32,
    pub model: ModelType,
    pub model_configuration: ModelConfiguration,
}

impl Default for RenderSettingsData {
    fn default() -> Self {
        RenderSettingsData {
            left_eye: EyeSettings {
                image_width: 400,
                image_height: 480,
            },
            right_eye: EyeSettings {
                image_width: 400,
                image_height: 480,
            },
            v_offset: 0,
            space_between: 0,
            model: ModelType::YoloV8mInt8ONNX,
            model_configuration: ModelConfiguration {
                confidence: 0.25,
                iou: 0.7,
                kconf: 0.55,
            },
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Copy)]
pub enum ModelType {
    YoloV8mInt8ONNX,
    YoloV8mHalfONNX,
    YoloV8mFullONNX,
    YoloV11sInt8ONNX,
    YoloV11sHalfONNX,
    YoloV11sFullONNX,
    YoloV11mInt8ONNX,
    YoloV11mHalfONNX,
    YoloV11mFullONNX,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelConfiguration {
    pub confidence: f32,
    pub iou: f32,
    pub kconf: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize, Copy)]
pub enum LogMessageType {
    Info,
    Error,
    Warning,
    Debug,
}

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
    },
    ModelConfiguration {
        model: ModelType,
        config: ModelConfiguration,
    },
    Log {
        message: String,
        message_type: LogMessageType,
    },
    PushRenderSettings {
        data: RenderSettingsData,
    },
    WheelState {
        rotation: i128,
        left_button: bool,
        right_button: bool,
    },
    DriverStateUpdate {
        gyro_online: bool,
        swarm_online: bool,
        server_time: u64,
    },
    FPSUpdate {
        fps: f32,
    }
}