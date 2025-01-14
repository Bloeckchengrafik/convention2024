mod image_post_processing;
mod image_loader;
mod transform;
mod segmentation;
mod models;
mod imgstream;

use std::fmt::{format, Debug, Formatter};
use std::time::Instant;
use ggez::{Context, GameResult};
use ggez::conf::{FullscreenType, WindowMode, WindowSetup};
use ggez::event::{EventHandler, EventLoop};
use ggez::glam::Vec2;
use ggez::graphics::{self, Color, DrawParam, Image, Transform};
use ggez::mint::{Point2, Vector2};
use log::error;
use pub_sub::{PubSub, Subscription};
use tracing::{debug_span, instrument};
use messages::{Interface, LogMessageType, RenderSettingsData, VrMessage};
use crate::image_loader::{dynamic_to_ggez, ImageLoader};
use crate::image_post_processing::postprocess;
use messages::file_config::{read_config, save_config};
use crate::transform::{left_offset_left, right_offset_right, TransformSet};

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
    interface: Option<Interface>,
    whl_rot: i128,
    whl_btn: bool,
    last_whl_btn: bool,
}


impl Debug for MainWindowState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MainWindowState")
            .field("tick", &self.tick)
            .finish()
    }
}

impl MainWindowState {
    pub fn new(ctx: &mut Context, pub_sub: PubSub<VrMessage>) -> Self {
        let config = read_config();
        ctx.gfx.add_font(
            "Arial",
            graphics::FontData::from_slice(include_bytes!("../../../Arial.ttf")).unwrap(),
        );

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
            interface: None,
            whl_rot: 0,
            whl_btn: false,
            last_whl_btn: false,
        }
    }

    #[instrument]
    fn process_bus(&mut self) {
        while let Ok(message) = self.subscription.try_recv() {
            match message {
                VrMessage::VrDistanceConfiguration { distance_between_b, distance_between_f, v_offset, distance_between_u } => {
                    self.settings.space_between_back = distance_between_b;
                    self.settings.space_between_front = distance_between_f;
                    self.settings.space_between_ui = distance_between_u;
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

                VrMessage::WheelState { rotation, left_button, right_button, .. } => {
                    self.whl_rot = rotation;
                    self.whl_btn = left_button || right_button;
                }

                VrMessage::ShowRenderedInterface { interface } => {
                    self.interface = Some(interface)
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

    fn draw_input_number(&mut self, ctx: &mut Context, canvas: &mut graphics::Canvas, text: String, pos: Vec2) -> GameResult {
        let chosen_number = ((self.whl_rot / 20) % 10).abs();
        canvas.draw(
            graphics::Text::new(format!("{}{}\n<-> Rad | [x] Taster", text, chosen_number))
                .set_font("Arial")
                .set_scale(12.)
            , pos);

        Ok(())
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

        let (lb, lf, rb, rf) = debug_span!("postprocess")
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

        if self.whl_btn && !self.last_whl_btn {
            let number = ((self.whl_rot / 20) % 10).abs();
            if let Some(interface) = &self.interface {
                match interface {
                    Interface::InputNumberAndConfirm { .. } => {
                        let _ = self.msgbus.send(VrMessage::InterfaceConfirm {
                            data: number as i32,
                        });

                        self.interface = None;
                    }
                }
            }
        };

        self.last_whl_btn = self.whl_btn;

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

        if let Some(interface) = &self.interface {
            match interface {
                Interface::InputNumberAndConfirm { text } => {
                    let left = left_offset_left(&(self.settings.space_between_ui as f32));
                    let right = right_offset_right(&(self.settings.space_between_ui as f32));
                    let text = text.clone();

                    self.draw_input_number(ctx, &mut canvas, text.clone(), left)?;
                    self.draw_input_number(ctx, &mut canvas, text.clone(), right)?;
                }
            }
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
