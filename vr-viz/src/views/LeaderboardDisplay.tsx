import {$leaderboard} from "../state.ts";
import {useStore} from "@nanostores/react";
import {SendJsonMessage} from "react-use-websocket/dist/lib/types";
import {useState} from "react";

function LeaderboardDisplay({setter}: { setter: SendJsonMessage }) {
    const reading = useStore($leaderboard);
    const [running, setRunning] = useState(false);

    function deleteLeaderboard(id: number) {
        setter({DeleteTimerEntry: {id}});
        $leaderboard.set(reading.filter((entry) => entry.id !== id));
    }

    return (
        <div className="padding-around flex flex-col gap-2">
            <div className="flex gap-2">
                <input type={"text"} placeholder={"Name"} id={"name"} disabled={running}/>
                <button onClick={() => {
                    const name = (document.getElementById("name") as HTMLInputElement).value;
                    setter({TimerStart: {name}});
                    setRunning(true);
                }} disabled={running}>Start
                </button>
                <button onClick={() => {
                    setter({TimerEnd: {}});
                    setRunning(false);
                }} disabled={!running}>Stop
                </button>
            </div>

            {reading
                .sort((a, b) => a.time - b.time)
                .map((entry) => (
                <div key={entry.id} className="flex gap-2 items-center">
                    <button onClick={() => deleteLeaderboard(entry.id)}>&times;</button>
                    <div className="w-[10ch] text-right">{entry.time.toFixed(2)}s</div>
                    <div>{entry.name}</div>
                </div>
            ))}
        </div>
    )
}

export default LeaderboardDisplay;