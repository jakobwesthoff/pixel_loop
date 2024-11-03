//! Input handling and keyboard state management.
//!
//! This module provides traits and types for handling keyboard input across different
//! platforms. It defines a comprehensive set of keyboard keys and traits for tracking
//! keyboard state and input processing.

#[cfg(feature = "crossterm")]
pub mod crossterm;
#[cfg(feature = "crossterm")]
pub use crossterm::CrosstermInputState;

use anyhow::Result;

/// Represents all possible keyboard keys that can be handled.
///
/// This enum provides a comprehensive list of keyboard keys including:
/// - Alphanumeric keys (A-Z, 0-9)
/// - Special characters (comma, period, etc.)
/// - Function keys (F1-F12)
/// - Navigation keys (arrows, home, end, etc.)
/// - Modifier keys (shift, control, alt, etc.)
/// - Keypad keys
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyboardKey {
    // Alphanumeric keys
    /// Key: '
    Apostrophe,
    /// Key: ,
    Comma,
    /// Key: -
    Minus,
    /// Key: .
    Period,
    /// Key: /
    Slash,
    /// Key: 0
    Zero,
    /// Key: 1
    One,
    /// Key: 2
    Two,
    /// Key: 3
    Three,
    /// Key: 4
    Four,
    /// Key: 5
    Five,
    /// Key: 6
    Six,
    /// Key: 7
    Seven,
    /// Key: 8
    Eight,
    /// Key: 9
    Nine,
    /// Key: ;
    Semicolon,
    /// Key: =
    Equal,
    /// Key: A | a
    A,
    /// Key: B | b
    B,
    /// Key: C | c
    C,
    /// Key: D | d
    D,
    /// Key: E | e
    E,
    /// Key: F | f
    F,
    /// Key: G | g
    G,
    /// Key: H | h
    H,
    /// Key: I | i
    I,
    /// Key: J | j
    J,
    /// Key: K | k
    K,
    /// Key: L | l
    L,
    /// Key: M | m
    M,
    /// Key: N | n
    N,
    /// Key: O | o
    O,
    /// Key: P | p
    P,
    /// Key: Q | q
    Q,
    /// Key: R | r
    R,
    /// Key: S | s
    S,
    /// Key: T | t
    T,
    /// Key: U | u
    U,
    /// Key: V | v
    V,
    /// Key: W | w
    W,
    /// Key: X | x
    X,
    /// Key: Y | y
    Y,
    /// Key: Z | z
    Z,
    /// Key: [
    LeftBracket,
    /// Key: '\'
    Backslash,
    /// Key: ]
    RightBracket,
    /// Key: `
    Grave,

    // Function keys
    /// Key: Space
    Space,
    /// Key: Esc
    Escape,
    /// Key: Enter
    Enter,
    /// Key: Tab
    Tab,
    /// Key: Backspace
    Backspace,
    /// Key: Ins
    Insert,
    /// Key: Del
    Delete,
    /// Key: Cursor right
    Right,
    /// Key: Cursor left
    Left,
    /// Key: Cursor down
    Down,
    /// Key: Cursor up
    Up,
    /// Key: Page up
    PageUp,
    /// Key: Page down
    PageDown,
    /// Key: Home
    Home,
    /// Key: End
    End,
    /// Key: Caps lock
    CapsLock,
    /// Key: Scroll down
    ScrollLock,
    /// Key: Num lock
    NumLock,
    /// Key: Print screen
    PrintScreen,
    /// Key: Pause
    Pause,
    /// Key: F1
    F1,
    /// Key: F2
    F2,
    /// Key: F3
    F3,
    /// Key: F4
    F4,
    /// Key: F5
    F5,
    /// Key: F6
    F6,
    /// Key: F7
    F7,
    /// Key: F8
    F8,
    /// Key: F9
    F9,
    /// Key: F10
    F10,
    /// Key: F11
    F11,
    /// Key: F12
    F12,
    /// Key: Shift left
    LeftShift,
    /// Key: Control left
    LeftControl,
    /// Key: Alt left
    LeftAlt,
    /// Key: Super left
    LeftSuper,
    /// Key: Shift right
    RightShift,
    /// Key: Control right
    RightControl,
    /// Key: Alt right
    RightAlt,
    /// Key: Super right
    RightSuper,
    /// Key: KB menu
    KbMenu,

    // Keypad keys
    /// Key: Keypad 0
    Kp0,
    /// Key: Keypad 1
    Kp1,
    /// Key: Keypad 2
    Kp2,
    /// Key: Keypad 3
    Kp3,
    /// Key: Keypad 4
    Kp4,
    /// Key: Keypad 5
    Kp5,
    /// Key: Keypad 6
    Kp6,
    /// Key: Keypad 7
    Kp7,
    /// Key: Keypad 8
    Kp8,
    /// Key: Keypad 9
    Kp9,
    /// Key: Keypad .
    KpDecimal,
    /// Key: Keypad /
    KpDivide,
    /// Key: Keypad *
    KpMultiply,
    /// Key: Keypad -
    KpSubtract,
    /// Key: Keypad +
    KpAdd,
    /// Key: Keypad Enter
    KpEnter,
    /// Key: Keypad =
    KpEqual,
}

/// Trait for tracking keyboard state.
///
/// This trait provides methods for checking the current state of keyboard keys,
/// including whether they were just pressed, are being held down, were just
/// released, or are currently up.
pub trait KeyboardState {
    /// Checks if a key was pressed this frame.
    ///
    /// # Arguments
    /// * `key` - The key to check
    fn is_key_pressed(&self, key: KeyboardKey) -> bool;

    /// Checks if a key is currently being held down.
    ///
    /// # Arguments
    /// * `key` - The key to check
    fn is_key_down(&self, key: KeyboardKey) -> bool;

    /// Checks if a key was released this frame.
    ///
    /// # Arguments
    /// * `key` - The key to check
    fn is_key_released(&self, key: KeyboardKey) -> bool;

    /// Checks if a key is currently up (not being pressed).
    ///
    /// # Arguments
    /// * `key` - The key to check
    fn is_key_up(&self, key: KeyboardKey) -> bool;
}

/// Trait for managing input state in a game loop.
///
/// This trait extends `KeyboardState` and provides methods for managing input
/// state throughout the lifecycle of a game loop.
///
/// Its methods provide a way for different platform implementations to hook
/// into the game loop cycle to handle input event processing.
pub trait InputState: KeyboardState {
    /// Initializes the input state before starting a loop.
    ///
    /// This method is called once before entering the main loop.
    fn begin(&mut self) -> Result<()>;

    /// Updates the input state for the next frame.
    ///
    /// This method is called at the beginning of each loop iteration, before the
    /// update function is invoked.
    fn next_loop(&mut self) -> Result<()>;

    /// Finalizes the input state after the loop ends.
    ///
    /// This method is called once after exiting the main loop.
    fn finish(&mut self) -> Result<()>;
}
