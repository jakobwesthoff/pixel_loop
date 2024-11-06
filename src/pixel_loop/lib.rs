//! Core game loop and engine functionality.
//!
//! This module provides the main game loop implementation with fixed timestep updates
//! and rendering. It supports different input and canvas implementations through
//! traits.
//!
//! # Example
//! ```
//! use pixel_loop::{run, EngineEnvironment};
//! use pixel_loop::canvas::{Canvas, CrosstermCanvas, RenderableCanvas};
//! use pixel_loop::color::Color;
//! use pixel_loop::input::{CrosstermInputState, KeyboardKey, KeyboardState};
//! use anyhow::Result;
//!
//! // Game state definition
//! struct Box {
//!     position: (i64, i64),
//!     size: (u32, u32),
//!     color: Color,
//! }
//!
//! struct State {
//!     box_entity: Box,
//! }
//!
//! // Create initial state
//! let state = State {
//!     box_entity: Box {
//!         position: (0, 0),
//!         size: (5, 5),
//!         color: Color::from_rgb(156, 80, 182),
//!     },
//! };
//!
//! // Setup canvas and input
//! let canvas = CrosstermCanvas::new(80, 24);
//! let input = CrosstermInputState::new();
//!
//! // Update function - called at fixed timestep
//! fn update(env: &mut EngineEnvironment,
//!           state: &mut State,
//!           input: &CrosstermInputState,
//!           canvas: &mut CrosstermCanvas) -> Result<()> {
//!     // Handle input
//!     if input.is_key_down(KeyboardKey::Up) {
//!         state.box_entity.position.1 -= 1;
//!     }
//!     if input.is_key_down(KeyboardKey::Down) {
//!         state.box_entity.position.1 += 1;
//!     }
//!     Ok(())
//! }
//!
//! // Render function - called as often as possible
//! fn render(env: &mut EngineEnvironment,
//!          state: &mut State,
//!          input: &CrosstermInputState,
//!          canvas: &mut CrosstermCanvas,
//!          dt: std::time::Duration) -> Result<()> {
//!     canvas.clear_screen(&Color::from_rgb(0, 0, 0));
//!     canvas.filled_rect(
//!         state.box_entity.position.0,
//!         state.box_entity.position.1,
//!         state.box_entity.size.0,
//!         state.box_entity.size.1,
//!         &state.box_entity.color,
//!     );
//!     canvas.render()?;
//!     Ok(())
//! }
//!
//! // Run the game loop
//! run(60, state, input, canvas, update, render)?;
//! Ok(())
//! ```

pub mod canvas;
pub mod color;
pub mod input;

// Re-exporting deps for convenience in code using pixel_loop
#[cfg(feature = "crossterm")]
pub use crossterm;
pub use rand;
pub use rand_xoshiro;

use anyhow::Result;
use canvas::RenderableCanvas;
use input::InputState;
use rand::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

/// Function type for the update step of the game loop.
///
/// Called at a fixed timestep to update game state.
///
/// # Arguments
/// * `env` - Global engine environment containing shared resources
/// * `state` - Mutable reference to the game state
/// * `input` - Reference to the current input state
/// * `canvas` - Mutable reference to the rendering canvas
type UpdateFn<State, CanvasImpl> = fn(
    &mut EngineEnvironment,
    &mut State,
    &<CanvasImpl as RenderableCanvas>::Input,
    &mut CanvasImpl,
) -> Result<()>;

/// Function type for the render step of the game loop.
///
/// Called as often as possible with the actual frame time delta.
///
/// # Arguments
/// * `env` - Global engine environment containing shared resources
/// * `state` - Mutable reference to the game state
/// * `input` - Reference to the current input state
/// * `canvas` - Mutable reference to the rendering canvas
/// * `dt` - Time elapsed since last render
type RenderFn<State, CanvasImpl> = fn(
    &mut EngineEnvironment,
    &mut State,
    &<CanvasImpl as RenderableCanvas>::Input,
    &mut CanvasImpl,
    Duration,
) -> Result<()>;

/// Global engine state containing shared resources.
///
/// Provides access to engine-wide functionality and resources that
/// are available to both update and render functions.
pub struct EngineEnvironment {
    /// Random number generator for game logic
    pub rand: Box<dyn rand::RngCore>,
}

