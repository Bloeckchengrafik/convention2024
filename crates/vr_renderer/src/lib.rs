mod image_post_processing;
mod image_loader;
mod transform;
mod segmentation;
mod models;
mod imgstream;

use std::fmt::{Debug, Formatter};
use std::time::{Instant};
use ggez::{Context, GameResult};
use ggez::conf::{FullscreenType, WindowMode, WindowSetup};
use ggez::event::{EventHandler, EventLoop};
use ggez::graphics::{self, Color, DrawParam, Image, Text, TextFragment, Transform};
use ggez::mint::{Point2, Vector2};
use image::{DynamicImage, GenericImage, GenericImageView, GrayImage};
use imageproc::distance_transform::Norm;
use itertools::izip;
use log::error;
use pub_sub::{PubSub, Subscription};
use tracing::{debug_span, instrument, span, trace, trace_span, Level};
use messages::{Interface, LogMessageType, RenderSettingsData, VrMessage};
use crate::image_loader::{dynamic_to_ggez, ImageLoader};
use crate::image_post_processing::postprocess;
use messages::file_config::{read_config, save_config};
use crate::transform::TransformSet;

pub struct Images {
    pub left_back: Image,
    pub left_front: Image,
    pub right_back: Image,
    pub right_front: Image,
}


struct MainWindowState {
    loader: ImageLoader,
    lowest_level: Option<Images>,
    settings: RenderSettingsData,
    msgbus: PubSub<VrMessage>,
    subscription: Subscription<VrMessage>,
    tick: u64,
    fps_buffer: [u128; 20],
    last_frame: Instant,
    fps_buf_idx: usize,
    // interface: Option<Interface>,
    // wheel: VrMessage::WheelState,
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
            fps_buffer: [0; 20],
            last_frame: Instant::now(),
            fps_buf_idx: 0,
            // interface: None,
            // wheel: VrMessage::WheelState {
            //     rotation: 0,
            //     left_button: false,
            //     right_button: false,
            //     flipped: false,
            // },
        }
    }

    #[instrument]
    fn process_bus(&mut self) {
        while let Ok(message) = self.subscription.try_recv() {
            match message {
                VrMessage::VrDistanceConfiguration { distance_between_b, distance_between_f, v_offset } => {
                    self.settings.space_between_back = distance_between_b;
                    self.settings.space_between_front = distance_between_f;
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

    fn finish_frame(&mut self) {
        let now = Instant::now();
        let duration = now - self.last_frame;
        self.last_frame = now;

        self.fps_buffer[self.fps_buf_idx] = duration.as_millis();
        self.fps_buf_idx += 1;
        if self.fps_buf_idx >= 20 {
            self.fps_buf_idx = 0;
            let sum: u128 = self.fps_buffer.iter().sum();
            let avg = sum / 20;
            let fps = 1000.0 / avg as f32;
            let _ = self.msgbus.send(VrMessage::FPSUpdate { fps });
        }

        tracy_client::frame_mark();
    }
}

impl EventHandler for MainWindowState {
    #[instrument]
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.tick += 1;
        self.tick %= 3;
        self.process_bus();

        let ((lf, lb), (rf, rb)) = debug_span!("loader.images()")
            .in_scope(|| self.loader.images(self.tick == 0));

        let ((lb, lf, rb, rf)) = debug_span!("postprocess")
            .in_scope(|| (
                postprocess(lb, &self.settings.left_eye, false),
                postprocess(lf, &self.settings.left_eye, true),
                postprocess(rb, &self.settings.right_eye, false),
                postprocess(rf, &self.settings.right_eye, true)
            ));

        debug_span!("update.lowest_level")
            .in_scope(|| {
                self.lowest_level = Some(Images {
                    left_back: dynamic_to_ggez(ctx, lb),
                    left_front: dynamic_to_ggez(ctx, lf),
                    right_back: dynamic_to_ggez(ctx, rb),
                    right_front: dynamic_to_ggez(ctx, rf),
                });
            });


        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let transformations = TransformSet::from(&self.settings);

        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        if let Some(lowest_level) = &self.lowest_level {
            canvas.draw(&lowest_level.left_back, DrawParam {
                transform: Transform::Values {
                    dest: Point2::from(transformations.position_left_back),
                    rotation: 0.0,
                    scale: Vector2 { x: 1.0, y: 1.0 },
                    offset: Point2 { x: 0.0, y: 0.0 },
                },
                ..Default::default()
            });
            canvas.draw(&lowest_level.left_front, DrawParam {
                transform: Transform::Values {
                    dest: Point2::from(transformations.position_left_front),
                    rotation: 0.0,
                    scale: Vector2 { x: 1.0, y: 1.0 },
                    offset: Point2 { x: 0.0, y: 0.0 },
                },
                ..Default::default()
            });
            canvas.draw(&lowest_level.right_back, DrawParam {
                transform: Transform::Values {
                    dest: Point2::from(transformations.position_right_back),
                    rotation: 0.0,
                    scale: Vector2 { x: 1.0, y: 1.0 },
                    offset: Point2 { x: 0.0, y: 0.0 },
                },
                ..Default::default()
            });
            canvas.draw(&lowest_level.right_front, DrawParam {
                transform: Transform::Values {
                    dest: Point2::from(transformations.position_right_front),
                    rotation: 0.0,
                    scale: Vector2 { x: 1.0, y: 1.0 },
                    offset: Point2 { x: 0.0, y: 0.0 },
                },
                ..Default::default()
            });
        }

        self.finish_frame();
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
