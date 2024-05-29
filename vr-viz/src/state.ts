import {createContext} from "react";
import {GyroMessage, VrDistanceConfiguration} from "./types.ts";

export const GyroReadings = createContext<GyroMessage>({
    GyroscopeReading: {
        x: 0,
        y: 0,
        z: 0,
        temperature: 0,
    }
});

export const VrDistanceConfigurationReadings = createContext<VrDistanceConfiguration>({
    VrDistanceConfiguration: {
        v_offset: 0,
        distance_between: 0,
    }
});
