use noise::{Fbm, NoiseFn};

pub struct DomainWarpingNoise {
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
