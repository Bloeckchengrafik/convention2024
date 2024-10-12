import {ServoConfiguration} from "../types.ts";
import {$servoReading} from "../state.ts";
import {useStore} from "@nanostores/react";

function ServoConfigurationDisplay(props: { setter: (_: ServoConfiguration) => void }) {
    const {setter} = props;
    const reading = useStore($servoReading);

    function patch(partial: Partial<ServoConfiguration["SetServoConfig"]["config"]>) {
        setter({
            SetServoConfig: {
                config: {
                    ...reading.SetServoConfig.config,
                    ...partial
                }
            }
        });
    }

    return (
        <div className="padding-around">
            <label>Steer Offset</label><br/>
            <input type="range" value={reading.SetServoConfig.config.steer_offset} min={0} max={255} step={1}
                   title={String(reading.SetServoConfig.config.steer_offset)}
                   onChange={(e) => {
                       patch({
                           steer_offset: Number(e.target.value),
                       })
                   }}/><br/> <br/>

            <label>Yaw Offset</label><br/>
            <input type="range" value={reading.SetServoConfig.config.yaw_offset} min={0} max={255} step={1}
                   title={String(reading.SetServoConfig.config.yaw_offset)}
                   onChange={(e) => {
                       patch({
                           yaw_offset: Number(e.target.value),
                       })
                   }}/><br/> <br/>

            <label>Pitch Offset</label><br/>
            <input type="range" value={reading.SetServoConfig.config.pitch_offset} min={0} max={255} step={1}
                   title={String(reading.SetServoConfig.config.pitch_offset)}
                   onChange={(e) => {
                       patch({
                           pitch_offset: Number(e.target.value),
                       })
                   }}/><br/> <br/>
        </div>
    )
}

export default ServoConfigurationDisplay;