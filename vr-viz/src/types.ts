export type GyroMessage = {
    GyroscopeReading: {
        x: number;
        y: number;
        z: number;
        temperature: number;
    }
}

export type VrDistanceConfiguration = {
    VrDistanceConfiguration: {
        distance_between: number;
        v_offset: number;
    }
}

export type WebsocketMessage = GyroMessage | VrDistanceConfiguration;