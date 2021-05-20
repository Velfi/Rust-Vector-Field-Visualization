type Point2<T> = [T; 2];

#[derive(Clone)]
pub struct LineSegment {
    pub points: [Point2<f32>; 2],
    pub scale: f32,
}

impl LineSegment {
    pub fn new(p0: Point2<f32>, p1: Point2<f32>, scale: f32) -> Self {
        LineSegment {
            points: [p0, p1],
            scale,
        }
    }

    pub fn from_angle(p0: Point2<f32>, angle: f32, scale: f32) -> Self {
        let p1 = [scale * angle.cos() + p0[0], scale * angle.sin() + p0[1]];
        LineSegment::new(p0, p1, scale)
    }

    pub fn set_p1_relative(&mut self, x: f32, y: f32) {
        self.points[1][0] = x * self.scale + self.points[0][0];
        self.points[1][1] = y * self.scale + self.points[0][1];
    }
}

impl Default for LineSegment {
    fn default() -> Self {
        LineSegment::new([0.0, 0.0], [1.0, 1.0], 1.0)
    }
}
