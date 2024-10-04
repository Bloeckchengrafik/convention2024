import './App.css'
import useWebSocket from "react-use-websocket";
import {useEffect, useState} from "react";
import {
    DriverStateUpdate,
    FullWebsocketMessage,
    GyroMessage,
    LogMessage, ModelConfiguration,
    PushRenderSettings,
    VrDistanceConfiguration,
    WebsocketMessage
} from "./types.ts";
import 'dockview/dist/styles/dockview.css';
import {DockviewApi, DockviewReact, DockviewReadyEvent} from "dockview";
import {GyroReadingDisplay} from "./views/GyroReadingDisplay.tsx";
import Cmdk from "./components/Cmdk.tsx";
import {GyroReadings, InferenceReadings, VrDistanceConfigurationReadings} from "./state.ts";
import VrDistanceConfigurationDisplay from "./views/VrDistanceConfigurationDisplay.tsx";
import {openGyroTab, openVrDistanceConfigurationTab, restoreDefaultLayout} from "./dockviewapi.ts";
import {useDebouncedCallback} from "use-debounce";
import {toast, Toaster} from "sonner";
import InferenceConfigurationDisplay from "./views/InferenceConfigurationDisplay.tsx";
import {StatusBar} from "./components/StatusBar.tsx";
import {Watermark} from "./components/Watermark.tsx";


function App() {
    const currentHostName = window.location.hostname;
    const {
        lastMessage,
        sendJsonMessage,
        readyState
    } = useWebSocket<WebsocketMessage>("ws://" + currentHostName + ":6342/webclient");
    const [dockview, setDockview] = useState<DockviewApi>();

    const [gyroReading, setGyroReading] = useState<GyroMessage>({
        GyroscopeReading: {
            yaw: 0,
            pitch: 0,
            roll: 0,
            temperature: 0
        }
    });

    const [vrDistanceConfigurationReading, setVrDistanceConfigurationReading] = useState<VrDistanceConfiguration>({
        VrDistanceConfiguration: {
            v_offset: 0,
            distance_between: 0,
        }
    });


    const [inferenceConfigReading, setInferenceConfigReading] = useState<ModelConfiguration>({
        ModelConfiguration: {
            model: "YoloV8mInt8ONNX",
            config: {
                iou: 0.5,
                confidence: 0.5,
                kconf: 0.5
            }
        }
    });

    const [drvStateReading, setDrvStateReading] = useState<DriverStateUpdate>({
        DriverStateUpdate: {gyro_online: false, server_time: 0, swarm_online: false}
    });

    const [fpsReading, setFpsReading] = useState<{FPSUpdate: {fps: number}}>({FPSUpdate: {fps: 0}});

    const vrSetter = useDebouncedCallback(sendJsonMessage, 100);
    const infrSetter = useDebouncedCallback(sendJsonMessage, 100);

    useEffect(() => {
        if (lastMessage !== null) {
            const websocketMessage = JSON.parse(lastMessage.data) as unknown as WebsocketMessage;
            const keys = Object.keys(websocketMessage) as (keyof FullWebsocketMessage)[];

            if (keys.length !== 1) {
                console.error("Unexpected message", websocketMessage);
                return;
            }

            const key = keys[0];

            if (key == "GyroscopeReading") {
                setGyroReading(websocketMessage as GyroMessage);
            } else if (key == "VrDistanceConfiguration") {
                setVrDistanceConfigurationReading(websocketMessage as VrDistanceConfiguration);
            } else if (key == "Log") {
                const log = websocketMessage as LogMessage;
                const functions: Record<LogMessage["Log"]["message_type"], (typeof toast["success"])> = {
                    Debug: toast.message,
                    Error: toast.error,
                    Info: toast.info,
                    Warning: toast.warning,
                }

                functions[log.Log.message_type](log.Log.message, {
                    richColors: true,
                    dismissible: true,
                })
            } else if (key == "PushRenderSettings") {
                const msg = websocketMessage as PushRenderSettings;
                setVrDistanceConfigurationReading({
                    VrDistanceConfiguration: {
                        distance_between: msg.PushRenderSettings.data.space_between,
                        v_offset: msg.PushRenderSettings.data.v_offset,
                    }
                })
                setInferenceConfigReading({
                    ModelConfiguration: {
                        model: msg.PushRenderSettings.data.model,
                        config: {
                            confidence: msg.PushRenderSettings.data.model_configuration.confidence,
                            iou: msg.PushRenderSettings.data.model_configuration.iou,
                            kconf: msg.PushRenderSettings.data.model_configuration.kconf,
                        }
                    }
                })
                toast.success("Loaded config", {
                    dismissible: true,
                    richColors: true,
                    duration: 2000
                })
            } else if (key == "DriverStateUpdate") {
                setDrvStateReading(websocketMessage as DriverStateUpdate);
            } else if (key == "FPSUpdate") {
                setFpsReading(websocketMessage as {FPSUpdate: {fps: number}});
            }
        }
    }, [lastMessage]);

    useEffect(() => {
        if (readyState === 1) {
            toast.dismiss("websocket-disconnected");
        } else {
            toast.error("Disconnected from server", {
                id: "websocket-disconnected",
                duration: 100000000,
                richColors: true
            })
        }
    }, [readyState]);


    function onDockviewReady(event: DockviewReadyEvent) {
        const api: DockviewApi = event.api;
        setDockview(api);

        openGyroTab(api)
        openVrDistanceConfigurationTab(api)

        restoreDefaultLayout(api)
    }

    return (
        <>
            <Toaster
                theme="dark"
            />
            <GyroReadings.Provider value={gyroReading}>
                <VrDistanceConfigurationReadings.Provider value={vrDistanceConfigurationReading}>
                    <InferenceReadings.Provider value={inferenceConfigReading}>
                        <DockviewReact
                            onReady={onDockviewReady}
                            components={{
                                "gyro": () => <GyroReadingDisplay resetFn={() => {
                                    sendJsonMessage({SetGyroscopeZero: {}})
                                }}/>,
                                "vrdc": () => <VrDistanceConfigurationDisplay setter={(json) => {
                                    vrSetter(json);
                                    setVrDistanceConfigurationReading(json);
                                }}/>,
                                "infr": () => <InferenceConfigurationDisplay setter={(json) => {
                                    infrSetter(json);
                                    setInferenceConfigReading(json);
                                }}/>
                            }}
                            watermarkComponent={() => <Watermark/>}
                            className="dockview-cstm"
                        />
                    </InferenceReadings.Provider>
                </VrDistanceConfigurationReadings.Provider>
            </GyroReadings.Provider>

            <StatusBar
                ftSwarm={drvStateReading.DriverStateUpdate.swarm_online}
                gyro={drvStateReading.DriverStateUpdate.gyro_online}
                fps={fpsReading.FPSUpdate.fps}
            />

            <Cmdk dockview={dockview}/>
        </>
    )
}

export default App
