import {
    DriverStateUpdate,
    FPSUpdate,
    GyroMessage, LeaderboardEntry,
    ModelConfiguration, PedalState, ServoConfiguration,
    VrDistanceConfiguration,
    WheelState
} from "./types.ts";
import {atom} from "nanostores";

export const $gyroReadings = atom<GyroMessage>({
    GyroscopeReading: {yaw: 0, pitch: 0, roll: 0, temperature: 0}
});

export const $vrDistanceConfigurationReadings = atom<VrDistanceConfiguration>({
    VrDistanceConfiguration: {v_offset: 0, distance_between_b: 0, distance_between_f: 0, distance_between_u: 0}
});

export const $inferenceReadings = atom<ModelConfiguration>({
    ModelConfiguration: {
        model: "YoloV8mInt8ONNX",
        config: {
            iou: 0.5,
            confidence: 0.5,
            kconf: 0.5
        }
    }
});

export const $wheelReadings = atom<WheelState>({
    WheelState: {left_button: false, right_button: false, rotation: 0, flipped: false}
});

export const $pedalReadings = atom<PedalState>({
    PedalState: {pressed: 0}
});

export const $drvStateReading = atom<DriverStateUpdate>({
    DriverStateUpdate: {states: [{Offline: {name: "Backend"}}]}
});

export const $fpsReading = atom<FPSUpdate>({
    FPSUpdate: {fps: 0}
});

export const $servoReading = atom<ServoConfiguration>({
    SetServoConfig: {
        config: {
            steer_offset: 0,
            yaw_offset: 0,
            pitch_offset: 0,
        }
    }
});

export const $leaderboard = atom<LeaderboardEntry[]>([]);