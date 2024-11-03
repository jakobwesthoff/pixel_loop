//@TODO: Export tao initialization as well?
//pub mod tao;
pub mod canvas;
pub mod color;
pub mod input;

#[cfg(feature = "winit")]
pub mod winit;

use anyhow::{Context, Result};
use canvas::RenderableCanvas;
use color::*;
use input::InputState;
use rand::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

type UpdateFn<State, InputStateImpl, CanvasImpl> =
    fn(&mut EngineEnvironment, &mut State, &InputStateImpl, &mut CanvasImpl) -> Result<()>;
type RenderFn<State, InputStateImpl, CanvasImpl> = fn(
    &mut EngineEnvironment,
    &mut State,
    &InputStateImpl,
    &mut CanvasImpl,
    Duration,
) -> Result<()>;

pub struct EngineEnvironment {
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

struct PixelLoop<State, InputStateImpl: InputState, CanvasImpl: RenderableCanvas> {
    accumulator: Duration,
    current_time: Instant,
    last_time: Instant,
    update_timestep: Duration,
    state: State,
    input_state: InputStateImpl,
    engine_state: EngineEnvironment,
    canvas: CanvasImpl,
    update: UpdateFn<State, InputStateImpl, CanvasImpl>,
    render: RenderFn<State, InputStateImpl, CanvasImpl>,
}

impl<State, InputStateImpl, CanvasImpl> PixelLoop<State, InputStateImpl, CanvasImpl>
where
    CanvasImpl: RenderableCanvas,
    InputStateImpl: InputState,
{
    pub fn new(
        update_fps: usize,
        state: State,
        input_state: InputStateImpl,
        canvas: CanvasImpl,
        update: UpdateFn<State, InputStateImpl, CanvasImpl>,
        render: RenderFn<State, InputStateImpl, CanvasImpl>,
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

    pub fn begin(&mut self) -> Result<()> {
        self.input_state.begin()?;
        Ok(())
    }

    // Inpsired by: https://gafferongames.com/post/fix_your_timestep/
    pub fn next_loop(&mut self) -> Result<()> {
        self.last_time = self.current_time;
        self.current_time = Instant::now();
        let mut dt = self.current_time - self.last_time;

        // Escape hatch if update calls take to long in order to not spiral into
        // death
        // @FIXME: It may be useful to make this configurable?
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

    pub fn finish(&mut self) -> Result<()> {
        self.input_state.finish()?;
        Ok(())
    }
}

pub fn run<State, InputStateImpl: InputState, CanvasImpl: RenderableCanvas>(
    updates_per_second: usize,
    state: State,
    input_state: InputStateImpl,
    canvas: CanvasImpl,
    update: UpdateFn<State, InputStateImpl, CanvasImpl>,
    render: RenderFn<State, InputStateImpl, CanvasImpl>,
) -> Result<()> {
    let mut pixel_loop = PixelLoop::new(
        updates_per_second,
        state,
        input_state,
        canvas,
        update,
        render,
    );

    pixel_loop.begin()?;
    loop {
        pixel_loop.next_loop().context("run next pixel loop")?;
    }
    // @TODO: Allow pixel_loop loop to end properly, to be able to reach this code.
    // pixel_loop.finish()?;
}
