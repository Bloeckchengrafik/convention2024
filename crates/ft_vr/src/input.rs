use pub_sub::PubSub;
use input_devices::InputDevices;
use messages::VrMessage;

fn send_accelerometer(input_devices: &InputDevices, bus: &PubSub<VrMessage>) {
    if let Some(accelerometer) = &input_devices.headset_gyroscope {
        let data = &accelerometer.last_data;
        let message = VrMessage::GyroscopeReading {
            yaw: data.yaw,
            pitch: data.pitch,
            roll: data.roll,
            temperature: 0f32,
        };

        if let Err(e) = bus.send(message) {
            error!("Error publishing gyroscope reading: {:?}", e)
        }
    }
}

pub fn send_inputs(input_devices: &InputDevices, bus: &PubSub<VrMessage>) {
    send_accelerometer(input_devices, bus);
}