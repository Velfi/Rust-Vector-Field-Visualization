//! Noise Explorer
//!
//! Controls:
//!
//! N | B       Cycle forward and back through Noise types
//! + | -       Zoom in and out by changing the "scale" of the noise
//! ] | [       Speed up or slow down the rate of change
//! Arrow Keys  Move around by offsetting generated noise
//! O           Reset your offset back to the origin
//! R           Reset speed, scale, and offset
//! H           Show or hide the help screen
//! R           Reset everything
//! H           Show or hide this help screen
//! Esc         Quit and return to the desktop

mod line_segment;

use self::line_segment::LineSegment;
use chrono::Local;
use ggez::{
    conf::{WindowMode, WindowSetup},
    event::{EventHandler, KeyCode, KeyMods},
    graphics, Context, ContextBuilder, GameError, GameResult,
};
use log::{error, info, warn};
use noise::NoiseFn;
use std::{borrow::Borrow, path::PathBuf};
use svg::node::element;

const GRID_CELL_W: f32 = SCREEN_W as f32 / GRID_SIZE_X as f32;
const GRID_CELL_H: f32 = SCREEN_H as f32 / GRID_SIZE_Y as f32;
const GRID_SIZE_X: usize = 96;
const GRID_SIZE_Y: usize = 60;
const DEFAULT_MOVE_SPEED: f32 = 0.05;
const DEFAULT_NOISE_SCALE: f32 = 0.01;
const DEFAULT_NOISE_SCALE_INCREMENT: f32 = DEFAULT_NOISE_SCALE * 0.2;
const DEFAULT_NOISE_SPEED: f32 = 0.00001;
const DEFAULT_NOISE_SPEED_INCREMENT: f32 = DEFAULT_NOISE_SPEED * 0.2;
const SCREEN_W: usize = 1920;
const SCREEN_H: usize = 1080;
const TWO_PI: f32 = std::f32::consts::PI + std::f32::consts::PI;
const VECTOR_SCALE: f32 = 20.0;
const VECTOR_WIDTH: f32 = 2.0;

struct State {
    active_noise_index: Counter,
    base_x_offset: f32,
    base_y_offset: f32,
    line_mesh: Option<graphics::Mesh>,
    line_segments: Vec<LineSegment>,
    noise_scale: f32,
    noise_speed: f32,
    noise_vec: Vec<Box<dyn NoiseFn<[f64; 3]>>>,
    z_offset: f32,
    show_help: bool,
}

impl State {
    fn active_noise(&self) -> &dyn NoiseFn<[f64; 3]> {
        self.noise_vec[self.active_noise_index.count()].borrow()
    }

    fn next_noise(&mut self) {
        self.active_noise_index.increment();
    }

    fn previous_noise(&mut self) {
        self.active_noise_index.decrement();
    }
}

