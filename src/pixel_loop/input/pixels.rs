use std::collections::HashSet;

use winit::event::{Event, VirtualKeyCode};

use super::{InputState, KeyboardKey, KeyboardState};

// Map winit keycodes to our KeyboardKey enum
fn map_winit_key_to_pixel_loop(key: winit::event::VirtualKeyCode) -> Option<KeyboardKey> {
    match key {
        // Alphanumeric
        VirtualKeyCode::Apostrophe => Some(KeyboardKey::Apostrophe),
        VirtualKeyCode::Comma => Some(KeyboardKey::Comma),
        VirtualKeyCode::Minus => Some(KeyboardKey::Minus),
        VirtualKeyCode::Period => Some(KeyboardKey::Period),
        VirtualKeyCode::Slash => Some(KeyboardKey::Slash),
        VirtualKeyCode::Key0 => Some(KeyboardKey::Zero),
        VirtualKeyCode::Key1 => Some(KeyboardKey::One),
        VirtualKeyCode::Key2 => Some(KeyboardKey::Two),
        VirtualKeyCode::Key3 => Some(KeyboardKey::Three),
        VirtualKeyCode::Key4 => Some(KeyboardKey::Four),
        VirtualKeyCode::Key5 => Some(KeyboardKey::Five),
        VirtualKeyCode::Key6 => Some(KeyboardKey::Six),
        VirtualKeyCode::Key7 => Some(KeyboardKey::Seven),
        VirtualKeyCode::Key8 => Some(KeyboardKey::Eight),
        VirtualKeyCode::Key9 => Some(KeyboardKey::Nine),
        VirtualKeyCode::Semicolon => Some(KeyboardKey::Semicolon),
        VirtualKeyCode::Equals => Some(KeyboardKey::Equal),
        VirtualKeyCode::A => Some(KeyboardKey::A),
        VirtualKeyCode::B => Some(KeyboardKey::B),
        VirtualKeyCode::C => Some(KeyboardKey::C),
        VirtualKeyCode::D => Some(KeyboardKey::D),
        VirtualKeyCode::E => Some(KeyboardKey::E),
        VirtualKeyCode::F => Some(KeyboardKey::F),
        VirtualKeyCode::G => Some(KeyboardKey::G),
        VirtualKeyCode::H => Some(KeyboardKey::H),
        VirtualKeyCode::I => Some(KeyboardKey::I),
        VirtualKeyCode::J => Some(KeyboardKey::J),
        VirtualKeyCode::K => Some(KeyboardKey::K),
        VirtualKeyCode::L => Some(KeyboardKey::L),
        VirtualKeyCode::M => Some(KeyboardKey::M),
        VirtualKeyCode::N => Some(KeyboardKey::N),
        VirtualKeyCode::O => Some(KeyboardKey::O),
        VirtualKeyCode::P => Some(KeyboardKey::P),
        VirtualKeyCode::Q => Some(KeyboardKey::Q),
        VirtualKeyCode::R => Some(KeyboardKey::R),
        VirtualKeyCode::S => Some(KeyboardKey::S),
        VirtualKeyCode::T => Some(KeyboardKey::T),
        VirtualKeyCode::U => Some(KeyboardKey::U),
        VirtualKeyCode::V => Some(KeyboardKey::V),
        VirtualKeyCode::W => Some(KeyboardKey::W),
        VirtualKeyCode::X => Some(KeyboardKey::X),
        VirtualKeyCode::Y => Some(KeyboardKey::Y),
        VirtualKeyCode::Z => Some(KeyboardKey::Z),
        VirtualKeyCode::LBracket => Some(KeyboardKey::LeftBracket),
        VirtualKeyCode::Backslash => Some(KeyboardKey::Backslash),
        VirtualKeyCode::RBracket => Some(KeyboardKey::RightBracket),
        VirtualKeyCode::Grave => Some(KeyboardKey::Grave),

        // Function keys
        VirtualKeyCode::Space => Some(KeyboardKey::Space),
        VirtualKeyCode::Escape => Some(KeyboardKey::Escape),
        VirtualKeyCode::Return => Some(KeyboardKey::Enter),
        VirtualKeyCode::Tab => Some(KeyboardKey::Tab),
        VirtualKeyCode::Back => Some(KeyboardKey::Backspace),
        VirtualKeyCode::Insert => Some(KeyboardKey::Insert),
        VirtualKeyCode::Delete => Some(KeyboardKey::Delete),
        VirtualKeyCode::Right => Some(KeyboardKey::Right),
        VirtualKeyCode::Left => Some(KeyboardKey::Left),
        VirtualKeyCode::Down => Some(KeyboardKey::Down),
        VirtualKeyCode::Up => Some(KeyboardKey::Up),
        VirtualKeyCode::PageUp => Some(KeyboardKey::PageUp),
        VirtualKeyCode::PageDown => Some(KeyboardKey::PageDown),
        VirtualKeyCode::Home => Some(KeyboardKey::Home),
        VirtualKeyCode::End => Some(KeyboardKey::End),
        VirtualKeyCode::Capital => Some(KeyboardKey::CapsLock),
        VirtualKeyCode::Scroll => Some(KeyboardKey::ScrollLock),
        VirtualKeyCode::Numlock => Some(KeyboardKey::NumLock),
        VirtualKeyCode::Snapshot => Some(KeyboardKey::PrintScreen),
        VirtualKeyCode::Pause => Some(KeyboardKey::Pause),
        VirtualKeyCode::F1 => Some(KeyboardKey::F1),
        VirtualKeyCode::F2 => Some(KeyboardKey::F2),
        VirtualKeyCode::F3 => Some(KeyboardKey::F3),
        VirtualKeyCode::F4 => Some(KeyboardKey::F4),
        VirtualKeyCode::F5 => Some(KeyboardKey::F5),
        VirtualKeyCode::F6 => Some(KeyboardKey::F6),
        VirtualKeyCode::F7 => Some(KeyboardKey::F7),
        VirtualKeyCode::F8 => Some(KeyboardKey::F8),
        VirtualKeyCode::F9 => Some(KeyboardKey::F9),
        VirtualKeyCode::F10 => Some(KeyboardKey::F10),
        VirtualKeyCode::F11 => Some(KeyboardKey::F11),
        VirtualKeyCode::F12 => Some(KeyboardKey::F12),
        VirtualKeyCode::LShift => Some(KeyboardKey::LeftShift),
        VirtualKeyCode::LControl => Some(KeyboardKey::LeftControl),
        VirtualKeyCode::LAlt => Some(KeyboardKey::LeftAlt),
        VirtualKeyCode::LWin => Some(KeyboardKey::LeftSuper),
        VirtualKeyCode::RShift => Some(KeyboardKey::RightShift),
        VirtualKeyCode::RControl => Some(KeyboardKey::RightControl),
        VirtualKeyCode::RAlt => Some(KeyboardKey::RightAlt),
        VirtualKeyCode::RWin => Some(KeyboardKey::RightSuper),
        VirtualKeyCode::Apps => Some(KeyboardKey::KbMenu),

        // Keypad
        VirtualKeyCode::Numpad0 => Some(KeyboardKey::Kp0),
        VirtualKeyCode::Numpad1 => Some(KeyboardKey::Kp1),
        VirtualKeyCode::Numpad2 => Some(KeyboardKey::Kp2),
        VirtualKeyCode::Numpad3 => Some(KeyboardKey::Kp3),
        VirtualKeyCode::Numpad4 => Some(KeyboardKey::Kp4),
        VirtualKeyCode::Numpad5 => Some(KeyboardKey::Kp5),
        VirtualKeyCode::Numpad6 => Some(KeyboardKey::Kp6),
        VirtualKeyCode::Numpad7 => Some(KeyboardKey::Kp7),
        VirtualKeyCode::Numpad8 => Some(KeyboardKey::Kp8),
        VirtualKeyCode::Numpad9 => Some(KeyboardKey::Kp9),
        VirtualKeyCode::NumpadDecimal => Some(KeyboardKey::KpDecimal),
        VirtualKeyCode::NumpadDivide => Some(KeyboardKey::KpDivide),
        VirtualKeyCode::NumpadMultiply => Some(KeyboardKey::KpMultiply),
        VirtualKeyCode::NumpadSubtract => Some(KeyboardKey::KpSubtract),
        VirtualKeyCode::NumpadAdd => Some(KeyboardKey::KpAdd),
        VirtualKeyCode::NumpadEnter => Some(KeyboardKey::KpEnter),
        VirtualKeyCode::NumpadEquals => Some(KeyboardKey::KpEqual),

        // Keys we don't map
        _ => None,
    }
}
pub struct PixelsInputState {
    keys_down: HashSet<KeyboardKey>,
    keys_pressed_this_update: HashSet<KeyboardKey>,
    keys_released_this_update: HashSet<KeyboardKey>,
    clear_before_next_event: bool,
}

