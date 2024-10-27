//@TODO: Make conditional compiled based on feature
//pub mod tao;
pub mod canvas;
pub mod input;
pub mod winit;

use anyhow::{Context, Result};
use input::InputState;
use rand::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;
use std::ops::Range;
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

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
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

    pub const fn from_rgba(r: u8, b: u8, g: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
    pub const fn from_rgb(r: u8, b: u8, g: u8) -> Self {
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

    pub fn as_hsl(&self) -> HslColor {
        // Taken and converted from: https://stackoverflow.com/a/9493060
        let r = self.r as f64 / 255.0;
        let g = self.g as f64 / 255.0;
        let b = self.b as f64 / 255.0;
        let vmax = r.max(g.max(b));
        let vmin = r.min(g.min(b));
        let l = (vmax + vmin) / 2.0;

        if vmax == vmin {
            return HslColor::new(0.0, 0.0, l); // achromatic
        }

        let d = vmax - vmin;
        let s = if l > 0.5 {
            d / (2.0 - vmax - vmin)
        } else {
            d / (vmax + vmin)
        };

        let mut h = (vmax + vmin) / 2.0;

        if vmax == r {
            h = (g - b) / d;
            if g < b {
                h += 6.0
            }
        }

        if vmax == g {
            h = (b - r) / d + 2.0;
        }

        if vmax == b {
            h = (r - g) / d + 4.0;
        }

        h /= 6.0;

        // The color conversion moves every value into the [0,1] number space.
        // But we want the hue in [0,360], s in [0,100] and l in [0,100]
        HslColor::new(h * 360f64, s * 100f64, l * 100f64)
    }
}

impl From<HslColor> for Color {
    fn from(v: HslColor) -> Self {
        // Taken and converted from: https://stackoverflow.com/a/9493060

        fn hue_to_rgb(p: f64, q: f64, t: f64) -> f64 {
            let mut t = t;
            if t < 0f64 {
                t += 1f64
            };
            if t > 1f64 {
                t -= 1f64
            };
            if t < 1f64 / 6f64 {
                return p + (q - p) * 6f64 * t;
            }
            if t < 1f64 / 2f64 {
                return q;
            }
            if t < 2f64 / 3f64 {
                return p + (q - p) * (2f64 / 3f64 - t) * 6f64;
            };
            return p;
        }

        let r;
        let g;
        let b;

        // The input for this algorithm expects all the h,s and l values in the
        // range [0,1].
        let h = v.h / 360f64;
        let s = v.s / 100f64;
        let l = v.l / 100f64;

        if s == 0.0 {
            r = l;
            g = l;
            b = l;
        } else {
            let q = if l < 0.5 {
                l * (1.0 + s)
            } else {
                l + s - l * s
            };
            let p = 2.0 * l - q;

            r = hue_to_rgb(p, q, h + 1f64 / 3f64);
            g = hue_to_rgb(p, q, h);
            b = hue_to_rgb(p, q, h - 1f64 / 3f64);
        }
        Color::from_rgb(
            (r * 255.0).round() as u8,
            (g * 255.0).round() as u8,
            (b * 255.0).round() as u8,
        )
    }
}

pub struct HslColor {
    pub h: f64, // Hue in [0,360]
    pub s: f64, // Saturation in [0,100]
    pub l: f64, // Lightness in [0,100]
}

impl HslColor {
    pub fn new(h: f64, s: f64, l: f64) -> Self {
        Self { h, s, l }
    }
}

pub trait Canvas {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn set_range(&mut self, range: Range<usize>, color: &[Color]);
    fn get_range(&self, range: Range<usize>) -> &[Color];

    fn blit<C: Canvas>(&mut self, src_canvas: &C, dst_x: i64, dst_y: i64, tint: Option<&Color>) {
        self.blit_rect(
            src_canvas,
            0,
            0,
            src_canvas.width(),
            src_canvas.height(),
            dst_x,
            dst_y,
            tint,
        )
    }

    fn blit_rect<C: Canvas>(
        &mut self,
        src_canvas: &C,
        src_x: u32,
        src_y: u32,
        width: u32,
        height: u32,
        dst_x: i64,
        dst_y: i64,
        tint: Option<&Color>,
    ) {
        if let Some((norm_dst_x, norm_dst_y, norm_width, norm_height)) =
            self.normalize_rect(dst_x, dst_y, width, height)
        {
            for y in 0..norm_height {
                let src_start = (((src_y + y) * src_canvas.width()) + src_x) as usize;
                let src_end = src_start + u32::min(width, norm_width) as usize;
                let dst_start = (((norm_dst_y + y) * self.width()) + norm_dst_x) as usize;
                let dst_end = dst_start + norm_width as usize;
                let row = src_canvas.get_range(src_start..src_end);

                if let Some(tint) = tint {
                    self.set_range(
                        dst_start..dst_end,
                        &row.iter()
                            .map(|c| {
                                Color::from_rgb(
                                    (c.r as usize * tint.r as usize / 255 as usize) as u8,
                                    (c.g as usize * tint.g as usize / 255 as usize) as u8,
                                    (c.b as usize * tint.b as usize / 255 as usize) as u8,
                                )
                            })
                            .collect::<Vec<Color>>(),
                    );
                } else {
                    self.set_range(dst_start..dst_end, row);
                }
            }
        }
    }

    fn get(&self, x: u32, y: u32) -> &Color {
        let i = (y * self.width() + x) as usize;
        let color_slice = self.get_range(i..i + 1);
        // @TODO: Check if clone happens here
        &color_slice[0]
    }

    fn maybe_get(&self, x: i64, y: i64) -> Option<&Color> {
        if x < 0 || y < 0 || x >= self.width() as i64 || y >= self.height() as i64 {
            // Out of view
            None
        } else {
            Some(self.get(x as u32, y as u32))
        }
    }

    // @TODO: Not ideal: It would be better if the canvas knew its "empty"
    // (clear) color and could be asked `is_empty`.
    fn is_empty_or_color(&self, x: i64, y: i64, color: &Color) -> bool {
        self.maybe_get(x, y).map(|c| c == color).unwrap_or(true)
    }

    fn set(&mut self, x: u32, y: u32, color: &Color) {
        let i = (y * self.width() + x) as usize;
        self.set_range(i..i + 1, std::slice::from_ref(color));
    }

    fn normalize_rect(
        &self,
        x: i64,
        y: i64,
        width: u32,
        height: u32,
    ) -> Option<(u32, u32, u32, u32)> {
        let width = width as i64;
        let height = height as i64;
        if x < -width || y < -height || x >= self.width() as i64 || y >= self.height() as i64 {
            // Completely out of view
            None
        } else {
            let norm_x = i64::max(0, x);
            let norm_y = i64::max(0, y);
            let norm_width = i64::min(width - (norm_x - x), self.width() as i64 - norm_x - 1);
            let norm_height = i64::min(height - (norm_y - y), self.height() as i64 - norm_y - 1);
            Some((
                norm_x as u32,
                norm_y as u32,
                norm_width as u32,
                norm_height as u32,
            ))
        }
    }

    fn clear_screen(&mut self, color: &Color) {
        self.filled_rect(0, 0, self.width(), self.height(), color)
    }

    fn filled_rect(&mut self, sx: i64, sy: i64, width: u32, height: u32, color: &Color) {
        if let Some((sx, sy, width, height)) = self.normalize_rect(sx, sy, width, height) {
            let color_row = vec![color.clone(); width as usize];
            for y in sy..sy + height {
                self.set_range(
                    (y * self.width() + sx) as usize..(y * self.width() + sx + width) as usize,
                    color_row.as_slice(),
                );
            }
        }
    }
}

pub trait RenderableCanvas: Canvas {
    fn render(&mut self) -> Result<()>;
    fn physical_pos_to_canvas_pos(&self, x: f64, y: f64) -> Option<(u32, u32)>;
    fn resize_surface(&mut self, width: u32, height: u32);
    fn resize(&mut self, width: u32, height: u32);
}
