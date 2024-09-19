mod image_post_processing;
mod image_loader;
mod render_settings;
mod transform;
mod segmentation;
mod yolov8;

#[macro_use]
extern crate log;

use std::time::SystemTime;
use ggez::{Context, GameError, GameResult};
use ggez::conf::{FullscreenType, WindowMode, WindowSetup};
use ggez::context::HasMut;
use ggez::event::{EventHandler, EventLoop};
use ggez::graphics::{self, Image, Color, DrawParam, ImageFormat, Transform};
use ggez::mint::{Point2, Vector2};
use image::{EncodableLayout, GenericImageView};
use pub_sub::{PubSub, Subscription};
use screen_info::DisplayInfo;
use messages::VrMessage;
use crate::image_loader::{dynamic_to_ggez, left_image, right_image};
use crate::image_post_processing::postprocess;
use crate::render_settings::{config_has_changed, read_config, save_config, SettingsFrame};
use crate::transform::TransformSet;

pub struct ImagePair {
    pub left: Image,
    pub right: Image,
}

struct MainWindowState {
    lowest_level: ImagePair,
    settings: SettingsFrame,
    msgbus: PubSub<VrMessage>,
    subscription: Subscription<VrMessage>,
}

impl MainWindowState {
    pub fn new(ctx: &mut Context, pub_sub: PubSub<VrMessage>) -> Self {
        let config = read_config();

        let left = postprocess(left_image(), &config.data.left_eye);
        let right = postprocess(right_image(), &config.data.right_eye);

        let lowest_level = ImagePair {
            left: dynamic_to_ggez(ctx, left),
            right: dynamic_to_ggez(ctx, right),
        };

        MainWindowState {
            lowest_level,
            settings: config,
            subscription: pub_sub.subscribe(),
            msgbus: pub_sub,
        }
    }

    pub fn update_lowest_level(&mut self, ctx: &mut Context, pair: ImagePair) {
        self.lowest_level = pair;
    }

    fn reload_settings(&mut self, ctx: &mut Context) {
        if config_has_changed(&self.settings) {
            self.settings = read_config();

            let left = postprocess(left_image(), &self.settings.data.left_eye);
            let right = postprocess(right_image(), &self.settings.data.right_eye);

            self.lowest_level = ImagePair {
                left: dynamic_to_ggez(ctx, left),
                right: dynamic_to_ggez(ctx, right),
            };
        }
    }

    fn process_bus(&mut self) {
        loop {
            let message = self.subscription.try_recv();
            if message.is_err() {
                break;
            }

            let message = message.unwrap();

            match message {
                VrMessage::VrDistanceConfiguration { distance_between, v_offset } => {
                    self.settings.data.space_between = distance_between;
                    self.settings.data.v_offset = v_offset;
                    save_config(&self.settings);
                    self.settings.file_last_modified = SystemTime::now();
                }

                _ => {}
            }
        }
    }
}

impl EventHandler for MainWindowState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.process_bus();
        self.reload_settings(ctx);

        let transformations = TransformSet::from(&self.settings.data);

        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        canvas.draw(&self.lowest_level.left, DrawParam {
            transform: Transform::Values {
                dest: Point2::from(transformations.position_left),
                rotation: 0.0,
                scale: Vector2 { x: 1.0, y: 1.0 },
                offset: Point2 { x: 0.0, y: 0.0 },
            },
            ..Default::default()
        });
        canvas.draw(&self.lowest_level.right, DrawParam {
            transform: Transform::Values {
                dest: Point2::from(transformations.position_right),
                rotation: 0.0,
                scale: Vector2 { x: 1.0, y: 1.0 },
                offset: Point2 { x: 0.0, y: 0.0 },
            },
            ..Default::default()
        });
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
    return build_context(FullscreenType::Windowed);
}

#[cfg(feature = "fullscreen")]
fn build_context_according_to_config() -> GameResult<(Context, EventLoop<()>)> {
    return build_context(FullscreenType::True);
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
