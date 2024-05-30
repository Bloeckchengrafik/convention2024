import {createContext} from "react";
import {GyroMessage, VrDistanceConfiguration} from "./types.ts";

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
