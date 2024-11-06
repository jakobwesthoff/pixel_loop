//! Noop input state implementation.
//!
//! This implementation does nothing, and is useful for testing or for games that
//! don't require input handling.

use super::{InputState, KeyboardKey, KeyboardState};

/// Noop input state implementation.
///
/// This implementation does nothing, and is useful for testing or for games that
/// don't require input handling.
pub struct NoopInputState {}

impl Default for NoopInputState {
    fn default() -> Self {
        Self::new()
    }
}

impl NoopInputState {
    /// Create a new `NoopInputState`.
    pub fn new() -> Self {
        Self {}
    }
}

impl InputState for NoopInputState {
    fn begin(&mut self) -> anyhow::Result<()> {
        // Noop
        Ok(())
    }

    fn next_loop(&mut self) -> anyhow::Result<()> {
        // Noop
        Ok(())
    }

    fn finish(&mut self) -> anyhow::Result<()> {
        // Noop
        Ok(())
    }
}

impl KeyboardState for NoopInputState {
    fn is_key_pressed(&self, _key: KeyboardKey) -> bool {
        false
    }

    fn is_key_down(&self, _key: KeyboardKey) -> bool {
        false
    }

    fn is_key_released(&self, _key: KeyboardKey) -> bool {
        true
    }

    fn is_key_up(&self, _key: KeyboardKey) -> bool {
        true
    }
}
