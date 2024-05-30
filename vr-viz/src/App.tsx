import './App.css'
import useWebSocket from "react-use-websocket";
import {useEffect, useState} from "react";
import {GyroMessage, VrDistanceConfiguration, WebsocketMessage} from "./types.ts";
import 'dockview/dist/styles/dockview.css';
import {DockviewApi, DockviewReact, DockviewReadyEvent} from "dockview";
import {GyroReadingDisplay} from "./views/GyroReadingDisplay.tsx";
import Cmdk from "./components/Cmdk.tsx";
import {GyroReadings, VrDistanceConfigurationReadings} from "./state.ts";
import VrDistanceConfigurationDisplay from "./views/VrDistanceConfigurationDisplay.tsx";
import {openGyroTab, openVrDistanceConfigurationTab, restoreDefaultLayout} from "./dockviewapi.ts";
import {useDebouncedCallback} from "use-debounce";


function App() {
    const {lastMessage, sendJsonMessage} = useWebSocket<WebsocketMessage>("ws://localhost:6342/webclient");
    const [dockview, setDockview] = useState<DockviewApi>();
    const [gyroUpdate, setGyroUpdate] = useState<number>(0);

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

    const vrSetter = useDebouncedCallback(sendJsonMessage, 100);

    useEffect(() => {
        if (lastMessage !== null) {
            const websocketMessage = JSON.parse(lastMessage.data) as unknown as WebsocketMessage;
            const keys = Object.keys(websocketMessage);

            if (keys.includes("GyroscopeReading")) {
                if (gyroUpdate > 10) {
                    setGyroReading(websocketMessage as GyroMessage);
                    setGyroUpdate(0);
                } else {
                    setGyroUpdate(gyroUpdate + 1);
                }
            } else if (keys.includes("VrDistanceConfiguration")) {
                setVrDistanceConfigurationReading(websocketMessage as VrDistanceConfiguration);
            }
        }
    }, [lastMessage]);


    function onDockviewReady(event: DockviewReadyEvent) {
        const api: DockviewApi = event.api;
        setDockview(api);

        openGyroTab(api)
        openVrDistanceConfigurationTab(api)

        restoreDefaultLayout(api)
    }

    return (
        <>
            <GyroReadings.Provider value={gyroReading}>
                <VrDistanceConfigurationReadings.Provider value={vrDistanceConfigurationReading}>
                    <DockviewReact onReady={onDockviewReady} components={{
                        "gyro": () => <GyroReadingDisplay resetFn={() => {
                            sendJsonMessage({SetGyroscopeZero:{}})
                        }}/>,
                        "vrdc": () => <VrDistanceConfigurationDisplay setter={(json) => {
                            vrSetter(json);
                            setVrDistanceConfigurationReading(json as VrDistanceConfiguration);
                        }}/>
                    }}/>
                </VrDistanceConfigurationReadings.Provider>
            </GyroReadings.Provider>

            <Cmdk dockview={dockview}/>
        </>
    )
}

export default App
