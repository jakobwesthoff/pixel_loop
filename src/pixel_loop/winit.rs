use super::{EngineEnvironment, PixelLoop, PixelsCanvas, RenderFn, UpdateFn};
use anyhow::{Context, Result};
use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};
use winit_input_helper::WinitInputHelper;

type WinitEventFn<State, CanvasImpl> = fn(
    &mut EngineEnvironment,
    &mut State,
    &mut CanvasImpl,
    &Window,
    &mut WinitInputHelper,
    event: &Event<()>,
) -> Result<()>;

pub struct WinitContext {
    event_loop: EventLoop<()>,
    input_helper: WinitInputHelper,
    window: Window,
}

impl WinitContext {
    pub fn window_ref(&self) -> &Window {
        &self.window
    }

    pub fn input_helper_ref(&self) -> &WinitInputHelper {
        &self.input_helper
    }
}

pub fn init_window(
    title: &str,
    min_width: u32,
    min_height: u32,
    resizable: bool,
) -> Result<WinitContext> {
    let event_loop = EventLoop::new();
    let input_helper = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(min_width as f64, min_height as f64);
        WindowBuilder::new()
            .with_title(title)
            .with_inner_size(size)
            .with_min_inner_size(size)
            .with_resizable(resizable)
            .build(&event_loop)?
    };

    Ok(WinitContext { event_loop, input_helper, window })
}

pub fn init_pixels(context: &WinitContext, width: u32, height: u32) -> Result<PixelsCanvas> {
    let physical_dimensions = context.window_ref().inner_size();
    let surface_texture = SurfaceTexture::new(
        physical_dimensions.width,
        physical_dimensions.height,
        context.window_ref(),
    );
    let pixels = Pixels::new(width, height, surface_texture).context("create pixels surface")?;
    Ok(PixelsCanvas::new(pixels))
}

pub fn run<State: 'static>(
    state: State,
    mut context: WinitContext,
    canvas: PixelsCanvas,
    update: UpdateFn<State, PixelsCanvas>,
    render: RenderFn<State, PixelsCanvas>,
    handle_event: WinitEventFn<State, PixelsCanvas>,
) -> ! {
    let mut pixel_loop = PixelLoop::new(120, state, canvas, update, render);

    context.event_loop.run(move |event, _, control_flow| {
        handle_event(
            &mut pixel_loop.engine_state,
            &mut pixel_loop.state,
            &mut pixel_loop.canvas,
            &context.window,
            &mut context.input_helper,
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
            },
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
