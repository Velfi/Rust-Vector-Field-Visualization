use crate::circles::Circles;
use crate::consts::{
    DEFAULT_MOVE_SPEED, DEFAULT_NOISE_SCALE, DEFAULT_NOISE_SCALE_INCREMENT, DEFAULT_NOISE_SPEED,
    DEFAULT_NOISE_SPEED_INCREMENT,
};
use crate::counter::Counter;
use crate::line_segments::LineSegments;
use crate::visualizer::{Visualizer, VisualizerParams};
use chrono::Local;
use ggez::{
    event::{EventHandler, KeyCode, KeyMods},
    graphics, Context, GameError, GameResult,
};
use log::{error, info, warn};
use noise::{Fbm, NoiseFn};
use std::path::PathBuf;

pub struct State {
    active_noise_index: Counter,
    active_visualizer_index: Counter,
    params: VisualizerParams,
    visualizer: Box<dyn Visualizer>,
    show_help: bool,
}

impl State {
    pub fn new() -> Self {
        let params = VisualizerParams::default();
        let visualizer = Box::new(LineSegments::new(&params));
        // let visualizer = Box::new(Circles::new(&params));

        Self {
            // There are nine noise algos to choose from
            active_noise_index: Counter::new(0, 8),
            active_visualizer_index: Counter::new(0, 1),
            visualizer,
            params,
            show_help: true,
        }
    }

    fn next_noise(&mut self) {
        self.active_noise_index.increment();
        self.set_noise_fn(self.active_noise_index.count());
    }

    fn previous_noise(&mut self) {
        self.active_noise_index.decrement();
        self.set_noise_fn(self.active_noise_index.count());
    }

    fn next_visualizer(&mut self) {
        self.active_visualizer_index.increment();
        self.set_visualizer(self.active_visualizer_index.count());
    }

    fn previous_visualizer(&mut self) {
        self.active_visualizer_index.decrement();
        self.set_visualizer(self.active_visualizer_index.count());
    }

    fn set_noise_fn(&mut self, index: usize) {
        match index {
            0 => {
                println!("now using Domain Warping Noise generator");
                // println!("now using Billowy Noise generator");
                // self.params.noise_fn = Box::new(noise::Billow::new());
                // Image for f(p) = fbm(p+fbm(p+fbm(p)))
                let noise_fn = DomainWarpingNoise::new();
                self.params.noise_fn = Box::new(noise_fn);
            }
            1 => {
                println!("now using Heterogenous Multifractal Noise generator");
                self.params.noise_fn = Box::new(noise::BasicMulti::new());
            }
            2 => {
                println!("now using Checkerboard Noise generator");
                self.params.noise_fn = Box::new(noise::Checkerboard::new(1));
            }
            3 => {
                println!("now using Fractal Brownian Motion Noise generator");
                self.params.noise_fn = Box::new(noise::Fbm::new());
            }
            4 => {
                println!("now using Hybrid Multifractal Noise generator");
                self.params.noise_fn = Box::new(noise::HybridMulti::new());
            }
            5 => {
                println!("now using Open Simplex Noise generator");
                self.params.noise_fn = Box::new(noise::OpenSimplex::new());
            }
            6 => {
                println!("now using Perlin Noise generator");
                self.params.noise_fn = Box::new(noise::Perlin::new());
            }
            7 => {
                println!("now using Value Noise generator");
                self.params.noise_fn = Box::new(noise::Value::new());
            }
            8 => {
                println!("now using Worley Noise generator");
                self.params.noise_fn = Box::new(noise::Worley::new());
            }
            _ => unreachable!(),
        }
    }

    fn set_visualizer(&mut self, index: usize) {
        match index {
            0 => {
                println!("now using Line Segments visualizer");
                self.visualizer = Box::new(LineSegments::new(&self.params));
            }
            1 => {
                println!("now using Circles visualizer");
                self.visualizer = Box::new(Circles::new(&self.params));
            }
            _ => unreachable!(),
        }
    }

