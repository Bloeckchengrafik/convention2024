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

export type WebsocketMessage = GyroMessage | VrDistanceConfiguration;