import {ModelConfiguration, models} from "../types.ts";
import {useStore} from "@nanostores/react";
import {$inferenceReadings} from "../state.ts";

export default function OptimizerInfoDisplay({setter}: { setter: (_: ModelConfiguration) => void }) {
    const reading = useStore($inferenceReadings);

    function patch(partial: Partial<ModelConfiguration["ModelConfiguration"]>) {
        setter({
            ModelConfiguration: {
                ...reading.ModelConfiguration,
                ...partial
            }
        });
    }

    return (
        <div className="padding-around">
            Model<br/><select value={reading.ModelConfiguration.model}
                                         onChange={(e) => {
                                             patch({
                                                 model: e.target.value as never
                                             })
                                         }}>
                {models.map((model) => (
                    <option key={model} value={model}>{model}</option>
                ))}
            </select> <br /> <br />
            <div style={{
                display: "flex",
                flexDirection: "column",
                gap: "1em",
            }}>
                <div className="measurement-item">
                    <span>PRE</span>
                    <span className="value">batch-multi-pre-v7</span>
                </div>
                <div className="measurement-item">
                    <span>INF</span>
                    <span className="value">def-inf-v1</span>
                </div>
                <div className="measurement-item">
                    <span>POST</span>
                    <span className="value">fast-post-v2</span>
                </div>
            </div>
        </div>
    )
}
