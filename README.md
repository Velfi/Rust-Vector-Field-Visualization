# Vector Field Noise Visualizer

A visualizer to see what kinds of shapes different noise algorithms make.

The noise algorithms demonstrated are:

- Heterogenous Multifractal Noise
- Billowy Noise
- Checkerboard Noise
- Fractal Brownian Motion Noise
- Hybrid Multifractal Noise
- Open Simplex Noise
- Perlin Noise
- Value Noise
- Worley Noise

![example]

## Running The Visualizer

You'll need to have Rust and `cargo` installed. Then, run `cargo run --release` in you terminal of choice.

## Controls

| key        | what it does                                         |
| ---------- | ---------------------------------------------------- |
| N and B    | Cycle forward and back through Noise types           |
| + and -    | Zoom in and out by changing the "scale" of the noise |
| ] and [    | Speed up or slow down the rate of change             |
| Arrow Keys | Move around by offsetting generated noise            |
| O          | Reset your offset back to the origin                 |
| R          | Reset speed, scale, and offset                       |
| H          | Show or hide the help screen                         |
| R          | Reset everything                                     |
| H          | Show or hide this help screen                        |
| Esc        | Quit and return to the desktop                       |

[example]: /example.png "An example of the visualizer"
