use serde::{Deserialize, Serialize};

pub mod file_config;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServoConfig {
    pub steer_offset: i32,
    pub yaw_offset: i32,
    pub pitch_offset: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EyeSettings {
    pub image_width: u32,
    pub image_height: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LeaderboardEntry {
    pub name: String,
    pub time: f32,
    pub id: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RenderSettingsData {
    pub left_eye: EyeSettings,
    pub right_eye: EyeSettings,
    pub v_offset: i32,
    pub space_between_back: i32,
    pub space_between_front: i32,
    #[serde(default)]
    pub space_between_ui: i32,
    pub model: ModelType,
    pub model_configuration: ModelConfiguration,
    pub servo_config: ServoConfig,
    #[serde(default)]
    pub pedal_calibration_lower: i32,
    #[serde(default)]
    pub pedal_calibration_upper: i32,
    #[serde(default)]
    pub speed_mul: f32,
    #[serde(default)]
    pub leaderboard: Vec<LeaderboardEntry>,
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
            space_between_back: 0,
            space_between_front: 0,
            space_between_ui: 0,
            model: ModelType::YoloV8mInt8ONNX,
            model_configuration: ModelConfiguration {
                confidence: 0.25,
                iou: 0.7,
                kconf: 0.55,
            },
            servo_config: ServoConfig {
                steer_offset: 0,
                yaw_offset: 0,
                pitch_offset: 0,
            },
            pedal_calibration_lower: 0,
            pedal_calibration_upper: 0,
            speed_mul: 1.0,
            leaderboard: Vec::new(),
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

#[derive(Clone, Debug, Serialize, Deserialize, Copy)]
pub enum PedalPosition {
    Lower,
    Upper,
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
pub enum DriverState {
    Online {
        name: String
    },
    Offline {
        name: String
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Interface {
    InputNumberAndConfirm {
        text: String,
    },
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
        distance_between_b: i32,
        distance_between_f: i32,
        distance_between_u: i32,
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
        flipped: bool,
    },
    DriverStateUpdate {
        states: Vec<DriverState>
    },
    FPSUpdate {
        fps: f32,
    },
    ResetWheel {},
    FlipWheelBtns {
        flip: bool,
    },
    PedalState {
        pressed: u8
    },
    ZeroPedal {
        position: PedalPosition
    },
    ShowRenderedInterface {
        interface: Interface
    },
    InterfaceConfirm {
        data: i32
    },
    AskPin {
        length: u8,
    },
    ConfirmPin {
        pin: String,
    },
    SetServoConfig {
        config: ServoConfig,
    },
    TimerStart {
        name: String,
    },
    TimerEnd {},
    PushTimerEntry {
        entry: LeaderboardEntry
    },
    DeleteTimerEntry {
        id: u32
    },
}