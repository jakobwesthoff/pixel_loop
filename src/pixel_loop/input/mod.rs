pub mod crossterm;

pub use crossterm::CrosstermInputState;

use anyhow::Result;

// The basic key list is taken from the raylib keys list
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyboardKey {
    // Alphanumeric keys
    Apostrophe,   // Key: '
    Comma,        // Key: ,
    Minus,        // Key: -
    Period,       // Key: .
    Slash,        // Key: /
    Zero,         // Key: 0
    One,          // Key: 1
    Two,          // Key: 2
    Three,        // Key: 3
    Four,         // Key: 4
    Five,         // Key: 5
    Six,          // Key: 6
    Seven,        // Key: 7
    Eight,        // Key: 8
    Nine,         // Key: 9
    Semicolon,    // Key: ;
    Equal,        // Key: =
    A,            // Key: A | a
    B,            // Key: B | b
    C,            // Key: C | c
    D,            // Key: D | d
    E,            // Key: E | e
    F,            // Key: F | f
    G,            // Key: G | g
    H,            // Key: H | h
    I,            // Key: I | i
    J,            // Key: J | j
    K,            // Key: K | k
    L,            // Key: L | l
    M,            // Key: M | m
    N,            // Key: N | n
    O,            // Key: O | o
    P,            // Key: P | p
    Q,            // Key: Q | q
    R,            // Key: R | r
    S,            // Key: S | s
    T,            // Key: T | t
    U,            // Key: U | u
    V,            // Key: V | v
    W,            // Key: W | w
    X,            // Key: X | x
    Y,            // Key: Y | y
    Z,            // Key: Z | z
    LeftBracket,  // Key: [
    Backslash,    // Key: '\'
    RightBracket, // Key: ]
    Grave,        // Key: `

    // Function keys
    Space,        // Key: Space
    Escape,       // Key: Esc
    Enter,        // Key: Enter
    Tab,          // Key: Tab
    Backspace,    // Key: Backspace
    Insert,       // Key: Ins
    Delete,       // Key: Del
    Right,        // Key: Cursor right
    Left,         // Key: Cursor left
    Down,         // Key: Cursor down
    Up,           // Key: Cursor up
    PageUp,       // Key: Page up
    PageDown,     // Key: Page down
    Home,         // Key: Home
    End,          // Key: End
    CapsLock,     // Key: Caps lock
    ScrollLock,   // Key: Scroll down
    NumLock,      // Key: Num lock
    PrintScreen,  // Key: Print screen
    Pause,        // Key: Pause
    F1,           // Key: F1
    F2,           // Key: F2
    F3,           // Key: F3
    F4,           // Key: F4
    F5,           // Key: F5
    F6,           // Key: F6
    F7,           // Key: F7
    F8,           // Key: F8
    F9,           // Key: F9
    F10,          // Key: F10
    F11,          // Key: F11
    F12,          // Key: F12
    LeftShift,    // Key: Shift left
    LeftControl,  // Key: Control left
    LeftAlt,      // Key: Alt left
    LeftSuper,    // Key: Super left
    RightShift,   // Key: Shift right
    RightControl, // Key: Control right
    RightAlt,     // Key: Alt right
    RightSuper,   // Key: Super right
    KbMenu,       // Key: KB menu

    // Keypad keys
    Kp0,        // Key: Keypad 0
    Kp1,        // Key: Keypad 1
    Kp2,        // Key: Keypad 2
    Kp3,        // Key: Keypad 3
    Kp4,        // Key: Keypad 4
    Kp5,        // Key: Keypad 5
    Kp6,        // Key: Keypad 6
    Kp7,        // Key: Keypad 7
    Kp8,        // Key: Keypad 8
    Kp9,        // Key: Keypad 9
    KpDecimal,  // Key: Keypad .
    KpDivide,   // Key: Keypad /
    KpMultiply, // Key: Keypad *
    KpSubtract, // Key: Keypad -
    KpAdd,      // Key: Keypad +
    KpEnter,    // Key: Keypad Enter
    KpEqual,    // Key: Keypad =
}

pub trait KeyboardState {
    fn is_key_pressed(&self, key: KeyboardKey) -> bool;
    fn is_key_down(&self, key: KeyboardKey) -> bool;
    fn is_key_released(&self, key: KeyboardKey) -> bool;
    fn is_key_up(&self, key: KeyboardKey) -> bool;
}

pub trait InputState: KeyboardState {
    /// Called once before a loop is started
    fn begin(&mut self) -> Result<()>;
    /// Next loop is always called directly before the next update cycle.
    fn next_loop(&mut self) -> Result<()>;
    /// Called once a loop has ended
    fn finish(&mut self) -> Result<()>;
}
