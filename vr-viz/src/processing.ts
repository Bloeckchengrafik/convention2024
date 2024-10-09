import {LogMessage, WebsocketMessage} from "./types.ts";
import {
    $drvStateReading,
    $fpsReading,
    $gyroReadings,
    $inferenceReadings, $pedalReadings, $servoReading,
    $vrDistanceConfigurationReadings, $wheelReadings
} from "./state.ts";
import {terror, tinfo, tmessage, twarning} from "./toasties.ts";
import {toast} from "sonner";

type ExtractTopLevelKey<T> = T extends { [K in keyof T]: unknown } ? keyof T : never;
type WebsocketMessageKeys = ExtractTopLevelKey<WebsocketMessage>;
export type MessageObjectForKey<K extends WebsocketMessageKeys> = Extract<WebsocketMessage, Record<K, unknown>>;


export type Processors = {
    [K in WebsocketMessageKeys]: (data: MessageObjectForKey<K>) => void;
};

export const processors: Partial<Processors> = {
    GyroscopeReading: $gyroReadings.set,
    VrDistanceConfiguration: $vrDistanceConfigurationReadings.set,
    DriverStateUpdate: $drvStateReading.set,
    FPSUpdate: $fpsReading.set,
    WheelState: $wheelReadings.set,
    ModelConfiguration: $inferenceReadings.set,
    PedalState: $pedalReadings.set,

    Log(log) {
        const functions: Record<LogMessage["Log"]["message_type"], (typeof tinfo)> = {
            Debug: tmessage, Error: terror, Info: tinfo, Warning: twarning
        }
        functions[log.Log.message_type](log.Log.message)
    },
    PushRenderSettings(msg) {
        $vrDistanceConfigurationReadings.set({
            VrDistanceConfiguration: {
                distance_between_b: msg.PushRenderSettings.data.space_between_back,
                distance_between_f: msg.PushRenderSettings.data.space_between_front,
                v_offset: msg.PushRenderSettings.data.v_offset,
            }
        })
        $inferenceReadings.set({
            ModelConfiguration: {
                model: msg.PushRenderSettings.data.model,
                config: {
                    confidence: msg.PushRenderSettings.data.model_configuration.confidence,
                    iou: msg.PushRenderSettings.data.model_configuration.iou,
                    kconf: msg.PushRenderSettings.data.model_configuration.kconf,
                }
            }
        })
        $servoReading.set({
            SetServoConfig: {
                config: msg.PushRenderSettings.data.servo_config
            }
        })
        toast.success("Loaded config", {
            dismissible: true,
            richColors: true,
            duration: 2000
        })
    },
}