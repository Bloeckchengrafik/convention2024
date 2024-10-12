export type GyroMessage = {
    GyroscopeReading: {
        yaw: number;
        pitch: number;
        roll: number;
        temperature: number;
    }
}

export type VrDistanceConfiguration = {
    VrDistanceConfiguration: {
        distance_between_b: number;
        distance_between_f: number;
        distance_between_u: number;
        v_offset: number;
    }
}

export type LogMessage = {
    Log: {
        message: string;
        message_type: "Info" | "Error" | "Warning" | "Debug";
    }
}

export const models = [
    "YoloV8mInt8ONNX",
    "YoloV8mHalfONNX",
    "YoloV8mFullONNX",
    "YoloV11sInt8ONNX",
    "YoloV11sHalfONNX",
    "YoloV11sFullONNX",
    "YoloV11mInt8ONNX",
    "YoloV11mHalfONNX",
    "YoloV11mFullONNX",
] as const;

export type Model = typeof models[number];

export type ModelConfiguration = {
    ModelConfiguration: {
        model: Model;
        config: {
            confidence: number;
            iou: number;
            kconf: number;
        }
    }
}

export type ServoConfiguration = {
    SetServoConfig: {
        config: {
            steer_offset: number;
            yaw_offset: number;
            pitch_offset: number;
        }
    }
}

export type LeaderboardEntry = {
    name: string;
    time: number;
    id: number;
}

export type PushRenderSettings = {
    PushRenderSettings: {
        data: {
            left_eye: unknown;
            right_eye: unknown;
            v_offset: number;
            space_between_back: number;
            space_between_front: number;
            space_between_ui: number;
            model: Model;
            model_configuration: {
                confidence: number;
                iou: number;
                kconf: number;
            },
            servo_config: {
                steer_offset: number;
                yaw_offset: number;
                pitch_offset: number;
            },
            leaderboard: LeaderboardEntry[];
        }
    }
}

export type WheelState = {
    WheelState: {
        rotation: number;
        left_button: boolean;
        right_button: boolean;
        flipped: boolean;
    }
}


export type DriverState = {
    Online: {
        name: string;
    }
} | {
    Offline: {
        name: string;
    }
}

export type DriverStateUpdate = {
    DriverStateUpdate: {
        states: DriverState[];
    }
}

export type FPSUpdate = {
    FPSUpdate: {
        fps: number;
    }
}

export type ResetWheel = {
    ResetWheel: Record<string, never>
}

export type FlipWheelBtns = {
    FlipWheelBtns: {
        flip: boolean;
    }
}

export type PedalState = {
    PedalState: {
        pressed: number;
    }
}

export type ZeroPedal = {
    ZeroPedal: {
        position: "Lower" | "Upper";
    }
}

export type TimerStart = {
    TimerStart: {
        name: string;
    }
}

export type TimerEnd = {
    TimerEnd: Record<string, never>
}

export type PushTimerEntry = {
    PushTimerEntry: {
        entry: LeaderboardEntry;
    }
}

export type DeleteTimerEntry = {
    DeleteTimerEntry: {
        id: number;
    }
}

export type WebsocketMessage = GyroMessage
    | VrDistanceConfiguration
    | LogMessage
    | ModelConfiguration
    | PushRenderSettings
    | WheelState
    | DriverStateUpdate
    | FPSUpdate
    | ResetWheel
    | FlipWheelBtns
    | PedalState
    | ZeroPedal
    | ServoConfiguration
    | TimerStart
    | TimerEnd
    | PushTimerEntry
    | DeleteTimerEntry
    ;

export type FullWebsocketMessage = GyroMessage
    & VrDistanceConfiguration
    & LogMessage
    & ModelConfiguration
    & PushRenderSettings
    & WheelState
    & DriverStateUpdate
    & FPSUpdate
    & ResetWheel
    & FlipWheelBtns
    & PedalState
    & ZeroPedal
    & ServoConfiguration
    & TimerStart
    & TimerEnd
    & PushTimerEntry
    & DeleteTimerEntry
    ;
