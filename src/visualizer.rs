use ggez::{graphics::Mesh, Context};
use noise::NoiseFn;

use crate::consts::{DEFAULT_NOISE_SCALE, DEFAULT_NOISE_SPEED};

pub struct VisualizerParams {
    pub base_x_offset: f64,
    pub base_y_offset: f64,
    pub noise_scale: f64,
    pub noise_speed: f64,
    pub noise_fn: Box<dyn NoiseFn<[f64; 3]>>,
    pub z_offset: f64,
}

impl Default for VisualizerParams {
    fn default() -> Self {
        let noise_fn = Box::new(noise::Billow::new());

        Self {
            base_x_offset: 0.0,
            base_y_offset: 0.0,
            noise_scale: DEFAULT_NOISE_SCALE,
            noise_speed: DEFAULT_NOISE_SPEED,
            noise_fn,
            z_offset: 0.0,
        }
    }
}

pub trait Visualizer {
    fn update(&mut self, params: &mut VisualizerParams);
    fn build_mesh(&self, ctx: &mut Context) -> Option<Mesh>;
    fn build_svg_document_from_state(&self) -> svg::Document;
}
