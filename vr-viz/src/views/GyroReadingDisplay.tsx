import {$gyroReadings} from "../state.ts";
import {Canvas} from "@react-three/fiber";
import {Euler, Vector3} from "three";
import {OrbitControls} from "@react-three/drei";
import {Model as Headset} from "../3d/Headset.tsx";
import {useStore} from "@nanostores/react";

interface AxisProps {
    direction: [number, number, number];
    length: number;
    color: number;
}

const Axis: React.FC<AxisProps> = ({direction, length, color}) => {
    const dir = new Vector3(...direction).normalize();
    const origin = new Vector3(0, 0, 0);

    return <arrowHelper args={[dir, origin, length, color]}/>;
};

const CoordinateSystem = () => {
    return (
        <>
            <Axis direction={[1, 0, 0]} length={2} color={0xff0000}/>
            <Axis direction={[0, 1, 0]} length={2} color={0x00ff00}/>
            <Axis direction={[0, 0, 1]} length={2} color={0x0000ff}/>
        </>
    );
};

function format(num: number) {
    if (num == null) return "null";
    return num.toFixed(2);
}

export function GyroReadingDisplay(params: {resetFn: () => void}) {
    const reset = params.resetFn;
    const gyroReading = useStore($gyroReadings);

    const rotation = new Euler(-gyroReading.GyroscopeReading.pitch-0.1, -gyroReading.GyroscopeReading.yaw, gyroReading.GyroscopeReading.roll+0.13, "YXZ");
    return (
        <>
            <div>
                <div className="measurement-list">
                    <div className="measurement-item">
                        <span>yaw</span>
                        <span className="value">{format(gyroReading.GyroscopeReading.yaw)}</span>
                    </div>
                    <div className="measurement-item">
                        <span>pitch</span>
                        <span className="value">{format(gyroReading.GyroscopeReading.pitch)}</span>
                    </div>
                    <div className="measurement-item">
                        <span>tilt</span>
                        <span className="value">{format(gyroReading.GyroscopeReading.roll)}</span>
                    </div>

                    <button onClick={reset}>reset</button>
                </div>


            </div>
            <Canvas>
            <ambientLight intensity={Math.PI / 2}/>
                <spotLight position={[10, 10, 10]} angle={0.15} penumbra={1} decay={0} intensity={Math.PI}/>
                <pointLight position={[-10, -10, -10]} decay={0} intensity={Math.PI}/>
                <Headset rotation={rotation}/>
                <CoordinateSystem/>
                <OrbitControls/>
            </Canvas>
        </>
    )
}