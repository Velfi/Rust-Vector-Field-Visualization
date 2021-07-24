mod domain_warping_noise;

use domain_warping_noise::DomainWarpingNoise;
use noise::NoiseFn;

pub fn new_noise_fn_by_index(index: usize) -> Box<dyn NoiseFn<[f64; 3]>> {
    match index {
        0 => {
            println!("now using Billowy Noise generator");
            Box::new(noise::Billow::new())
        }
        1 => {
            println!("now using Heterogenous Multifractal Noise generator");
            Box::new(noise::BasicMulti::new())
        }
        2 => {
            println!("now using Domain Warping Noise generator");
            Box::new(DomainWarpingNoise::new())
        }
        3 => {
            println!("now using Fractal Brownian Motion Noise generator");
            Box::new(noise::Fbm::new())
        }
        4 => {
            println!("now using Hybrid Multifractal Noise generator");
            Box::new(noise::HybridMulti::new())
        }
        5 => {
            println!("now using Open Simplex Noise generator");
            Box::new(noise::OpenSimplex::new())
        }
        6 => {
            println!("now using Perlin Noise generator");
            Box::new(noise::Perlin::new())
        }
        7 => {
            println!("now using Value Noise generator");
            Box::new(noise::Value::new())
        }
        8 => {
            println!("now using Worley Noise generator");
            Box::new(noise::Worley::new())
        }
        _ => unreachable!(),
    }
}
