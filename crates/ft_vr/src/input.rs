use pub_sub::PubSub;
use input_devices::InputDevices;
use messages::VrMessage;

fn send_accelerometer(input_devices: &InputDevices, bus: &PubSub<VrMessage>) {
    let data = &input_devices.headset_gyroscope.last_data;
    let message = VrMessage::GyroscopeReading {
        x: data.yaw,
        y: data.pitch,
        z: data.roll,
        temperature: data.temperature,
    };

    if let Err(e) = bus.send(message) {
        error!("Error publishing gyroscope reading: {:?}", e)
    }
}

pub fn send_inputs(input_devices: &InputDevices, bus: &PubSub<VrMessage>) {
    send_accelerometer(input_devices, bus);
}