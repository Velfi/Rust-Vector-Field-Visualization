mod domain_warping_noise;

use domain_warping_noise::DomainWarpingNoise;
use noise::{NoiseFn, Simplex};

use crate::noise::domain_warping_noise::DomainWarpingNoiseParams;

pub fn new_noise_fn_by_index(index: usize) -> Box<dyn NoiseFn<f64, 3>> {
    match index {
        0 => {
            log::info!("now using Domain Warping Noise generator");
            let dwn_params = DomainWarpingNoiseParams::random();
            Box::new(DomainWarpingNoise::<Simplex>::new(dwn_params))
        }
        1 => {
            log::info!("now using Billowy Noise generator");
            // TODO allow setting frequency, lacunarity, octaves, persistence
            Box::new(noise::Billow::<Simplex>::new(0))
        }
        2 => {
            log::info!("now using Heterogenous Multifractal Noise generator");
            Box::new(noise::BasicMulti::<Simplex>::new(0))
        }
        3 => {
            log::info!("now using Fractal Brownian Motion Noise generator");
            Box::new(noise::Fbm::<Simplex>::new(0))
        }
        4 => {
            log::info!("now using Hybrid Multifractal Noise generator");
            Box::new(noise::HybridMulti::<Simplex>::new(0))
        }
        5 => {
            log::info!("now using Open Simplex Noise generator");
            Box::new(noise::OpenSimplex::new(0))
        }
        6 => {
            log::info!("now using Perlin Noise generator");
            Box::new(noise::Perlin::new(0))
        }
        7 => {
            log::info!("now using Value Noise generator");
            Box::new(noise::Value::new(0))
        }
        8 => {
            log::info!("now using Worley Noise generator");
            Box::new(noise::Worley::new(0))
        }
        _ => unreachable!(),
    }
}