impl Default for EngineEnvironment {
    fn default() -> Self {
        let micros = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("If time since UNIX_EPOCH is 0 there is something wrong?")
            .as_micros();
        Self {
            rand: Box::new(Xoshiro256PlusPlus::seed_from_u64(micros as u64)),
        }
    }
}

/// Main game loop handler.
///
/// Manages the game loop timing, state updates, and rendering.
/// Uses a fixed timestep for updates while rendering as fast as possible.
pub struct PixelLoop<State, CanvasImpl: RenderableCanvas> {
    accumulator: Duration,
    current_time: Instant,
    last_time: Instant,
    update_timestep: Duration,
    state: State,
    input_state: CanvasImpl::Input,
    engine_state: EngineEnvironment,
    canvas: CanvasImpl,
    update: UpdateFn<State, CanvasImpl>,
    render: RenderFn<State, CanvasImpl>,
}

impl<State, CanvasImpl> PixelLoop<State, CanvasImpl>
where
    CanvasImpl: RenderableCanvas,
{
    /// Creates a new game loop instance.
    ///
    /// # Arguments
    /// * `update_fps` - Target updates per second for the fixed timestep
    /// * `state` - Initial game state
    /// * `input_state` - Input handling implementation
    /// * `canvas` - Rendering canvas implementation
    /// * `update` - Update function called at fixed timestep
    /// * `render` - Render function called as often as possible
    ///
    /// # Panics
    /// If update_fps is 0
    pub fn new(
        update_fps: usize,
        state: State,
        input_state: CanvasImpl::Input,
        canvas: CanvasImpl,
        update: UpdateFn<State, CanvasImpl>,
        render: RenderFn<State, CanvasImpl>,
    ) -> Self {
        if update_fps == 0 {
            panic!("Designated FPS for updates needs to be > 0");
        }

        Self {
            accumulator: Duration::default(),
            current_time: Instant::now(),
            last_time: Instant::now(),
            update_timestep: Duration::from_nanos(
                (1_000_000_000f64 / update_fps as f64).round() as u64
            ),
            engine_state: EngineEnvironment::default(),
            state,
            input_state,
            canvas,
            update,
            render,
        }
    }

    /// Initializes the game loop.
    pub fn begin(&mut self) -> Result<()> {
        self.input_state.begin()?;
        Ok(())
    }

    /// Processes the next frame of the game loop.
    pub fn next_loop(&mut self) -> Result<()> {
        self.last_time = self.current_time;
        self.current_time = Instant::now();
        let mut dt = self.current_time - self.last_time;

        if dt > Duration::from_millis(100) {
            dt = Duration::from_millis(100);
        }

        while self.accumulator > self.update_timestep {
            (self.input_state).next_loop()?;
            (self.update)(
                &mut self.engine_state,
                &mut self.state,
                &self.input_state,
                &mut self.canvas,
            )?;
            self.accumulator -= self.update_timestep;
        }

        (self.render)(
            &mut self.engine_state,
            &mut self.state,
            &self.input_state,
            &mut self.canvas,
            dt,
        )?;

        self.accumulator += dt;
        Ok(())
    }

    /// Cleans up resources when the game loop ends.
    pub fn finish(&mut self) -> Result<()> {
        self.input_state.finish()?;
        Ok(())
    }
}

/// Runs the game loop with the provided state and implementations.
///
/// # Arguments
/// * `updates_per_second` - Target rate for fixed timestep updates
/// * `state` - Initial game state
/// * `input_state` - Input handling implementation
/// * `canvas` - Rendering canvas implementation
/// * `update` - Update function called at fixed timestep
/// * `render` - Render function called as often as possible
///
/// # Errors
/// Returns an error if initialization fails or if any update/render call fails
pub fn run<State: 'static, CanvasImpl: RenderableCanvas + 'static>(
    updates_per_second: usize,
    state: State,
    input_state: CanvasImpl::Input,
    canvas: CanvasImpl,
    update: UpdateFn<State, CanvasImpl>,
    render: RenderFn<State, CanvasImpl>,
) -> ! {
    CanvasImpl::run(PixelLoop::new(
        updates_per_second,
        state,
        input_state,
        canvas,
        update,
        render,
    ))
}
