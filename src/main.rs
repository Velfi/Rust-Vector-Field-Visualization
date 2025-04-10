//! Noise Explorer
//!
//! Controls:
//!
//! - N | B       Cycle forward and back through Noise types
//! - J | K       Change the visualizer kind (circles vs. lines)
//! - + | -       Zoom in and out by changing the "scale" of the noise
//! - ] | [       Speed up or slow down the rate of change
//! - Arrow Keys  Move around by offsetting generated noise
//! - O           Reset your offset back to the origin
//! - R           Reset speed, scale, and offset
//! - H           Show or hide this help screen
//! - Esc         Quit

mod circles;
mod consts;
mod counter;
mod line_segments;
mod noise;
mod state;
mod visualizer;

use consts::{SCREEN_H, SCREEN_W};
use log::warn;
use state::State;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Vector Field Visualization".to_owned(),
        fullscreen: false,
        window_width: SCREEN_W as i32,
        window_height: SCREEN_H as i32,
        sample_count: 4,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let res = dotenv::dotenv();
    env_logger::init();
    if let Err(err) = res {
        warn!("{err}")
    };
    let mut state = State::new();

    loop {
        clear_background(BLACK);

        state.update();
        state.render();

        next_frame().await
    }
}
