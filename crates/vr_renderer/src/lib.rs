mod image_post_processing;
mod image_loader;
mod render_settings;
mod transform;

#[macro_use]
extern crate log;

use ggez::{Context, GameError, GameResult};
use ggez::conf::{FullscreenType, WindowMode, WindowSetup};
use ggez::event::EventHandler;
use ggez::graphics::{self, Image, Color, DrawParam, ImageFormat, Transform};
use ggez::mint::{Point2, Vector2};
use image::{EncodableLayout, GenericImageView};
use crate::image_loader::{dynamic_to_ggez, left_example, right_example};
use crate::image_post_processing::postprocess;
use crate::render_settings::{config_has_changed, read_config, SettingsFrame};
use crate::transform::TransformSet;

pub struct ImagePair {
    pub left: Image,
    pub right: Image,
}

struct MainWindowState {
    lowest_level: ImagePair,
    settings: SettingsFrame
}

impl MainWindowState {
    pub fn new(ctx: &mut Context) -> Self {
        let config = read_config();

        let left = postprocess(left_example(), &config.data.left_eye);
        let right = postprocess(right_example(), &config.data.right_eye);

        let lowest_level = ImagePair {
            left: dynamic_to_ggez(ctx, left),
            right: dynamic_to_ggez(ctx, right),
        };

        MainWindowState {
            lowest_level,
            settings: config
        }
    }

    pub fn update_lowest_level(&mut self, ctx: &mut Context, pair: ImagePair) {
        self.lowest_level = pair;
    }

    fn reload_settings(&mut self, ctx: &mut Context) {
        if config_has_changed(&self.settings) {
            self.settings = read_config();

            let left = postprocess(left_example(), &self.settings.data.left_eye);
            let right = postprocess(right_example(), &self.settings.data.right_eye);

            self.lowest_level = ImagePair {
                left: dynamic_to_ggez(ctx, left),
                right: dynamic_to_ggez(ctx, right),
            };
        }
    }
}

impl EventHandler for MainWindowState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
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

pub fn vr_render_main() {
    let (mut ctx, event_loop) = ggez::ContextBuilder::new("vr_renderer", "chris b")
        .window_mode(
            WindowMode::default()
                .dimensions(800.0, 480.0)
                .resizable(false)
        )
        .window_setup(
            WindowSetup::default()
                .title("VR Preview")
        )
        .build()
        .unwrap();

    let state = MainWindowState::new(&mut ctx);

    ggez::event::run(ctx, event_loop, state);
}