impl EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut y_offset = 0.0 + self.base_y_offset;
        for y in 0..GRID_SIZE_Y {
            let mut x_offset = 0.0 + self.base_x_offset;
            for x in 0..GRID_SIZE_X {
                let angle: f32 = self.active_noise().get([
                    f64::from(x_offset),
                    f64::from(y_offset),
                    f64::from(self.z_offset),
                ]) as f32
                    * TWO_PI;
                let next_line_to_draw = &mut self.line_segments[x + y * GRID_SIZE_X];

                next_line_to_draw.scale = VECTOR_SCALE * angle.atan();
                next_line_to_draw.set_p1_relative(angle.cos(), angle.sin());

                x_offset += self.noise_scale;
            }
            y_offset += self.noise_scale;
            self.z_offset += self.noise_speed;
        }

        let mut line_mesh_builder = graphics::MeshBuilder::new();
        let color = graphics::Color::new(1.0, 1.0, 1.0, 1.0);

        for line_segment in self.line_segments.iter() {
            line_mesh_builder
                .line(&line_segment.points, VECTOR_WIDTH, color)
                .unwrap();
        }

        self.line_mesh = line_mesh_builder.build(ctx).ok();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::Color::new(0.0, 0.0, 0.0, 1.0));

        if let Some(line_mesh) = &self.line_mesh {
            graphics::draw(
                ctx,
                line_mesh,
                graphics::DrawParam::new().dest([0.0, 0.0]).rotation(0.0),
            )?;
        }

        graphics::present(ctx)
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        repeat: bool,
    ) {
        match keycode {
            KeyCode::B if !repeat => self.previous_noise(),
            KeyCode::N if !repeat => self.next_noise(),
            KeyCode::Minus => self.noise_scale += DEFAULT_NOISE_SCALE_INCREMENT,
            KeyCode::Equals => self.noise_scale -= DEFAULT_NOISE_SCALE_INCREMENT,
            KeyCode::LBracket => self.noise_speed -= DEFAULT_NOISE_SPEED_INCREMENT,
            KeyCode::RBracket => self.noise_speed += DEFAULT_NOISE_SPEED_INCREMENT,
            KeyCode::Left => self.base_x_offset -= DEFAULT_MOVE_SPEED,
            KeyCode::Right => self.base_x_offset += DEFAULT_MOVE_SPEED,
            KeyCode::Up => self.base_y_offset -= DEFAULT_MOVE_SPEED,
            KeyCode::Down => self.base_y_offset += DEFAULT_MOVE_SPEED,
            KeyCode::O if !repeat => {
                self.base_x_offset = 0.0;
                self.base_y_offset = 0.0
            }
            KeyCode::R if !repeat => {
                self.z_offset = 0.0;
                self.base_x_offset = 0.0;
                self.base_y_offset = 0.0;
                self.noise_speed = DEFAULT_NOISE_SPEED;
                self.noise_scale = DEFAULT_NOISE_SCALE;
            }
            KeyCode::H => self.show_help = !self.show_help,
            KeyCode::X => export_as_svg(&self),
            KeyCode::Escape => ggez::event::quit(ctx),
            _ => (), // Do nothing
        }
    }
}

fn main() {
    let res = dotenv::dotenv();
    env_logger::init();
    if let Err(err) = res {
        warn!("{}", err)
    };

    let window_mode = WindowMode {
        height: SCREEN_H as f32,
        width: SCREEN_W as f32,
        ..Default::default()
    };

    let window_setup = WindowSetup {
        title: "Vector Field".to_owned(),
        ..Default::default()
    };

    let cb = ContextBuilder::new("Vector Field", "Zelda Hessler")
        .window_setup(window_setup)
        .window_mode(window_mode);
    let (ctx, events_loop) = cb.build().unwrap();

    let noise_vec = gen_noise_vec();

    let state = State {
        active_noise_index: Counter::new(0, noise_vec.len() - 1),
        base_x_offset: 0.0,
        base_y_offset: 0.0,
        line_mesh: None,
        line_segments: gen_line_segments(&noise::Value::new(), 0.0),
        noise_scale: DEFAULT_NOISE_SCALE,
        noise_speed: DEFAULT_NOISE_SPEED,
        noise_vec,
        z_offset: 0.0,
        show_help: true,
    };

    // graphics::set_background_color(ctx, graphics::Color::new(0.0, 0.0, 0.0, 1.0));

    ggez::event::run(ctx, events_loop, state)
}

fn gen_line_segments(noise: &dyn NoiseFn<[f64; 3]>, z_offset: f32) -> Vec<LineSegment> {
    // I wish I didn't have to create these as they get thrown away immediately.
    let mut positions = Vec::with_capacity(GRID_SIZE_X * GRID_SIZE_Y);

    for y in 0..GRID_SIZE_Y {
        for x in 0..GRID_SIZE_X {
            let p0 = [
                x as f32 * GRID_CELL_W + GRID_CELL_W / 2.0,
                y as f32 * GRID_CELL_H + GRID_CELL_H / 2.0,
            ];

            let angle: f32 = noise.get([x as f64, y as f64, f64::from(z_offset)]) as f32 * TWO_PI;

            positions.push(LineSegment::from_angle(p0, angle, VECTOR_SCALE));
        }
    }

    positions
}

