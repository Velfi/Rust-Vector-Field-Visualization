use ggez::{
    event,
    GameResult,
    Context,
    graphics,
    conf,
};
use noise::{
    Perlin,
    NoiseFn,
};

mod line_segment;

use self::line_segment::LineSegment;

const GRID_CELL_W: f32 = SCREEN_W as f32 / GRID_SIZE_X as f32;
const GRID_CELL_H: f32 = SCREEN_H as f32 / GRID_SIZE_Y as f32;
const GRID_SIZE_X: usize = 48;
const GRID_SIZE_Y: usize = 30;
const NOISE_SCALE: f32 = 0.03;
const NOISE_SPEED: f32 = 0.00001;
const SCREEN_W: usize = 1440;
const SCREEN_H: usize = 900;
const TWO_PI: f32 = std::f32::consts::PI + std::f32::consts::PI;
const VECTOR_SCALE: f32 = 20.0;
const VECTOR_WIDTH: f32 = 3.0;

struct State {
    line_segments: Vec<LineSegment>,
    line_mesh: graphics::Mesh,
    perlin: Perlin,
    z_offset: f32,
}

impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut y_offset = 0.0;
        for y in 0..GRID_SIZE_Y {
            let mut x_offset = 0.0;
            for x in 0..GRID_SIZE_X {
                let next_line_to_draw = &mut self.line_segments[x + y * GRID_SIZE_X];
                let angle: f32 = self.perlin.get([x_offset as f64, y_offset as f64, self.z_offset as f64]) as f32 * TWO_PI;

                next_line_to_draw.scale = VECTOR_SCALE * angle.atan();
                next_line_to_draw.set_p1_relative(
                    angle.cos(),
                    angle.sin(),
                );

                x_offset += NOISE_SCALE;
            }
            y_offset += NOISE_SCALE;
            self.z_offset += NOISE_SPEED;
        }

        let mut line_mesh_builder = graphics::MeshBuilder::new();

        for line_segment in self.line_segments.iter() {
            line_mesh_builder.line(&line_segment.points, VECTOR_WIDTH);
        }

        self.line_mesh = line_mesh_builder.build(ctx).expect("Error: failed to build line mesh");

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        graphics::draw(ctx, &self.line_mesh, graphics::Point2::new(0.0, 0.0), 0.0).expect("Error: failed to draw the line mesh");

        graphics::present(ctx);
        Ok(())
    }
}

fn main() {
    let mut c = conf::Conf::new();
    c.window_mode.fullscreen_type = conf::FullscreenType::True;
    c.window_mode.height = SCREEN_H as u32;
    c.window_mode.width = SCREEN_W as u32;

    let ctx = &mut Context::load_from_conf("Vector Field", "Zelda Hessler", c).unwrap();

    let perlin = Perlin::new();
    let line_mesh = graphics::MeshBuilder::new().build(ctx).expect("Error: failed to build empty mesh");

    let state = &mut State {
        line_segments: gen_line_segments(&perlin, 0.0),
        line_mesh,
        perlin,
        z_offset: 0.0,
    };

    graphics::set_background_color(ctx, graphics::Color::new(
        0.0, 0.0, 0.0, 1.0,
    ));

    event::run(ctx, state).unwrap();
}

fn gen_line_segments(perlin: &Perlin, z_offset: f32) -> Vec<LineSegment> {
    // I wish I didn't have to creat these as they get thrown away immediately.
    let mut positions = Vec::with_capacity(GRID_SIZE_X * GRID_SIZE_Y);

    for y in 0..GRID_SIZE_Y {
        for x in 0..GRID_SIZE_X {
            let p0 = graphics::Point2::new(
                x as f32 * GRID_CELL_W + GRID_CELL_W / 2.0,
                y as f32 * GRID_CELL_H + GRID_CELL_H / 2.0,
            );

            let angle: f32 = perlin.get([x as f64, y as f64, z_offset as f64]) as f32 * TWO_PI;

            positions.push(LineSegment::from_angle(p0, angle, VECTOR_SCALE));
        }
    }

    positions
}
