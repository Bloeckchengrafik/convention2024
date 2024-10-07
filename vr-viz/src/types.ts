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

export type PushRenderSettings = {
    PushRenderSettings: {
        data: {
            left_eye: unknown;
            right_eye: unknown;
            v_offset: number;
            space_between_back: number;
            space_between_front: number;
            model: Model;
            model_configuration: {
                confidence: number;
                iou: number;
                kconf: number;
            }
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
;
