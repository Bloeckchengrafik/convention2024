use std::time::{Duration, Instant};
use log::LevelFilter;
use pub_sub::PubSub;
use tokio::time::sleep;
use tracing_subscriber::layer::SubscriberExt;
use tracing_tracy::client::ProfiledAllocator;
use game_core::game_main;
use input_devices::InputDevices;
use messages::{LogMessageType, VrMessage};
use vr_renderer::vr_render_main;
use websocket_server::websocket_server;

// #[global_allocator]
// static GLOBAL: ProfiledAllocator<std::alloc::System> = ProfiledAllocator::new(std::alloc::System, 100);

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
        // .filter_module("ftswarm_serial", LevelFilter::Warn)
        .filter_module("winit", LevelFilter::Warn)
        .filter_module("gilrs", LevelFilter::Warn)
        .filter_module("mio", LevelFilter::Warn)
        .filter_module("ggez", LevelFilter::Warn)
        .filter_module("tungstenite", LevelFilter::Warn)
        .init();
}

async fn input_device_loop(bus: PubSub<VrMessage>) {
    let mut input_devices = InputDevices::new(&bus).await;
    let states = input_devices.driver_states();
    let mut last_update = Instant::now();
    let mut device_drivers = input_devices.build().await;
    loop {
        let mut errors = vec![];
        for driver in &mut device_drivers {
            if let Err(e) = driver.process().await {
                errors.push(e);
            }
        }

        for e in errors {
            let err = format!("Error processing input devices: {:?}", e);
            error!("{}", err);
            let _ = bus.send(VrMessage::Log { message: err, message_type: LogMessageType::Error });
        }

        if last_update.elapsed().as_secs() > 1 {
            last_update = Instant::now();
            let _ = bus.send(VrMessage::DriverStateUpdate {
                states: states.clone()
            });
        }

        sleep(Duration::from_millis(40)).await;
    }
}

fn init_tracing() {
    tracing::subscriber::set_global_default(
        tracing_subscriber::registry()
            .with(tracing_tracy::TracyLayer::default())
            .with(tracing_subscriber::filter::LevelFilter::DEBUG)
    ).expect("setup tracy layer");
}

macro_rules! spawn_future_in_thread {
    ($f:expr) => {
        std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap();
            rt.block_on($f);
        });
    };
}

fn main() {
    init_logging();
    init_tracing();

    let bus = PubSub::<VrMessage>::new();

    let bus_game = bus.clone();
    let bus_input = bus.clone();
    let bus_ws = bus.clone();

    spawn_future_in_thread!(input_device_loop(bus_input));
    spawn_future_in_thread!(game_main(bus_game));
    spawn_future_in_thread!(async {websocket_server(bus_ws)});

    info!("Starting VR Renderer");
    vr_render_main(bus.clone());
}
