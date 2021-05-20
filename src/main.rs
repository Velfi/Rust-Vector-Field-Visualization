mod world_view;

use nannou::prelude::*;
use std::sync::Arc;
use world_view::WorldView;
use zelda_lib::line_segment::LineSegment;

const SCREEN_W: u32 = 1280;
const SCREEN_H: u32 = 720;
const VECTOR_WIDTH: f64 = 3.0;
const VECTOR_SCALE: f64 = 20.0;
const DEFAULT_MOVE_SPEED: f64 = 6.3;
const DEFAULT_NOISE_SCALE: f64 = 0.01;
const DEFAULT_NOISE_SCALE_INCREMENT: f64 = DEFAULT_NOISE_SCALE * 0.2;
const DEFAULT_NOISE_SPEED: f64 = 0.01;
const DEFAULT_NOISE_SPEED_INCREMENT: f64 = DEFAULT_NOISE_SPEED * 0.2;

fn main() {
    nannou::app(model).update(update).run();
}

type ThreadSafeNoiseFn = Box<dyn noise::NoiseFn<[f64; 3]> + Send + Sync>;

struct Model {
    _window: Option<window::Id>,
    noise_fn: ThreadSafeNoiseFn,
    line_segments: Vec<LineSegment<f64>>,
    noise_scale: f64,
    noise_speed: f64,
    world_view: WorldView,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(SCREEN_W, SCREEN_H)
        .title("Vector Field Visualization")
        .view(view)
        .key_pressed(key_pressed)
        .build()
        // This is never going to fail, I promise
        .ok();

    let noise_fn = Box::new(noise::OpenSimplex::new()) as ThreadSafeNoiseFn;
    let line_segments = Vec::new();
    let world_view = WorldView::new(
        0.0,
        0.0,
        (SCREEN_W as f64) / VECTOR_SCALE,
        (SCREEN_H as f64) / VECTOR_SCALE,
    );
    app.set_fullscreen_on_shortcut(true);

    Model {
        _window,
        noise_fn,
        line_segments,
        noise_scale: DEFAULT_NOISE_SCALE,
        noise_speed: DEFAULT_NOISE_SPEED,
        world_view,
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    model.line_segments.clear();

    let grid_refs = model.world_view.grid_references_in_view();
    let noise_fn = Arc::new(&model.noise_fn);
    let delta_t = update.since_start.as_secs_f64() * model.noise_speed;

    model.line_segments = grid_refs
        .iter()
        .map(|(x, y)| ((*x as f64) * VECTOR_SCALE, (*y as f64) * VECTOR_SCALE))
        .map(|(x, y)| {
            let p0 = zelda_lib::point2::Point2 { x, y };
            let angle = noise_fn.get([x * model.noise_scale, y * model.noise_scale, delta_t])
                * std::f64::consts::TAU;

            LineSegment::from_angle(p0, angle, VECTOR_SCALE)
        })
        .collect();
}

fn key_pressed(app: &App, model: &mut Model, keycode: Key) {
    match keycode {
        // Key::B => model.previous_noise(),
        // Key::N => model.next_noise(),
        Key::Minus => {
            model.noise_scale += DEFAULT_NOISE_SCALE_INCREMENT;
            println!("Noise Scale {}", model.noise_scale)
        }
        Key::Equals => {
            model.noise_scale -= DEFAULT_NOISE_SCALE_INCREMENT;
            println!("Noise Scale {}", model.noise_scale)
        }
        Key::LBracket => {
            model.noise_speed -= DEFAULT_NOISE_SPEED_INCREMENT;
            println!("Noise Speed {}", model.noise_speed)
        }
        Key::RBracket => {
            model.noise_speed += DEFAULT_NOISE_SPEED_INCREMENT;
            println!("Noise Speed {}", model.noise_speed)
        }
        Key::Left => {
            model.world_view.move_relative(DEFAULT_MOVE_SPEED, 0.0);
            println!("Camera Position {:?}", model.world_view.xy())
        }
        Key::Right => {
            model.world_view.move_relative(-DEFAULT_MOVE_SPEED, 0.0);
            println!("Camera Position {:?}", model.world_view.xy())
        }
        Key::Up => {
            model.world_view.move_relative(0.0, -DEFAULT_MOVE_SPEED);
            println!("Camera Position {:?}", model.world_view.xy())
        }
        Key::Down => {
            model.world_view.move_relative(0.0, DEFAULT_MOVE_SPEED);
            println!("Camera Position {:?}", model.world_view.xy())
        }
        Key::O => {
            model.world_view.move_absolute(0.0, 0.0);
        }
        Key::R => {
            model.world_view.move_absolute(0.0, 0.0);
            model.noise_speed = DEFAULT_NOISE_SPEED;
            model.noise_scale = DEFAULT_NOISE_SCALE;
        }
        // Key::H => model.show_help = !model.show_help,
        Key::Escape => app.quit(),
        _ => (), // Do nothing
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    model.line_segments.iter().for_each(|line_segment| {
        let (start, end) = vectors_from_line_segment(line_segment);

        draw.line()
            .caps_round()
            .color(LIGHTGRAY)
            .weight(VECTOR_WIDTH as f32)
            .start(start)
            .end(end)
            .finish();
    });

    draw.to_frame(app, &frame).unwrap();
}

// TODO This conversion business is ugly, why does nannou needs `Vector2<f32>`s?
fn vectors_from_line_segment(line_segment: &LineSegment<f64>) -> (Vector2, Vector2) {
    let p0 = line_segment.p0();
    let p1 = line_segment.p1();

    (
        Vector2::new(p0.x as f32, p0.y as f32),
        Vector2::new(p1.x as f32, p1.y as f32),
    )
}
