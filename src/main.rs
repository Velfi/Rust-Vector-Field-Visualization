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

const GRID_CELL_H: f32 = SCREEN_H as f32 / GRID_SIZE_Y as f32;
const GRID_CELL_W: f32 = SCREEN_W as f32 / GRID_SIZE_X as f32;
const GRID_SIZE_X: usize = 60;
const GRID_SIZE_Y: usize = 35;
const SCREEN_H: usize = 1050;
const SCREEN_W: usize = 1680;
const TWO_PI: f32 = std::f32::consts::PI + std::f32::consts::PI;
const VECTOR_SCALE: f32 = 25.0;
const VECTOR_WIDTH: f32 = 3.0;
const OFFSET_INCREMENT: f32 = 0.04;
const Z_INCREMENT: f32 = OFFSET_INCREMENT * 0.002;


struct State {
    line_segments: Vec<LineSegment>,
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

                next_line_to_draw.scale = VECTOR_SCALE * angle.sin();
                next_line_to_draw.set_p1_relative(
                    angle.cos(),
                    angle.sin(),
                );

                x_offset += OFFSET_INCREMENT;
            }
            y_offset += OFFSET_INCREMENT;
            self.z_offset += Z_INCREMENT;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        for line_segment in self.line_segments.iter() {
            graphics::line(ctx, &line_segment.points, VECTOR_WIDTH).expect("Error: failed to draw a line");
        }

        graphics::present(ctx);
        Ok(())
    }
}

fn main() {
    let mut c = conf::Conf::new();
    c.window_mode.fullscreen_type = conf::FullscreenType::Off;
    c.window_mode.height = SCREEN_H as u32;
    c.window_mode.width = SCREEN_W as u32;

    let ctx = &mut Context::load_from_conf("Vector Field", "Zelda Hessler", c).unwrap();

    let perlin = Perlin::new();

    let state = &mut State {
        line_segments: gen_line_segments(&perlin, 0.0),
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

//
//fn gen_line_points(
//    perlin_noise: Perlin,
//    x: f64,
//    y: f64,
//    x_off: f64,
//    y_off: f64,
//    grid_size_x: f64,
//    grid_size_y: f64,
//    canvas_x: f64,
//    canvas_y: f64,
//    vector_scale: f64,
//    time_float: f64
//) -> [graphics::Point2; 2] {
//    let noise_amt = perlin_noise.get([x_off, y_off, time_float]) * TWO_PI;
//
//    let x0: f64 = x * (canvas_x / grid_size_x) + (canvas_x / grid_size_x / 2.0);
//    let y0: f64 = y * (canvas_y / grid_size_y) + (canvas_y / grid_size_y / 2.0);
//
//    let scale = vector_scale * noise_amt;
//
//    let x1: f64 = scale * noise_amt.cos() + x0;
//    let y1: f64 = scale * noise_amt.sin() + y0;
//
//    [
//        graphics::Point2::new(x0 as f32, y0 as f32),
//        graphics::Point2::new(x1 as f32, y1 as f32)
//    ]
//}
//
//fn gen_lines(
//    ctx: &mut Context,
//    vector_scale: f64,
//    grid_size_x: usize,
//    grid_size_y: usize,
//    canvas_x: usize,
//    canvas_y: usize,
//    perlin_noise: Perlin,
//) -> Vec<Mesh> {
//    let mut line_positions = Vec::with_capacity(grid_size_x * grid_size_y);
//
//    let inc: f64 = 0.1;
//    let time_duration = ggez::timer::get_time_since_start(ctx);
//    let time_float = time_duration.as_secs() as f64 +
//        time_duration.subsec_nanos() as f64 / 1_000_000.0;
//
//    let mut y_off = 0.0;
//    for y in 0..grid_size_y {
//        let mut x_off = 0.0;
//        for x in 0..grid_size_x {
//            line_positions.push(gen_line_points(
//                perlin_noise,
//                x as f64,
//                y as f64,
//                x_off,
//                y_off,
//                grid_size_x as f64,
//                grid_size_y as f64,
//                canvas_x as f64,
//                canvas_y as f64,
//                vector_scale,
//                time_float / 1_000.0
//            ));
//
//            x_off += inc;
//        }
//        y_off += inc;
//    }
//
//    line_positions.into_iter().map(
//        |point_pair| -> Mesh {
//            Mesh::new_line(ctx, &point_pair, 3.0).expect("Error: failed to create line mesh")
//        }
//    ).collect()
//}

//const canvasX = 1920;
//const canvasY = 1080;
//const inc = 0.1;
//const speed = 0.002;
//const vectorScale = 25;
//const gridSizeX = 60;
//const gridSizeY = 35;
//let zOffset = 0;
//
//function setup() {
//  createCanvas(canvasX, canvasY);
//  background(0);
//  stroke(185);
//  strokeWeight(3);
//  frameRate(30);
//}
//
//function draw() {
//  background(0);
//  let yoff = 0;
//  for (let y = 0; y < gridSizeY; y++) {
//    let xoff = 0;
//    for (let x = 0; x < gridSizeX; x++) {
//      const angle = noise(xoff, yoff, zOffset) * TWO_PI;
//      const vector = p5.Vector.fromAngle(angle);
//      const xTrans = x * (canvasX / gridSizeX) + (canvasX / gridSizeX / 2);
//      const yTrans = y * (canvasY / gridSizeY) + (canvasY / gridSizeY / 2);
//      xoff += inc;
//      fill(angle);
//      push();
//      translate(xTrans, yTrans);
//      rotate(vector.heading());
//      line(0, 0, vectorScale * sin(angle), 0);
//      pop();
//    }
//    yoff += inc;
//    zOffset += inc * speed;
//  }
//}