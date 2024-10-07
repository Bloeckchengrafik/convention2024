import './App.css'
import useWebSocket from "react-use-websocket";
import {useEffect, useState} from "react";
import {FullWebsocketMessage, WebsocketMessage} from "./types.ts";
import 'dockview/dist/styles/dockview.css';
import {DockviewApi, DockviewReact, DockviewReadyEvent} from "dockview";
import Cmdk from "./components/Cmdk.tsx";
import {restoreDefaultLayout} from "./dockviewapi.ts";
import {useDebouncedCallback} from "use-debounce";
import {toast, Toaster} from "sonner";
import {StatusBar} from "./components/StatusBar.tsx";
import {Watermark} from "./components/Watermark.tsx";
import {terror} from "./toasties.ts";
import {MessageObjectForKey, processors} from "./processing.ts";
import {DockviewComponents} from "./dockviewComponents.tsx";


function App() {
    const currentHostName = window.location.hostname;

    const [version, setVersion] = useState<string>("");

    const {
        lastMessage,
        sendJsonMessage,
        readyState
    } = useWebSocket<WebsocketMessage>("ws://" + currentHostName + ":6342/webclient?v=" + version);
    const [dockview, setDockview] = useState<DockviewApi>();

    function useDebounceSetter() {
        return useDebouncedCallback(sendJsonMessage, 100);
    }

    useEffect(() => {
        if (lastMessage == null) {
            return;
        }

        const websocketMessage = JSON.parse(lastMessage.data) as unknown as WebsocketMessage;
        const keys = Object.keys(websocketMessage) as (keyof FullWebsocketMessage)[];

        if (keys.length !== 1) {
            terror(`Unexpected message: ${websocketMessage}`);
            return;
        }

        const key = keys[0];
        if (!(key in processors)) {
            return;
        }

        // eslint-disable-next-line @typescript-eslint/ban-ts-comment
        // @ts-expect-error
        processors[key](websocketMessage as MessageObjectForKey<typeof key>);

    }, [lastMessage]);

    useEffect(() => {
        if (readyState === 1) {
            toast.dismiss("websocket-disconnected");
        } else {
            toast.error("Disconnected from server", {
                id: "websocket-disconnected",
                duration: 100000000,
                richColors: true,
                action: {
                    key: "reconnect",
                    label: "Reconnect",
                    onClick: () => {
                        setVersion((prev) => prev + "1");
                    }
                }
            })
        }
    }, [readyState]);


    function onDockviewReady(event: DockviewReadyEvent) {
        const api: DockviewApi = event.api;
        setDockview(api);

        restoreDefaultLayout(api)
    }

    return (
        <>
            <Toaster
                theme="dark"
            />
            <DockviewReact
                onReady={onDockviewReady}
                components={DockviewComponents(useDebounceSetter, sendJsonMessage)}
                watermarkComponent={() => <Watermark/>}
                className="dockview-cstm"
            />

            <StatusBar/>

            <Cmdk dockview={dockview}/>
        </>
    )
}

export default App
