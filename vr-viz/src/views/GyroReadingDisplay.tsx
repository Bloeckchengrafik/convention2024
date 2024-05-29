import React, {useContext} from "react";
import {GyroReadings} from "../state.ts";
import {Canvas} from "@react-three/fiber";
import {Vector3} from "three";
import {OrbitControls} from "@react-three/drei";
import {Model as Headset} from "../3d/Headset.tsx";

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

export function GyroReadingDisplay() {
    const gyroReading = useContext(GyroReadings);
    return (
        <>
            <div>
                X: {format(gyroReading.GyroscopeReading.x)} Y: {format(gyroReading.GyroscopeReading.y)} Z: {format(gyroReading.GyroscopeReading.z)} T: {format(gyroReading.GyroscopeReading.temperature)}
            </div>
            <Canvas>
                <ambientLight intensity={Math.PI / 2}/>
                <spotLight position={[10, 10, 10]} angle={0.15} penumbra={1} decay={0} intensity={Math.PI}/>
                <pointLight position={[-10, -10, -10]} decay={0} intensity={Math.PI}/>
                <Headset rotation={[gyroReading.GyroscopeReading.x, gyroReading.GyroscopeReading.y, gyroReading.GyroscopeReading.z]}/>
                <CoordinateSystem/>
                <OrbitControls/>
            </Canvas>
        </>
    )
}