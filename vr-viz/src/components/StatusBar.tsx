import {GitBranch, History} from "lucide-react";

export function StatusBar(props: { ftSwarm: boolean, gyro: boolean, fps: number }) {
    return <div className="status-bar">
        <div className="status">
            <span className="pill info">{props.fps.toFixed(2)} FPS</span>
            <span className={"pill " + (props.gyro ? "up" : "down")}>Gyroscope</span>
            <span className={"pill " + (props.ftSwarm ? "up" : "down")}>ftSwarm</span>
        </div>

        <div className="time">
            <History /> <span>v1.0.0 -</span> <GitBranch /> <span>main</span>
        </div>
    </div>
}