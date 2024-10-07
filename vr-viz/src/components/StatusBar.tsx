import {GitBranch, History} from "lucide-react";
import {$drvStateReading, $fpsReading} from "../state.ts";
import {useStore} from '@nanostores/react'

export function StatusBar() {
    const drv = useStore($drvStateReading);
    const fps = useStore($fpsReading);
    return <div className="status-bar">
        <div className="status">
            <span className="pill info">{fps.FPSUpdate.fps.toFixed(2)} FPS</span>
            {drv
                .DriverStateUpdate
                .states
                .map((state) => {
                    const key: "Online" | "Offline" = Object.keys(state)[0] as keyof typeof state;
                    if (key === "Online") {
                        return {
                            online: true,
                            // eslint-disable-next-line @typescript-eslint/ban-ts-comment
                            // @ts-expect-error
                            name: state[key].name
                        }
                    } else {
                        return {
                            online: false,
                            // eslint-disable-next-line @typescript-eslint/ban-ts-comment
                            // @ts-expect-error
                            name: state[key].name
                        }
                    }
                })
                .map((state) => (
                    <span className={"pill " + (state.online ? "up" : "down")}
                          key={state.name}>{state.name}</span>
                ))}
        </div>

        <div className="time">
            <History/> <span>v1.0.0 -</span> <GitBranch/> <span>main</span>
        </div>
    </div>
}