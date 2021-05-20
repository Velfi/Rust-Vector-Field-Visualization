use nannou::{
    math::cgmath::{Point3, Vector3},
    math::Matrix4,
};

// A simple first person camera.
pub struct Camera {
    // The position of the camera.
    pub eye: Point3<f32>,
    // Rotation around the x axis.
    pub pitch: f32,
    // Rotation around the y axis.
    pub yaw: f32,
}

// The vertex type that we will use to represent a point on our triangle.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vertex {
    position: (f32, f32, f32),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Normal {
    normal: (f32, f32, f32),
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Uniforms {
    world: Matrix4<f32>,
    view: Matrix4<f32>,
    proj: Matrix4<f32>,
}

impl Camera {
    // Calculate the direction vector from the pitch and yaw.
    fn direction(&self) -> Vector3<f32> {
        pitch_yaw_to_direction(self.pitch, self.yaw)
    }

    // The camera's "view" matrix.
    fn view(&self) -> Matrix4<f32> {
        let direction = self.direction();
        let up = Vector3::new(0.0, 1.0, 0.0);
        Matrix4::look_at_dir(self.eye, direction, up)
    }
}

fn pitch_yaw_to_direction(pitch: f32, yaw: f32) -> Vector3<f32> {
    let xz_unit_len = pitch.cos();
    let x = xz_unit_len * yaw.cos();
    let y = pitch.sin();
    let z = xz_unit_len * (-yaw).sin();
    Vector3::new(x, y, z)
}
