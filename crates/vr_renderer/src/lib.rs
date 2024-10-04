mod image_post_processing;
mod image_loader;
mod transform;
mod segmentation;
mod models;

use std::fmt::{Debug, Formatter};
use std::time::{Instant};
use ggez::{Context, GameResult};
use ggez::conf::{FullscreenType, WindowMode, WindowSetup};
use ggez::event::{EventHandler, EventLoop};
use ggez::graphics::{self, Color, DrawParam, Image, Text, TextFragment, Transform};
use ggez::mint::{Point2, Vector2};
use log::error;
use pub_sub::{PubSub, Subscription};
use tracing::{instrument, span, trace, trace_span, Level};
use messages::{LogMessageType, RenderSettingsData, VrMessage};
use crate::image_loader::{dynamic_to_ggez, ImageLoader};
use crate::image_post_processing::postprocess;
use messages::file_config::{read_config, save_config};
use crate::transform::TransformSet;

pub struct ImagePair {
    pub left: Image,
    pub right: Image,
}


struct MainWindowState {
    loader: ImageLoader,
    lowest_level: Option<ImagePair>,
    settings: RenderSettingsData,
    msgbus: PubSub<VrMessage>,
    subscription: Subscription<VrMessage>,
    tick: u64,
}


impl Debug for MainWindowState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MainWindowState")
            .field("tick", &self.tick)
            .finish()
    }
}

impl MainWindowState {
    pub fn new(_: &mut Context, pub_sub: PubSub<VrMessage>) -> Self {
        let config = read_config();

        MainWindowState {
            loader: ImageLoader::new(),
            lowest_level: None,
            settings: config,
            subscription: pub_sub.subscribe(),
            msgbus: pub_sub,
            tick: 0,
        }
    }

    #[instrument]
    fn process_bus(&mut self) {
        loop {
            let message = self.subscription.try_recv();
            if message.is_err() {
                break;
            }

            let message = message.unwrap();

            match message {
                VrMessage::VrDistanceConfiguration { distance_between, v_offset } => {
                    self.settings.space_between = distance_between;
                    self.settings.v_offset = v_offset;
                    save_config(&self.settings);
                }

                VrMessage::ModelConfiguration { model, config } => {
                    self.settings.model = model;
                    self.settings.model_configuration = config;
                    save_config(&self.settings);

                    self.loader.reload(&self.settings);
                    let _ = self.msgbus.send(VrMessage::Log {
                        message: format!("Model changed to {:?}", model),
                        message_type: LogMessageType::Info,
                    });
                }

                _ => {}
            }
        }
    }
}

impl EventHandler for MainWindowState {
    #[instrument]
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.tick += 1;
        self.tick %= 3;
        self.process_bus();
        if self.tick == 0 {
            let (left, right) = trace_span!("loader.images()")
                .in_scope(|| self.loader.images());

            let (left, right) = trace_span!("postprocess")
                .in_scope(|| (
                    postprocess(left, &self.settings.left_eye),
                    postprocess(right, &self.settings.right_eye)
                ));

            trace_span!("update.lowest_level")
                .in_scope(|| {
                    self.lowest_level = Some(ImagePair {
                        left: dynamic_to_ggez(ctx, left),
                        right: dynamic_to_ggez(ctx, right),
                    });
                });
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let transformations = TransformSet::from(&self.settings);

        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        if let Some(lowest_level) = &self.lowest_level {
            canvas.draw(&lowest_level.left, DrawParam {
                transform: Transform::Values {
                    dest: Point2::from(transformations.position_left),
                    rotation: 0.0,
                    scale: Vector2 { x: 1.0, y: 1.0 },
                    offset: Point2 { x: 0.0, y: 0.0 },
                },
                ..Default::default()
            });
            canvas.draw(&lowest_level.right, DrawParam {
                transform: Transform::Values {
                    dest: Point2::from(transformations.position_right),
                    rotation: 0.0,
                    scale: Vector2 { x: 1.0, y: 1.0 },
                    offset: Point2 { x: 0.0, y: 0.0 },
                },
                ..Default::default()
            });
        }

        tracy_client::frame_mark();
        canvas.finish(ctx)
    }
}

fn build_context(fullscreen_type: FullscreenType) -> GameResult<(Context, EventLoop<()>)> {
    let ctx = ggez::ContextBuilder::new("vr_renderer", "bloeckchen")
        .window_mode(
            WindowMode::default()
                .dimensions(800.0, 480.0)
                .borderless(true)
                .fullscreen_type(fullscreen_type)
                .resizable(false)
        )
        .window_setup(
            WindowSetup::default()
                .title("VR Preview")
        )
        .build()?;

    Ok(ctx)
}

#[cfg(not(feature = "fullscreen"))]
fn build_context_according_to_config() -> GameResult<(Context, EventLoop<()>)> {
    build_context(FullscreenType::Windowed)
}

#[cfg(feature = "fullscreen")]
fn build_context_according_to_config() -> GameResult<(Context, EventLoop<()>)> {
    build_context(FullscreenType::True)
}

pub fn vr_render_main(pub_sub: PubSub<VrMessage>) {
    let result = build_context_according_to_config();

    let (mut ctx, event_loop) = match result {
        Ok((ctx, event_loop)) => (ctx, event_loop),
        Err(_) => {
            error!("Failed to build context");
            return;
        }
    };

    let state = MainWindowState::new(&mut ctx, pub_sub);

    ggez::event::run(ctx, event_loop, state);
}
