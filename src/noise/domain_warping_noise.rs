use log::trace;
use noise::{Fbm, NoiseFn};
use rand::{prelude::StdRng, Rng, SeedableRng};

pub struct DomainWarpingNoise {
    fbm: Fbm,
    params: DomainWarpingNoiseParams,
}

impl DomainWarpingNoise {
    pub fn new(params: DomainWarpingNoiseParams) -> Self {
        Self {
            fbm: Fbm::new(),
            params,
        }
    }

    pub fn inigo() -> Self {
        Self {
            fbm: Fbm::new(),
            params: DomainWarpingNoiseParams::inigo(),
        }
    }
}

impl Default for DomainWarpingNoise {
    fn default() -> Self {
        Self {
            fbm: Fbm::new(),
            params: Default::default(),
        }
    }
}

#[derive(Debug)]
pub struct DomainWarpingNoiseParams {
    q: [f64; 9],
    r: [f64; 9],
    qn: f64,
    rn: f64,
}

impl DomainWarpingNoiseParams {
    pub fn new(q: [f64; 9], r: [f64; 9], qn: f64, rn: f64) -> Self {
        Self { q, r, qn, rn }
    }

    pub fn random() -> Self {
        // rng.gen handles arrays just fine but fills them with numbers between 0.0 and 1.0
        // gen_range allows for numbers from a larger range but it can't handle arrays :(
        let mut rng = StdRng::from_entropy();
        let q_range = -100.0..100.0;
        let q: [f64; 9] = [
            rng.gen_range(q_range.clone()),
            rng.gen_range(q_range.clone()),
            rng.gen_range(q_range.clone()),
            rng.gen_range(q_range.clone()),
            rng.gen_range(q_range.clone()),
            rng.gen_range(q_range.clone()),
            rng.gen_range(q_range.clone()),
            rng.gen_range(q_range.clone()),
            rng.gen_range(q_range.clone()),
        ];

        let r_range = -100.0..100.0;
        let r: [f64; 9] = [
            rng.gen_range(r_range.clone()),
            rng.gen_range(r_range.clone()),
            rng.gen_range(r_range.clone()),
            rng.gen_range(r_range.clone()),
            rng.gen_range(r_range.clone()),
            rng.gen_range(r_range.clone()),
            rng.gen_range(r_range.clone()),
            rng.gen_range(r_range.clone()),
            rng.gen_range(r_range.clone()),
        ];
        let qn: f64 = rng.gen_range(0.001..5.0);
        let rn: f64 = rng.gen_range(0.001..5.0);

        let s = Self { q, r, qn, rn };

        trace!(
            "Creating random parameters for domain warping noise:\n{:?}",
            s
        );

        s
    }

    // https://www.iquilezles.org/www/articles/warp/warp.htm
    fn inigo() -> Self {
        Self {
            q: [0.0, 0.0, 0.0, 5.2, 1.3, 0.0, 0.0, 0.0, 0.0],
            r: [1.7, 9.2, 0.0, 8.3, 2.8, 0.0, 0.0, 0.0, 0.0],
            qn: 4.0,
            rn: 4.0,
        }
    }
}

impl Default for DomainWarpingNoiseParams {
    fn default() -> Self {
        Self {
            q: Default::default(),
            r: Default::default(),
            qn: 1.0,
            rn: 1.0,
        }
    }
}

// Image for f(p) = fbm(p+fbm(p+fbm(p)))
impl NoiseFn<[f64; 3]> for DomainWarpingNoise {
    fn get(&self, p: [f64; 3]) -> f64 {
        let [q0, q1, q2, q3, q4, q5, q6, q7, q8] = self.params.q;
        let q = vec3(
            self.fbm.get(add(p, vec3(q0, q1, q2))),
            self.fbm.get(add(p, vec3(q3, q4, q5))),
            self.fbm.get(add(p, vec3(q6, q7, q8))),
        );

        let q = mul_n(q, self.params.qn);

        let [r0, r1, r2, r3, r4, r5, r6, r7, r8] = self.params.r;
        let r = vec3(
            self.fbm.get(add(p, add(q, vec3(r0, r1, r2)))),
            self.fbm.get(add(p, add(q, vec3(r3, r4, r5)))),
            self.fbm.get(add(p, add(q, vec3(r6, r7, r8)))),
        );

        let r = mul_n(r, self.params.rn);
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
