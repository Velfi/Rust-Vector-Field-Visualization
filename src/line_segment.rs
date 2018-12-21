use ggez::graphics::Point2;

#[derive(Clone)]
pub struct LineSegment {
    pub points: [Point2; 2],
    pub scale: f32,
}

impl LineSegment {
    pub fn new(p0: Point2, p1: Point2, scale: f32) -> Self {
        LineSegment {
            points: [p0, p1],
            scale,
        }
    }

    pub fn from_angle(p0: Point2, angle: f32, scale: f32) -> Self {
        let p1 = Point2::new(scale * angle.cos() + p0.x,
                             scale * angle.sin() + p0.y);
        LineSegment::new(
            p0,
            p1,
            scale,
        )
    }

    pub fn set_p1_relative(&mut self, x: f32, y: f32) {
        self.points[1].x = x * self.scale + self.points[0].x;
        self.points[1].y = y * self.scale + self.points[0].y;
    }
}

impl Default for LineSegment {
    fn default() -> Self {
        LineSegment::new(
            Point2::new(0.0, 0.0),
            Point2::new(1.0, 1.0),
            1.0
        )
    }
}