    fn export_as_svg(&self) {
        info!("exporting image as SVG...");
        let base_path = std::env::var("SVG_EXPORT_DIRECTORY");

        if base_path.is_err() {
            error!("SVG export failed: SVG_EXPORT_DIRECTORY must be set to a valid directory in order to export and SVG");
            return;
        }

        let base_path = base_path.unwrap();

        let document = self.visualizer.build_svg_document_from_state();
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
}

impl EventHandler<GameError> for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.visualizer.update(&mut self.params);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::Color::new(0.0, 0.0, 0.0, 1.0));

        if let Some(line_mesh) = &self.visualizer.build_mesh(ctx) {
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
            KeyCode::J if !repeat => self.previous_visualizer(),
            KeyCode::K if !repeat => self.next_visualizer(),
            KeyCode::Minus => self.params.noise_scale += DEFAULT_NOISE_SCALE_INCREMENT,
            KeyCode::Equals => self.params.noise_scale -= DEFAULT_NOISE_SCALE_INCREMENT,
            KeyCode::LBracket => self.params.noise_speed -= DEFAULT_NOISE_SPEED_INCREMENT,
            KeyCode::RBracket => self.params.noise_speed += DEFAULT_NOISE_SPEED_INCREMENT,
            KeyCode::Left => self.params.base_x_offset -= DEFAULT_MOVE_SPEED,
            KeyCode::Right => self.params.base_x_offset += DEFAULT_MOVE_SPEED,
            KeyCode::Up => self.params.base_y_offset -= DEFAULT_MOVE_SPEED,
            KeyCode::Down => self.params.base_y_offset += DEFAULT_MOVE_SPEED,
            KeyCode::O if !repeat => {
                self.params.base_x_offset = 0.0;
                self.params.base_y_offset = 0.0
            }
            KeyCode::R if !repeat => {
                self.params.z_offset = 0.0;
                self.params.base_x_offset = 0.0;
                self.params.base_y_offset = 0.0;
                self.params.noise_speed = DEFAULT_NOISE_SPEED;
                self.params.noise_scale = DEFAULT_NOISE_SCALE;
            }
            KeyCode::H => self.show_help = !self.show_help,
            KeyCode::X => self.export_as_svg(),
            KeyCode::Escape => ggez::event::quit(ctx),
            _ => (), // Do nothing
        }
    }
}

struct DomainWarpingNoise {
    fbm: Fbm,
}

impl DomainWarpingNoise {
    pub fn new() -> Self {
        Self { fbm: Fbm::new() }
    }
}

// Image for f(p) = fbm(p+fbm(p+fbm(p)))
impl NoiseFn<[f64; 3]> for DomainWarpingNoise {
    fn get(&self, p: [f64; 3]) -> f64 {
        let q = vec3(
            self.fbm.get(add(p, vec3(0.0, 0.0, 1.0))),
            self.fbm.get(add(p, vec3(5.2, 1.3, 1.0))),
            self.fbm.get(add(p, vec3(1.0, 1.0, 1.0))),
        );

        let q = mul_n(q, 4.0);

        let r = vec3(
            self.fbm.get(add(p, add(q, vec3(1.7, 9.2, 1.0)))),
            self.fbm.get(add(p, add(q, vec3(8.3, 2.8, 1.0)))),
            self.fbm.get(add(p, add(q, vec3(1.0, 1.0, 1.0)))),
        );

        let r = mul_n(r, 4.0);
        self.fbm.get(add(p, r))
    }
}

fn vec3(x: f64, y: f64, z: f64) -> [f64; 3] {
    [x, y, z]
}

fn add(xyz_1: [f64; 3], xyz_2: [f64; 3]) -> [f64; 3] {
    let [x1, y1, z1] = xyz_1;
    let [x2, y2, z2] = xyz_2;

    [x1 + x2, y1 + y2, z1 + z2]
}

fn mul_n(xyz: [f64; 3], n: f64) -> [f64; 3] {
    let [x, y, z] = xyz;

    [x * n, y * n, z * n]
}
