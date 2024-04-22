use log::LevelFilter;
use vr_renderer::vr_render_main;

#[macro_use]
extern crate log;

fn init_logging() {
    pretty_env_logger::formatted_timed_builder()
        .parse_default_env()
        .filter_level(LevelFilter::Trace)
        .init();
}

fn main() {
    init_logging();

    info!("Starting VR Renderer");
    vr_render_main();
}
