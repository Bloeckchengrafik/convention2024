import {VrDistanceConfiguration} from "../types.ts";
import {useContext} from "react";
import {VrDistanceConfigurationReadings} from "../state.ts";

function VrDistanceConfigurationDisplay(props: {setter: (vdfc: VrDistanceConfiguration) => void}) {
    const {setter} = props;
    const vrDistanceConfigurationReading = useContext(VrDistanceConfigurationReadings);

    return (
        <div className="padding-around">
            <label>x-Distance</label><br/>
            <input type="range" value={vrDistanceConfigurationReading.VrDistanceConfiguration.distance_between}
                   onChange={(e) => {
                       setter({
                           VrDistanceConfiguration: {
                               distance_between: Number(e.target.value),
                               v_offset: vrDistanceConfigurationReading.VrDistanceConfiguration.v_offset,
                           }
                       })
                   }}/> <br/> <br />
            <label>v-Offset</label><br/>
            <input type="range" value={vrDistanceConfigurationReading.VrDistanceConfiguration.v_offset}
                   onChange={(e) => {
                       setter({
                           VrDistanceConfiguration: {
                               distance_between: vrDistanceConfigurationReading.VrDistanceConfiguration.distance_between,
                               v_offset: Number(e.target.value),
                           }
                       })
                   }}/>
        </div>
    )
}

export default VrDistanceConfigurationDisplay