import {VrDistanceConfiguration} from "../types.ts";
import {$vrDistanceConfigurationReadings} from "../state.ts";
import {useStore} from "@nanostores/react";

function VrDistanceConfigurationDisplay(props: { setter: (vdfc: VrDistanceConfiguration) => void }) {
    const {setter} = props;
    const vrDistanceConfigurationReading = useStore($vrDistanceConfigurationReadings);

    return (
        <div className="padding-around">
            <label>x-Distance-f</label><br/>
            <input type="range" value={vrDistanceConfigurationReading.VrDistanceConfiguration.distance_between_f}
                   onChange={(e) => {
                       setter({
                           VrDistanceConfiguration: {
                               distance_between_f: Number(e.target.value),
                               distance_between_b: vrDistanceConfigurationReading.VrDistanceConfiguration.distance_between_b,
                               v_offset: vrDistanceConfigurationReading.VrDistanceConfiguration.v_offset,
                           }
                       })
                   }}/> <br/> <br/>
            <label>x-Distance-b</label><br/>
            <input type="range" value={vrDistanceConfigurationReading.VrDistanceConfiguration.distance_between_b}
                   onChange={(e) => {
                       setter({
                           VrDistanceConfiguration: {
                               distance_between_f: vrDistanceConfigurationReading.VrDistanceConfiguration.distance_between_f,
                               distance_between_b: Number(e.target.value),
                               v_offset: vrDistanceConfigurationReading.VrDistanceConfiguration.v_offset,
                           }
                       })
                   }}/> <br/> <br/>
            <label>v-Offset</label><br/>
            <input type="range" value={vrDistanceConfigurationReading.VrDistanceConfiguration.v_offset}
                   onChange={(e) => {
                       setter({
                           VrDistanceConfiguration: {
                               distance_between_f: vrDistanceConfigurationReading.VrDistanceConfiguration.distance_between_f,
                               distance_between_b: vrDistanceConfigurationReading.VrDistanceConfiguration.distance_between_b,
                               v_offset: Number(e.target.value),
                           }
                       })
                   }}/>
        </div>
    )
}

export default VrDistanceConfigurationDisplay