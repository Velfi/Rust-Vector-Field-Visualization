use crate::{
    consts::{
        GRID_CELL_H, GRID_CELL_W, GRID_SIZE_X, GRID_SIZE_Y, SCREEN_H, SCREEN_W, VECTOR_SCALE,
        VECTOR_WIDTH,
    },
    visualizer::{Visualizer, VisualizerParams},
};
use log::info;
use std::f64::consts::TAU;
use svg::node::element;
use macroquad::prelude::*;

type Point2<T> = [T; 2];

#[derive(Clone)]
pub struct LineSegment {
    pub points: [Point2<f64>; 2],
    pub scale: f64,
}

impl LineSegment {
    pub fn new(p0: Point2<f64>, p1: Point2<f64>, scale: f64) -> Self {
        LineSegment {
            points: [p0, p1],
            scale,
        }
    }

    pub fn from_angle(p0: Point2<f64>, angle: f64, scale: f64) -> Self {
        let p1 = [scale * angle.cos() + p0[0], scale * angle.sin() + p0[1]];
        LineSegment::new(p0, p1, scale)
    }

    pub fn set_p1_relative(&mut self, x: f64, y: f64) {
        self.points[1][0] = x * self.scale + self.points[0][0];
        self.points[1][1] = y * self.scale + self.points[0][1];
    }
}

impl Default for LineSegment {
    fn default() -> Self {
        LineSegment::new([0.0, 0.0], [1.0, 1.0], 1.0)
    }
}

pub struct LineSegments {
    line_segments: Vec<LineSegment>,
}

impl LineSegments {
    pub fn new(params: &VisualizerParams) -> LineSegments {
        let mut line_segments = Vec::with_capacity(GRID_SIZE_X * GRID_SIZE_Y);

        for y in 0..GRID_SIZE_Y {
            for x in 0..GRID_SIZE_X {
                let (x, y) = (x as f64, y as f64);

                let p0 = [
                    x * GRID_CELL_W + GRID_CELL_W / 2.0,
                    y * GRID_CELL_H + GRID_CELL_H / 2.0,
                ];

                let angle = params.noise_fn.get([x, y, params.z_offset]) * TAU;

                line_segments.push(LineSegment::from_angle(p0, angle, VECTOR_SCALE));
            }
        }

        Self { line_segments }
    }
}

impl Visualizer for LineSegments {
    fn update(&mut self, params: &mut VisualizerParams) {
        let mut y_offset = 0.0 + params.base_y_offset;
        for y in 0..GRID_SIZE_Y {
            let mut x_offset = 0.0 + params.base_x_offset;
            for x in 0..GRID_SIZE_X {
                let angle = params.noise_fn.get([x_offset, y_offset, params.z_offset]) * TAU;
                let next_line_to_draw = &mut self.line_segments[x + y * GRID_SIZE_X];

                next_line_to_draw.scale = VECTOR_SCALE * angle.atan();
                next_line_to_draw.set_p1_relative(angle.cos(), angle.sin());

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

        info!("rendering {} lines", self.line_segments.len());

        for line in self.line_segments.iter() {
            let [[x1, y1], [x2, y2]] = line.points;
            let path = element::Line::new()
                .set("x1", x1)
                .set("y1", y1)
                .set("x2", x2)
                .set("y2", y2);

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
        for line_segment in self.line_segments.iter() {
            let [[x1, y1], [x2, y2]] = line_segment.points;
            draw_line(x1 as f32, y1 as f32, x2 as f32, y2 as f32,VECTOR_WIDTH as f32, WHITE);
        }
    }
}
