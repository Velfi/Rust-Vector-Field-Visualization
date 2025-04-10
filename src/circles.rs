use crate::{
    consts::{
        GRID_CELL_H, GRID_CELL_W, GRID_SIZE_X, GRID_SIZE_Y, SCREEN_H, SCREEN_W, VECTOR_SCALE,
    },
    visualizer::{Visualizer, VisualizerParams},
};
use log::info;
use macroquad::prelude::*;
use svg::node::element;

type Point2<T> = [T; 2];

#[derive(Clone)]
pub struct Circle {
    pub location: Point2<f64>,
    pub radius: f64,
    pub scale: f64,
}

impl Circle {
    pub fn new(location: Point2<f64>, radius: f64, scale: f64) -> Self {
        Self {
            location,
            radius,
            scale,
        }
    }

    pub fn set_radius(&mut self, radius: f64) {
        self.radius = radius * self.scale;
    }
}

impl Default for Circle {
    fn default() -> Self {
        Circle::new(Default::default(), 1.0, 1.0)
    }
}

pub struct Circles {
    circles: Vec<Circle>,
}

impl Circles {
    pub fn new(params: &VisualizerParams) -> Self {
        let mut circles = Vec::with_capacity(GRID_SIZE_X * GRID_SIZE_Y);
        // halving it cause it a radius and not a diameter
        let scale = VECTOR_SCALE / 0.5;

        for y in 0..GRID_SIZE_Y {
            for x in 0..GRID_SIZE_X {
                let (x, y) = (x as f64, y as f64);

                let location = [
                    x * GRID_CELL_W + GRID_CELL_W / 2.0,
                    y * GRID_CELL_H + GRID_CELL_H / 2.0,
                ];

                let radius = params.noise_fn.get([x, y, params.z_offset]) * scale;

                circles.push(Circle::new(location, radius, scale));
            }
        }

        Self { circles }
    }
}

impl Visualizer for Circles {
    fn update(&mut self, params: &mut VisualizerParams) {
        let mut y_offset = 0.0 + params.base_y_offset;
        for y in 0..GRID_SIZE_Y {
            let mut x_offset = 0.0 + params.base_x_offset;
            for x in 0..GRID_SIZE_X {
                let radius = params
                    .noise_fn
                    .get([x_offset, y_offset, params.z_offset])
                    .abs();
                // TODO is just setting radius interesting enough?
                self.circles[x + y * GRID_SIZE_X].set_radius(radius);

                x_offset += params.noise_scale;
            }
            y_offset += params.noise_scale;
            // TODO is this really meant to happen per y?
            params.z_offset += params.noise_speed;
        }
    }

    fn build_svg_document_from_state(&self) -> svg::Document {
        let doc = svg::Document::new().set("viewBox", (0, 0, SCREEN_W, SCREEN_H));

        let mut group = element::Group::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", "0.3mm");

        info!("rendering {} circles", self.circles.len());

        for circle in self.circles.iter() {
            let [cx, cy] = circle.location;
            let path = element::Circle::new()
                .set("cx", cx)
                .set("cy", cy)
                .set("r", circle.radius);

            group = group.add(path);
        }

        let bounding_rect = element::Rectangle::new()
            .set("width", SCREEN_W)
            .set("height", SCREEN_H)
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", "1mm");

        doc.add(group).add(bounding_rect)
    }

    fn render(&self) {
        for circle in self.circles.iter() {
            let [x, y] = circle.location;

            draw_circle(x as f32, y as f32, circle.radius as f32, WHITE);
        }
    }
}
