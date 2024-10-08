import {ModelConfiguration} from "../types.ts";
import {$inferenceReadings} from "../state.ts";
import {useStore} from "@nanostores/react";

function InferenceConfigurationDisplay(props: { setter: (_: ModelConfiguration) => void }) {
    const {setter} = props;
    const reading = useStore($inferenceReadings);

    function patchConf(partial: Partial<ModelConfiguration["ModelConfiguration"]["config"]>) {
        setter({
            ModelConfiguration: {
                ...reading.ModelConfiguration,
                config: {
                    ...reading.ModelConfiguration.config,
                    ...partial
                }
            }
        });
    }

    return (
        <div className="padding-around">
            <label>Intersection Over Union</label><br/>
            <input type="range" value={reading.ModelConfiguration.config.iou} min={0} max={1} step={0.01}
                   title={String(reading.ModelConfiguration.config.iou)}
                   onChange={(e) => {
                       patchConf({
                           iou: Number(e.target.value),
                       })
                   }}/><br/> <br/>
            <label>Confidence</label><br/>
            <input type="range" value={reading.ModelConfiguration.config.confidence} min={0} max={1} step={0.01}
                   title={String(reading.ModelConfiguration.config.confidence)}
                   onChange={(e) => {
                       patchConf({
                           confidence: Number(e.target.value),
                       })
                   }}/><br/> <br/>
            <label>Keypoint Confidence</label><br/>
            <input type="range" value={reading.ModelConfiguration.config.kconf} min={0} max={1} step={0.01}
                   title={String(reading.ModelConfiguration.config.kconf)}
                   onChange={(e) => {
                       patchConf({
                           kconf: Number(e.target.value),
                       })
                   }}/>
        </div>
    )
}

export default InferenceConfigurationDisplay