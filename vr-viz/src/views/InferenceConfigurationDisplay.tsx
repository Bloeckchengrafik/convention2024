import {ModelConfiguration, models} from "../types.ts";
import {useContext} from "react";
import {InferenceReadings} from "../state.ts";

function InferenceConfigurationDisplay(props: { setter: (_: ModelConfiguration) => void }) {
    const {setter} = props;
    const reading = useContext(InferenceReadings);

    function patch(partial: Partial<ModelConfiguration["ModelConfiguration"]>) {
        setter({
            ModelConfiguration: {
                ...reading.ModelConfiguration,
                ...partial
            }
        });
    }

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
        <div className="padding-around lr-container">
            <div className="left">
                <label>Model</label><br/>
                <select value={reading.ModelConfiguration.model}
                        onChange={(e) => {
                            patch({
                                model: e.target.value as never
                            })
                        }}>
                    {models.map((model) => (
                        <option key={model} value={model}>{model}</option>
                    ))}
                </select><br/> <br/>
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
            {reading.ModelConfiguration.model.includes("ONNX") && (
                <div className="right">
                    <h3>Optimizer State: YOLO-ONNX</h3>
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
            )}
        </div>
    )
}

export default InferenceConfigurationDisplay