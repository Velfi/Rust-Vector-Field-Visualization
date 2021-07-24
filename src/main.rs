//! Noise Explorer
//!
//! Controls:
//!
//! N | B       Cycle forward and back through Noise types
//! + | -       Zoom in and out by changing the "scale" of the noise
//! ] | [       Speed up or slow down the rate of change
//! Arrow Keys  Move around by offsetting generated noise
//! O           Reset your offset back to the origin
//! R           Reset speed, scale, and offset
//! H           Show or hide the help screen
//! R           Reset everything
//! H           Show or hide this help screen
//! Esc         Quit and return to the desktop

mod circles;
mod consts;
mod counter;
mod line_segments;
mod state;
mod visualizer;

use consts::{SCREEN_H, SCREEN_W};
use ggez::{
    conf::{WindowMode, WindowSetup},
    ContextBuilder,
};
use log::warn;
use state::State;

fn main() {
    let res = dotenv::dotenv();
    env_logger::init();
    if let Err(err) = res {
        warn!("{}", err)
    };

    let window_mode = WindowMode {
        height: SCREEN_H as f32,
        width: SCREEN_W as f32,
        ..Default::default()
    };

    let window_setup = WindowSetup {
        title: "Vector Field".to_owned(),
        ..Default::default()
    };

    let cb = ContextBuilder::new("Vector Field", "Zelda Hessler")
        .window_setup(window_setup)
        .window_mode(window_mode);
    let (ctx, events_loop) = cb.build().unwrap();

    let state = State::new();
    // graphics::set_background_color(ctx, graphics::Color::new(0.0, 0.0, 0.0, 1.0));

    ggez::event::run(ctx, events_loop, state)
}
