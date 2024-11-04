# 🎨 Pixel Loop 🔁

[![Crate](https://img.shields.io/crates/v/pixel_loop.svg)](https://crates.io/crates/pixel_loop)
[![Documentation](https://docs.rs/pixel_loop/badge.svg)](https://docs.rs/pixel_loop)

## Warning: **WORK IN PROGRESS**

This crate/library is still heavily being worked on. The API is not considered to be stable at this point in time. If you want to follow the development check out the following youtube channel [MrJakob](https://youtube.com/c/mrjakob).

## About

A Rust game loop implementation providing a solid foundation for building games and interactive applications. Inspired by the concepts from [Fix Your Timestep](https://gafferongames.com/post/fix_your_timestep/), it offers fixed timestep updates with variable rendering, supporting both windowed and terminal-based applications.

## Motivation

The idea behind Pixel Loop resonated with me as I have often faced challenges with timing aspects while working on animations from scratch. This project serves as a practical exploration of fixed time game/update loops and lays the groundwork for future experiments and projects.

## Installation

Add Pixel Loop to your `Cargo.toml`:

```toml
[dependencies]
pixel_loop = "*"
```

### Feature Flags

- `winit` - Enable window-based rendering
- `crossterm` - Enable terminal-based rendering
- `image-load` - Enable image loading support for canvases

By default all flags are currently enabled. If you only need a specific one, you may only use enable the backend/feature you specifically need, to cut down on compilation time and filesize.

## Examples

### Terminal Application

Create a simple moving box in your terminal:

```rust
use pixel_loop::{run, canvas::CrosstermCanvas, input::CrosstermInputState};
use pixel_loop::color::Color;
use pixel_loop::input::KeyboardKey;
use anyhow::Result;

struct GameState {
    box_pos: (i64, i64),
}

fn main() -> Result<()> {
    let mut canvas = CrosstermCanvas::new(80, 24);  // Terminal size
    canvas.set_refresh_limit(60);

    let state = GameState { box_pos: (0, 0) };
    let input = CrosstermInputState::new();

    run(
        60,  // Updates per second
        state,
        input,
        canvas,
        // Update function - fixed timestep
        |_env, state, input, _canvas| {
            if input.is_key_down(KeyboardKey::Right) {
                state.box_pos.0 += 1;
            }
            if input.is_key_pressed(KeyboardKey::Q) {
                std::process::exit(0);
            }
            Ok(())
        },
        // Render function - variable timestep
        |_env, state, _input, canvas, _dt| {
            canvas.clear_screen(&Color::from_rgb(0, 0, 0));
            canvas.filled_rect(
                state.box_pos.0,
                state.box_pos.1,
                5,
                5,
                &Color::from_rgb(255, 0, 0),
            );
            canvas.render()?;
            Ok(())
        },
    )
}
```

## Architecture

### Game Loop

Pixel Loop implements a fixed timestep game loop that:

- Updates game logic at a constant rate (configurable FPS)
- Renders as fast as possible while maintaining update consistency
- Handles timing and frame limiting automatically

### Canvas System

The library provides (currently) three canvas implementations:

- `PixelsCanvas`: Hardware-accelerated window rendering
- `CrosstermCanvas`: Terminal-based rendering using Unicode characters
- `InMemoryCanvas`: In-memory buffer for image manipulation

Each canvas (currently) supports:

- Basic shape rendering (rectangles)
- Color management (RGB and HSL)
- Efficient blitting operations
- Custom viewport management

### Input System

Input handling is abstracted through traits:

- `KeyboardState` for basic keyboard input
- `InputState` for game loop integration
- Support for key press, release, and hold states
- Cross-platform compatibility

**Note: Mouse integration for window rendering can currently be archieved, but an abstraction has not yet been implemented**

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Inspired by [Fix Your Timestep](https://gafferongames.com/post/fix_your_timestep/)
- Built with [pixels](https://github.com/parasyte/pixels) and [crossterm](https://github.com/crossterm-rs/crossterm)

## Subprojects

This repository housed a couple of different experiment implementations based on
`pixel_loop`. Those have mostly have been moved to their own repositories as
the library is now published on crates.io.

You can find the old subprojects here:

* [pixel_sand](https://github.com/jakobwesthoff/pixel_sand) - A sand movement simulator.
* [tetrotime](https://github.com/jakobwesthoff/tetrotime) - A Tetromino based clock, stopwatch and timer.
* [trivial_cli_demo](examples/trivial_cli_demo/README.md) - A trivial demo showing the CLI/Shell Unicode and ANSI based output driver.
* [shell_smash](https://github.com/jakobwesthofF/shell_smash) - A simple breakout clone running in your Terminal.
* [fireworks](https://github.com/jakobwesthoff/pixel_fireworks) - Fireworks particle simulation in your Terminal
