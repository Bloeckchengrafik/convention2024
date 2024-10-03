import {createContext} from "react";
import {GyroMessage, ModelConfiguration, VrDistanceConfiguration} from "./types.ts";

export const GyroReadings = createContext<GyroMessage>({
    GyroscopeReading: {
        yaw: 0,
        pitch: 0,
        roll: 0,
        temperature: 0,
    }
});

export const VrDistanceConfigurationReadings = createContext<VrDistanceConfiguration>({
    VrDistanceConfiguration: {
        v_offset: 0,
        distance_between: 0,
    }
});

export const InferenceReadings = createContext<ModelConfiguration>({
    ModelConfiguration: {
        model: "YoloV8mInt8ONNX",
        config: {
            confidence: 0.5,
            iou: 0.5,
            kconf: 0.5,
        }
    }
});
