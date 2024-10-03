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
        distance_between: number;
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
            space_between: number;
            model: Model;
            model_configuration: {
                confidence: number;
                iou: number;
                kconf: number;
            }
        }
    }
}

export type WebsocketMessage = GyroMessage
    | VrDistanceConfiguration
    | LogMessage
    | ModelConfiguration
    | PushRenderSettings;

export type FullWebsocketMessage = GyroMessage
    & VrDistanceConfiguration
    & LogMessage
    & ModelConfiguration
    & PushRenderSettings;
