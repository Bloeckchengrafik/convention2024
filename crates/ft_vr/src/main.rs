mod input;

use std::thread::sleep;
use std::time::Duration;
use log::LevelFilter;
use pub_sub::PubSub;
use input_devices::InputDevices;
use messages::VrMessage;
use vr_renderer::vr_render_main;
use websocket_server::websocket_server;
use crate::input::send_inputs;

#[macro_use]
extern crate log;

fn init_logging() {
    pretty_env_logger::formatted_timed_builder()
        .parse_default_env()
        .filter_level(LevelFilter::Trace)
        .filter_module("calloop", LevelFilter::Info)
        .filter_module("naga", LevelFilter::Info)
        .filter_module("wgpu", LevelFilter::Warn)
        .filter_module("wgpu_hal", LevelFilter::Warn)
        .filter_module("wgpu_core", LevelFilter::Warn)
        .filter_module("ftswarm_serial", LevelFilter::Warn)
        .filter_module("winit", LevelFilter::Warn)
        .filter_module("gilrs", LevelFilter::Warn)
        .filter_module("mio", LevelFilter::Warn)
        .filter_module("ggez", LevelFilter::Warn)
        .filter_module("tungstenite", LevelFilter::Warn)
        .init();
}

fn input_device_loop(bus: PubSub<VrMessage>) {
    info!("Starting Input Device Loop");
    let mut input_devices = InputDevices::new();
    loop {
        if let Err(e) = input_devices.process() {
            error!("Error processing input devices: {:?}", e);
            sleep(Duration::from_millis(100));
            continue;
        }

        send_inputs(&input_devices, &bus);

        sleep(Duration::from_millis(40));
    }
}

fn main() {
    init_logging();

    let bus = PubSub::<VrMessage>::new();

    let bus_input = bus.clone();
    let bus_ws = bus.clone();

    std::thread::spawn(move || input_device_loop(bus_input));
    std::thread::spawn(move || websocket_server(bus_ws));

    info!("Starting VR Renderer");
    vr_render_main(bus.clone());
}