impl PixelsInputState {
    pub fn new() -> Self {
        Self {
            keys_down: HashSet::new(),
            keys_pressed_this_update: HashSet::new(),
            keys_released_this_update: HashSet::new(),
            clear_before_next_event: true,
        }
    }

    pub(crate) fn handle_new_event(&mut self, event: &Event<()>) {
        if self.clear_before_next_event {
            self.keys_pressed_this_update.clear();
            self.keys_released_this_update.clear();
            self.clear_before_next_event = false;
        }

        match event {
            Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::KeyboardInput {
                    input:
                        winit::event::KeyboardInput {
                            state,
                            virtual_keycode: Some(key),
                            ..
                        },
                    ..
                } => {
                    if *state == winit::event::ElementState::Pressed {
                        if let Some(key) = map_winit_key_to_pixel_loop(*key) {
                            if !self.keys_down.contains(&key) {
                                self.keys_pressed_this_update.insert(key);
                            }
                            self.keys_down.insert(key);
                        }
                    } else {
                        if let Some(key) = map_winit_key_to_pixel_loop(*key) {
                            if self.keys_down.contains(&key) {
                                self.keys_released_this_update.insert(key);
                            }
                            self.keys_down.remove(&key);
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }
}

impl InputState for PixelsInputState {
    fn begin(&mut self) -> anyhow::Result<()> {
        // Nothing to do here
        Ok(())
    }

    fn next_loop(&mut self) -> anyhow::Result<()> {
        self.clear_before_next_event = true;
        Ok(())
    }

    fn finish(&mut self) -> anyhow::Result<()> {
        // Nothing to do here
        Ok(())
    }
}

impl KeyboardState for PixelsInputState {
    fn is_key_pressed(&self, key: KeyboardKey) -> bool {
        self.keys_pressed_this_update.contains(&key)
    }

    fn is_key_down(&self, key: KeyboardKey) -> bool {
        self.keys_down.contains(&key)
    }

    fn is_key_released(&self, key: KeyboardKey) -> bool {
        self.keys_released_this_update.contains(&key)
    }

    fn is_key_up(&self, key: KeyboardKey) -> bool {
        !self.keys_down.contains(&key)
    }
}
