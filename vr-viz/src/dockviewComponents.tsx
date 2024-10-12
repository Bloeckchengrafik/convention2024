import {DebouncedState} from "use-debounce";
import {GyroReadingDisplay} from "./views/GyroReadingDisplay.tsx";
import VrDistanceConfigurationDisplay from "./views/VrDistanceConfigurationDisplay.tsx";
import {$inferenceReadings, $servoReading, $vrDistanceConfigurationReadings} from "./state.ts";
import InferenceConfigurationDisplay from "./views/InferenceConfigurationDisplay.tsx";
import {SendJsonMessage} from "react-use-websocket/dist/lib/types";
import {WheelReadingDisplay} from "./views/WheelReadingDisplay.tsx";
import ServoConfigurationDisplay from "./views/ServoConfigurationDisplay.tsx";
import OptimizerInfoDisplay from "./views/OptimizerInfoDisplay.tsx";
import LeaderboardDisplay from "./views/LeaderboardDisplay.tsx";
import UtilitiesDisplay from "./views/Utilities.tsx";

export function DockviewComponents(useSetter: () => DebouncedState<SendJsonMessage>, setter: SendJsonMessage) {
    const vrSetter = useSetter();
    const infrSetter = useSetter();
    const servoSetter = useSetter();

    return {
        "gyro": () => <GyroReadingDisplay resetFn={() => {
            setter({SetGyroscopeZero: {}})
        }}/>,

        "vrdc": () => <VrDistanceConfigurationDisplay setter={(json) => {
            vrSetter(json);
            $vrDistanceConfigurationReadings.set(json);
        }}/>,

        "infr": () => <InferenceConfigurationDisplay setter={(json) => {
            infrSetter(json);
            $inferenceReadings.set(json);
        }}/>,

        "whl": () => <WheelReadingDisplay write={setter} />,
        "srvo": () => <ServoConfigurationDisplay setter={(v) => {
            servoSetter(v)
            $servoReading.set(v);
        }} />,
        "optm": () => <OptimizerInfoDisplay  setter={setter}/>,
        "ldbd": () => <LeaderboardDisplay setter={setter}/>,
        "util": () => <UtilitiesDisplay setter={setter}/>
    }
}