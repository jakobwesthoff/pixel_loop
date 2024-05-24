use super::{EngineEnvironment, PixelLoop, PixelsCanvas, RenderFn, UpdateFn};
use anyhow::{Context, Result};
use pixels::{Pixels, SurfaceTexture};
use tao::dpi::LogicalSize;
use tao::event::{Event, WindowEvent};
use tao::event_loop::{ControlFlow, EventLoop};
use tao::window::{Window, WindowBuilder};

type TaoEventFn<State, CanvasImpl> = fn(
    &mut EngineEnvironment,
    &mut State,
    &mut CanvasImpl,
    &Window,
    event: &Event<()>,
) -> Result<()>;

pub struct TaoContext {
    event_loop: EventLoop<()>,
    window: Window,
}

impl TaoContext {
    pub fn as_window(&self) -> &Window {
        &self.window
    }
}

pub fn init_window(
    title: &str,
    min_width: u32,
    min_height: u32,
    resizable: bool,
) -> Result<TaoContext> {
    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(min_width, min_height);
        WindowBuilder::new()
            .with_title(title)
            .with_inner_size(size)
            .with_min_inner_size(size)
            .with_resizable(resizable)
            .build(&event_loop)?
    };

    Ok(TaoContext { event_loop, window })
}

pub fn init_pixels(context: &TaoContext, width: u32, height: u32) -> Result<PixelsCanvas> {
    let physical_dimensions = context.as_window().inner_size();
    let surface_texture = SurfaceTexture::new(
        physical_dimensions.width,
        physical_dimensions.height,
        context.as_window(),
    );
    let pixels = Pixels::new(width, height, surface_texture).context("create pixels surface")?;
    Ok(PixelsCanvas::new(pixels))
}

pub fn run<State: 'static>(
    state: State,
    context: TaoContext,
    canvas: PixelsCanvas,
    update: UpdateFn<State, PixelsCanvas>,
    render: RenderFn<State, PixelsCanvas>,
    handle_event: TaoEventFn<State, PixelsCanvas>,
) -> ! {
    let mut pixel_loop = PixelLoop::new(120, state, canvas, update, render);
    context.event_loop.run(move |event, _, control_flow| {
        handle_event(
            &mut pixel_loop.engine_state,
            &mut pixel_loop.state,
            &mut pixel_loop.canvas,
            &context.window,
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