fn gen_noise_vec() -> Vec<Box<dyn NoiseFn<[f64; 3]>>> {
    vec![
        Box::new(noise::BasicMulti::new()),
        Box::new(noise::Billow::new()),
        Box::new(noise::Checkerboard::new(1)),
        Box::new(noise::Fbm::new()),
        Box::new(noise::HybridMulti::new()),
        Box::new(noise::OpenSimplex::new()),
        Box::new(noise::Perlin::new()),
        Box::new(noise::Value::new()),
        Box::new(noise::Worley::new()),
    ]
}

// fn noise_index_to_label(index: usize) -> String {
//     String::from(match index {
//         0 => "Heterogenous Multifractal Noise",
//         1 => "Billowy Noise",
//         2 => "Checkerboard Noise",
//         3 => "Fractal Brownian Motion Noise",
//         4 => "Hybrid Multifractal Noise",
//         5 => "Open Simplex Noise",
//         6 => "Perlin Noise",
//         7 => "Value Noise",
//         8 => "Worley Noise",
//         _ => unreachable!(),
//     })
// }

struct Counter {
    count: usize,
    min: usize,
    max: usize,
}

impl Counter {
    pub fn new(min: usize, max: usize) -> Self {
        assert!(min <= max);

        Counter { count: 0, min, max }
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn increment(&mut self) {
        match self.count.checked_add(1) {
            Some(new_count) if new_count > self.max => self.count = self.min,
            Some(new_count) => self.count = new_count,
            None => self.count = self.min,
        }
    }

    pub fn decrement(&mut self) {
        match self.count.checked_sub(1) {
            Some(new_count) => self.count = new_count,
            None => self.count = self.max,
        };
    }
}

fn export_as_svg(state: &State) {
    info!("exporting image as SVG...");
    let base_path = std::env::var("SVG_EXPORT_DIRECTORY");

    if base_path.is_err() {
        error!("SVG export failed: SVG_EXPORT_DIRECTORY must be set to a valid directory in order to export and SVG");
        return;
    }

    let base_path = base_path.unwrap();

    let document = build_svg_document_from_state(state);
    let current_date = Local::today().format("%Y-%m-%d");
    let svg_filename = format!("{}-vector-field-visualization.svg", &current_date);
    let mut svg_filepath: PathBuf = [base_path, svg_filename].iter().collect();

    // I don't want to silently overwrite anything so I look for an unused filename,
    // incrementing the counter until I find an unused number
    // I could have also used a random string/number, I just like this better
    if svg_filepath.exists() {
        let mut counter = 1;

        while svg_filepath.exists() {
            if counter > 100 {
                warn!(
                    "export_as_svg counter has reached {}, you're not in an infinite loop are you?",
                    counter
                );
            }

            let _ = svg_filepath.pop();
            let svg_filename = format!(
                "{}-vector-field-visualization-{}.svg",
                &current_date, &counter
            );
            svg_filepath.push(svg_filename);
            counter += 1;
        }
    }

    svg::save(&svg_filepath, &document).expect("couldn't save SVG");
    info!(
        "SVG successfully exported to {}",
        &svg_filepath.to_string_lossy()
    );
}

fn build_svg_document_from_state(state: &State) -> svg::Document {
    let doc = svg::Document::new().set("viewBox", (0, 0, SCREEN_W, SCREEN_H));

    let mut group = svg::node::element::Group::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", "0.3mm");

    info!("rendering {} lines", state.line_segments.len());

    for line in state.line_segments.iter() {
        let [[x1, y1], [x2, y2]] = line.points;
        let path = element::Line::new()
            .set("x1", x1)
            .set("y1", y1)
            .set("x2", x2)
            .set("y2", y2);

        group = group.add(path);
    }

    let bounding_rect = svg::node::element::Rectangle::new()
        .set("width", SCREEN_W)
        .set("height", SCREEN_H)
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", "1mm");

    doc.add(group).add(bounding_rect)
}
