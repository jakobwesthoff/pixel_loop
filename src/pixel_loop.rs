use anyhow::{Context, Result};
use pixels::{Pixels, SurfaceTexture};
use std::time::{Duration, Instant};
use tao::dpi::LogicalSize;
use tao::event::{Event, WindowEvent};
use tao::event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget};
use tao::window::{Window, WindowBuilder};

type UpdateFn<State, Surface> = fn(&mut State, &mut Surface) -> Result<()>;
type RenderFn<State, Surface> = fn(&mut State, &mut Surface, Duration) -> Result<()>;
type TaoEventFn<State, Surface> =
    fn(&mut State, &mut Surface, &EventLoopWindowTarget<()>, event: &Event<()>) -> Result<()>;

struct PixelLoop<State, Surface> {
    accumulator: Duration,
    current_time: Instant,
    last_time: Instant,
    update_timestep: Duration,
    state: State,
    surface: Surface,
    update: UpdateFn<State, Surface>,
    render: RenderFn<State, Surface>,
}

impl<State, Surface> PixelLoop<State, Surface> {
    pub fn new(
        update_fps: usize,
        state: State,
        surface: Surface,
        update: UpdateFn<State, Surface>,
        render: RenderFn<State, Surface>,
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
            state,
            surface,
            update,
            render,
        }
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
            (self.update)(&mut self.state, &mut self.surface)?;
            self.accumulator -= self.update_timestep;
        }

        (self.render)(&mut self.state, &mut self.surface, dt)?;

        self.accumulator += dt;
        Ok(())
    }
}

pub fn run<State, Surface>(
    state: State,
    surface: Surface,
    update: UpdateFn<State, Surface>,
    render: RenderFn<State, Surface>,
) -> Result<()> {
    let mut pixel_loop = PixelLoop::new(120, state, surface, update, render);
    loop {
        pixel_loop.next_loop().context("run next pixel loop")?;
    }
}

pub struct PixelsSurface {
    pixels: Pixels,
}

impl PixelsSurface {
    pub fn new(pixels: Pixels) -> Self {
        Self { pixels }
    }

    pub fn width(&self) -> u32 {
        self.pixels.texture().width()
    }
    pub fn height(&self) -> u32 {
        self.pixels.texture().height()
    }
    pub fn frame_mut(&mut self) -> &mut [u8] {
        self.pixels.frame_mut()
    }

    pub fn render(&mut self) -> Result<()> {
        self.pixels
            .render()
            .context("letting pixels lib blit to screen")?;
        Ok(())
    }

    pub fn pixels(&self) -> &Pixels {
        &self.pixels
    }
}

pub struct TaoContext {
    event_loop: EventLoop<()>,
    window: Window,
}

impl TaoContext {
    pub fn as_window(&self) -> &Window {
        &self.window
    }
}

pub fn init_tao_window(title: &str, width: u32, height: u32) -> Result<TaoContext> {
    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(width, height);
        WindowBuilder::new()
            .with_title(title)
            .with_inner_size(size)
            .with_min_inner_size(size)
            .with_resizable(false)
            .build(&event_loop)?
    };

    Ok(TaoContext { event_loop, window })
}

pub fn init_pixels(context: &TaoContext, width: u32, height: u32) -> Result<PixelsSurface> {
    let physical_dimensions = context.as_window().inner_size();
    let surface_texture = SurfaceTexture::new(
        physical_dimensions.width,
        physical_dimensions.height,
        context.as_window(),
    );
    let pixels = Pixels::new(width, height, surface_texture).context("create pixels surface")?;
    Ok(PixelsSurface::new(pixels))
}

pub fn run_with_tao_and_pixels<State: 'static>(
    state: State,
    context: TaoContext,
    surface: PixelsSurface,
    update: UpdateFn<State, PixelsSurface>,
    render: RenderFn<State, PixelsSurface>,
    handle_event: TaoEventFn<State, PixelsSurface>,
) -> ! {
    let mut pixel_loop = PixelLoop::new(120, state, surface, update, render);
    context.event_loop.run(move |event, window, control_flow| {
        handle_event(
            &mut pixel_loop.state,
            &mut pixel_loop.surface,
            window,
            &event,
        )
        .context("handle user events")
        .unwrap();
        match event {
            Event::MainEventsCleared => {
                pixel_loop
                    .next_loop()
                    .context("run next pixel loop")
                    .unwrap();
            }
            Event::WindowEvent {
                event: win_event, ..
            } => match win_event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                _ => {}
            },

            _ => {}
        }
    });
}
