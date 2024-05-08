use anyhow::{Context, Result};
use pixels::{Pixels, SurfaceTexture};
use std::ops::Range;
use std::time::{Duration, Instant};
use tao::dpi::LogicalSize;
use tao::event::{Event, WindowEvent};
use tao::event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget};
use tao::window::{Window, WindowBuilder};

type UpdateFn<State, CanvasImpl> = fn(&mut State, &mut CanvasImpl) -> Result<()>;
type RenderFn<State, CanvasImpl> = fn(&mut State, &mut CanvasImpl, Duration) -> Result<()>;
type TaoEventFn<State, CanvasImpl> =
    fn(&mut State, &mut CanvasImpl, &EventLoopWindowTarget<()>, event: &Event<()>) -> Result<()>;

struct PixelLoop<State, CanvasImpl: Canvas> {
    accumulator: Duration,
    current_time: Instant,
    last_time: Instant,
    update_timestep: Duration,
    state: State,
    canvas: CanvasImpl,
    update: UpdateFn<State, CanvasImpl>,
    render: RenderFn<State, CanvasImpl>,
}

impl<State, CanvasImpl> PixelLoop<State, CanvasImpl>
where
    CanvasImpl: Canvas,
{
    pub fn new(
        update_fps: usize,
        state: State,
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
            state,
            canvas,
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
            (self.update)(&mut self.state, &mut self.canvas)?;
            self.accumulator -= self.update_timestep;
        }

        (self.render)(&mut self.state, &mut self.canvas, dt)?;

        self.accumulator += dt;
        Ok(())
    }
}

pub fn run<State, CanvasImpl: Canvas>(
    state: State,
    canvas: CanvasImpl,
    update: UpdateFn<State, CanvasImpl>,
    render: RenderFn<State, CanvasImpl>,
) -> Result<()> {
    let mut pixel_loop = PixelLoop::new(120, state, canvas, update, render);
    loop {
        pixel_loop.next_loop().context("run next pixel loop")?;
    }
}

#[repr(C)]
#[derive(Clone, PartialEq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

trait ColorAsByteSlice {
    fn as_byte_slice(&self) -> &[u8];
}

impl ColorAsByteSlice for [Color] {
    fn as_byte_slice(&self) -> &[u8] {
        let byte_slice = unsafe {
            std::slice::from_raw_parts(
                self.as_ptr() as *const u8,
                std::mem::size_of::<Color>() * self.len(),
            )
        };
        byte_slice
    }
}

impl Color {
    pub fn from_bytes(bytes: &[u8]) -> &[Self] {
        if bytes.len() % std::mem::size_of::<Color>() != 0 {
            panic!("Color slices can only be initialized with a multiple of 4 byte slices");
        }

        let color_slice = unsafe {
            if bytes.as_ptr() as usize % std::mem::align_of::<Color>() != 0 {
                panic!(
                    "alignment of color byte slice must be fitting for alignment of Color struct"
                )
            }

            std::slice::from_raw_parts(
                bytes.as_ptr() as *const Color,
                bytes.len() / std::mem::size_of::<Color>(),
            )
        };

        color_slice
    }

    pub fn from_rgba(r: u8, b: u8, g: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
    pub fn from_rgb(r: u8, b: u8, g: u8) -> Self {
        Self::from_rgba(r, g, b, 255)
    }

    pub fn as_bytes(&self) -> &[u8] {
        let color_slice = std::slice::from_ref(self);
        let byte_slice = unsafe {
            std::slice::from_raw_parts(
                color_slice.as_ptr() as *const u8,
                std::mem::size_of::<Color>(),
            )
        };
        byte_slice
    }
}

pub trait Canvas {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn blit(&mut self) -> Result<()>;
    fn set_range(&mut self, range: Range<usize>, color: &[Color]);
    fn get_range(&self, range: Range<usize>) -> &[Color];
    fn in_bounds(&self, x: i64, y: i64) -> Option<(u32, u32)>;
    fn physical_pos_to_canvas_pos(&self, x: f64, y: f64) -> Option<(u32, u32)>;

    fn get(&self, x: u32, y: u32) -> &Color {
        let i = (y * self.width() + x) as usize;
        let color_slice = self.get_range(i..i + 1);
        // @TODO: Check if clone happens here
        &color_slice[0]
    }

    fn set(&mut self, x: u32, y: u32, color: &Color) {
        let i = (y * self.width() + x) as usize;
        self.set_range(i..i + 1, std::slice::from_ref(color));
    }

    fn clear_screen(&mut self, color: &Color) {
        self.filled_rect(0, 0, self.width(), self.height(), color)
    }

    fn filled_rect(&mut self, sx: u32, sy: u32, width: u32, height: u32, color: &Color) {
        let color_row = vec![color.clone(); width as usize];
        for y in sy..sy + height {
            self.set_range(
                (y * self.width() + sx) as usize..(y * self.width() + sx + width) as usize,
                color_row.as_slice(),
            );
        }
    }
}

pub struct PixelsCanvas {
    pixels: Pixels,
}

impl PixelsCanvas {
    pub fn new(pixels: Pixels) -> Self {
        Self { pixels }
    }
}

impl Canvas for PixelsCanvas {
    fn width(&self) -> u32 {
        self.pixels.texture().width()
    }
    fn height(&self) -> u32 {
        self.pixels.texture().height()
    }

    fn blit(&mut self) -> Result<()> {
        self.pixels
            .render()
            .context("letting pixels lib blit to screen")?;
        Ok(())
    }

    fn get_range(&self, range: Range<usize>) -> &[Color] {
        let byte_range = range.start * 4..range.end * 4;
        let buf = self.pixels.frame();
        let byte_slice = &buf[byte_range];
        Color::from_bytes(byte_slice)
    }

    fn set_range(&mut self, range: Range<usize>, colors: &[Color]) {
        let byte_range = range.start * 4..range.end * 4;
        let buf = self.pixels.frame_mut();
        buf[byte_range].copy_from_slice(colors.as_byte_slice())
    }

    fn in_bounds(&self, x: i64, y: i64) -> Option<(u32, u32)> {
        if x < 0 || x >= self.width() as i64 || y < 0 || y >= self.height() as i64 {
            None
        } else {
            Some((x as u32, y as u32))
        }
    }

    fn physical_pos_to_canvas_pos(&self, x: f64, y: f64) -> Option<(u32, u32)> {
        if let Ok((x, y)) = self.pixels.window_pos_to_pixel((x as f32, y as f32)) {
            Some((x as u32, y as u32))
        } else {
            None
        }
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

pub fn run_with_tao_and_pixels<State: 'static>(
    state: State,
    context: TaoContext,
    canvas: PixelsCanvas,
    update: UpdateFn<State, PixelsCanvas>,
    render: RenderFn<State, PixelsCanvas>,
    handle_event: TaoEventFn<State, PixelsCanvas>,
) -> ! {
    let mut pixel_loop = PixelLoop::new(120, state, canvas, update, render);
    context.event_loop.run(move |event, window, control_flow| {
        handle_event(
            &mut pixel_loop.state,
            &mut pixel_loop.canvas,
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
