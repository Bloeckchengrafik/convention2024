use std::thread::sleep;
use std::time::Duration;
use log::LevelFilter;
use vr_renderer::vr_render_main;

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
        .init();
}

fn input_device_loop() {
    info!("Starting Input Device Loop");
    let mut input_devices = input_devices::InputDevices::new();
    let mut i = 0;
    loop {
        if let Err(e) = input_devices.process() {
            error!("Error processing input devices: {:?}", e);
            sleep(Duration::from_millis(100));
            continue;
        }
        i += 1;
        if i % 100 == 0 {
            info!("headset data: {:?}", input_devices.headset_gyroscope.last_data);
            i = 0;
        }

        sleep(Duration::from_millis(20));
    }
}

fn main() {
    init_logging();

    std::thread::spawn(|| input_device_loop());

    info!("Starting VR Renderer");
    vr_render_main();
}
