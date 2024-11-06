//! Window-based game loop implementation using winit and pixels.
//!
//! This module provides window creation and management for desktop applications
//! using the winit windowing library. It is only available when the "winit"
//! feature is enabled.
//!
//! # @TODO
//! This module needs to be heavily refactored to utilize the [InputState] trait
//! instead of providing its own input handling callback.  It has been created
//! at a time, where the [InputState] trait was not yet implemented. Furthermore
//! the [InputState] trait should be adapted to the feature set needed to
//! properly handle all the needed winput events.
//!
//! # Warning
//!
//! Due to the mentioned TODO above the interface of this module is going to
//! change heavily in the future.
//!
//! # Example
//! ```
//! use pixel_loop::winit::{self, WinitContext};
//! use pixel_loop::EngineEnvironment;
//! use winit::event::Event;
//! use winit::window::Window;
//! use winit_input_helper::WinitInputHelper;
//! use anyhow::Result;
//!
//! struct GameState {
//!     score: i32,
//! }
//!
//! // Initialize window and pixels
//! let context = winit::init_window("My Game", 640, 480, true)?;
//! let canvas = winit::init_pixels(&context, 640, 480)?;
//! let input = WinitInputHelper::new();
//! let state = GameState { score: 0 };
//!
//! // Handle window events
//! fn handle_event(
//!     env: &mut EngineEnvironment,
//!     state: &mut GameState,
//!     canvas: &mut pixel_loop::canvas::PixelsCanvas,
//!     window: &Window,
//!     input: &mut WinitInputHelper,
//!     event: &Event<()>
//! ) -> Result<()> {
//!     // Handle window resizing
//!     if input.window_resized() {
//!         let size = window.inner_size();
//!         canvas.resize_surface(size.width, size.height);
//!     }
//!     Ok(())
//! }
//!
//! // Run the game loop
//! winit::run(
//!     60,
//!     state,
//!     input,
//!     context,
//!     canvas,
//!     |env, state, input, canvas| {
//!         // Update game state
//!         Ok(())
//!     },
//!     |env, state, input, canvas, dt| {
//!         // Render game state
//!         canvas.render()?;
//!         Ok(())
//!     },
//!     handle_event,
//! );
//! ```

// Re-export winit and pixels for convenience
pub use pixels;
pub use winit;

use super::{EngineEnvironment, PixelLoop, RenderFn, UpdateFn};
use crate::canvas::PixelsCanvas;
use crate::input::InputState;
use anyhow::{Context, Result};
use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};
use winit_input_helper::WinitInputHelper;

/// Function type for handling window events.
///
/// Called for each window event before it is processed by the game loop.
///
/// # Arguments
/// * `env` - Global engine environment
/// * `state` - Game state
/// * `canvas` - Rendering canvas
/// * `window` - Window reference
/// * `input` - Winit input helper
/// * `event` - Current window event
type WinitEventFn<State, CanvasImpl> = fn(
    &mut EngineEnvironment,
    &mut State,
    &mut CanvasImpl,
    &Window,
    &mut WinitInputHelper,
    event: &Event<()>,
) -> Result<()>;

/// Context holding window-related resources.
pub struct WinitContext {
    pub (crate) event_loop: EventLoop<()>,
    input_helper: WinitInputHelper,
    window: Window,
}

impl WinitContext {
    /// Returns a reference to the window.
    pub fn window_ref(&self) -> &Window {
        &self.window
    }

    /// Returns a reference to the input helper.
    pub fn input_helper_ref(&self) -> &WinitInputHelper {
        &self.input_helper
    }
}

/// Initializes a new window with the specified parameters.
///
/// # Arguments
/// * `title` - Window title
/// * `min_width` - Minimum window width in pixels
/// * `min_height` - Minimum window height in pixels
/// * `resizable` - Whether the window can be resized
///
/// # Returns
/// A WinitContext containing the window and related resources
///
/// # Example
/// ```
/// use pixel_loop::winit;
///
/// let context = winit::init_window("My Game", 640, 480, true)?;
/// ```
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

    Ok(WinitContext {
        event_loop,
        input_helper,
        window,
    })
}

/// Initializes a new pixels canvas for the given window context.
///
/// # Arguments
/// * `context` - Window context to create the canvas for
/// * `width` - Canvas width in pixels
/// * `height` - Canvas height in pixels
///
/// # Returns
/// A new PixelsCanvas ready for rendering
///
/// # Example
/// ```
/// use pixel_loop::winit;
///
/// let context = winit::init_window("My Game", 640, 480, true)?;
/// let canvas = winit::init_pixels(&context, 640, 480)?;
/// ```
pub fn init_pixels(context: WinitContext, width: u32, height: u32) -> Result<PixelsCanvas> {
    let physical_dimensions = context.window_ref().inner_size();
    let surface_texture = SurfaceTexture::new(
        physical_dimensions.width,
        physical_dimensions.height,
        context.window_ref(),
    );
    let pixels = Pixels::new(width, height, surface_texture).context("create pixels surface")?;
    Ok(PixelsCanvas::new(context, pixels))
}
