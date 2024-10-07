import {Speedometer, SteeringWheel} from "@phosphor-icons/react";
import {useStore} from "@nanostores/react";
import {$pedalReadings, $wheelReadings} from "../state.ts";
import {ReactNode} from "react";
import {Button, ButtonList} from "../components/ActionButton.tsx";
import {SendJsonMessage} from "react-use-websocket/dist/lib/types";
import {WebsocketMessage} from "../types.ts";

function Toggle({on}: {on: boolean}) {
    return (
        <div className="flex">
            <div className={`w-8 h-8 ${!on ? "bg-red-400" : "bg-stone-700"}`}></div>
            <div className={`w-8 h-8 ${!on ? "bg-stone-700" : "bg-green-400"}`}></div>
        </div>
    )
}

function WheelDisplay() {
    const wheel = useStore($wheelReadings);
    // one revolution = 200 steps
    const degreesFromSteps = (steps: number) => steps / 200 * 360;

    return <div className="flex gap-10 justify-center items-center">
        <Toggle on={wheel.WheelState.left_button} />
        <div>
            <SteeringWheel className="w-32 h-32 text-stone-600" style={{
                transform: `rotate(${degreesFromSteps(wheel.WheelState.rotation)}deg)`,
                transition: "transform 0.1s"
            }} />
        </div>
        <Toggle on={wheel.WheelState.right_button}/>
    </div>
}

function IconAndSlider({icon, value, min, max}: {icon: ReactNode, value: number, min: number, max: number}) {
    const valuePercentage = (value - min) / (max - min) * 100;
    const clamp = (value: number, min: number, max: number) => Math.min(Math.max(value, min), max);
    const clampedValuePercentage = clamp(valuePercentage, 0, 100);

    return (
        <div className="flex gap-4 items-center measurement-item">
            <div className="text-2xl">
                {icon}
            </div>
            <div className="w-60 h-2 bg-stone-700 rounded-full overflow-hidden">
                <div className="h-2 bg-blue-400" style={{width: `${clampedValuePercentage}%`}} />
            </div>
            <span className="w-[6ch] text-right value">{value}</span>
        </div>
    )
}

function InformationDisplay() {
    const wheel = useStore($wheelReadings);
    const pedal = useStore($pedalReadings);
    return (
        <div className="flex flex-col gap-2">
            <IconAndSlider icon={<SteeringWheel />} value={wheel.WheelState.rotation} min={-360} max={360} />
            <IconAndSlider icon={<Speedometer />} value={pedal.PedalState.pressed} min={0} max={255} />
        </div>
    )
}

function ActionButtons({write}: {write: (jsonMessage: WebsocketMessage) => void}) {
    const wheel = useStore($wheelReadings);
    return (
        <ButtonList>
            <Button onClick={write.bind(null, {ResetWheel: {}})}>Whl Rst</Button>
            <Button onClick={write.bind(null, {FlipWheelBtns: {flip: !wheel.WheelState.flipped}})}>Btn Flp</Button>
            <div className="bg-stone-700 w-px h-3/4" />
            <Button onClick={write.bind(null, {ZeroPedal: { position: "Lower" }})}>Lwr cal</Button>
            <Button onClick={write.bind(null, {ZeroPedal: { position: "Upper" }})}>Upr cal</Button>
        </ButtonList>
    )
}

export function WheelReadingDisplay({write}: {write: SendJsonMessage}) {
    return (
        <div className="w-full h-full flex flex-col items-center justify-center gap-10">
            <WheelDisplay/>
            <InformationDisplay />
            <ActionButtons write={write} />
        </div>
    )
}