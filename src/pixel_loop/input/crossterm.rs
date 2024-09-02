use std::collections::{HashMap, HashSet};
use std::time::Duration;

use super::{InputState, KeyboardKey, KeyboardState};
use anyhow::Result;
use crossterm::event::{
    Event, KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
};
use crossterm::execute;

pub struct CrosstermInputState {
    keys_down: HashMap<KeyboardKey, usize>,
    keys_pressed_this_update: HashSet<KeyboardKey>,
    keys_released_this_update: HashSet<KeyboardKey>,
    event_cycles_before_released: usize,
    enhanced_keyboard: bool,
}

impl CrosstermInputState {
    pub fn new() -> Self {
        Self {
            keys_down: HashMap::new(),
            keys_pressed_this_update: HashSet::new(),
            keys_released_this_update: HashSet::new(),
            event_cycles_before_released: 2,
            enhanced_keyboard: false,
        }
    }

    pub fn with_event_cycles_before_released(self, cycles: usize) -> Self {
        Self {
            event_cycles_before_released: cycles,
            ..self
        }
    }
}

fn map_crossterm_keycode_to_pixel_loop(keycode: &crossterm::event::KeyCode) -> Option<KeyboardKey> {
    use crossterm::event::KeyCode;
    match keycode {
        KeyCode::Backspace => Some(KeyboardKey::Backspace),
        KeyCode::Enter => Some(KeyboardKey::Enter),
        KeyCode::Left => Some(KeyboardKey::Left),
        KeyCode::Right => Some(KeyboardKey::Right),
        KeyCode::Up => Some(KeyboardKey::Up),
        KeyCode::Down => Some(KeyboardKey::Down),
        KeyCode::Home => Some(KeyboardKey::Home),
        KeyCode::End => Some(KeyboardKey::End),
        KeyCode::PageUp => Some(KeyboardKey::PageUp),
        KeyCode::PageDown => Some(KeyboardKey::PageDown),
        KeyCode::Tab => Some(KeyboardKey::Tab),
        KeyCode::BackTab => None,
        KeyCode::Delete => Some(KeyboardKey::Delete),
        KeyCode::Insert => Some(KeyboardKey::Insert),
        KeyCode::F(ref fkey) => match fkey {
            1 => Some(KeyboardKey::F1),
            2 => Some(KeyboardKey::F2),
            3 => Some(KeyboardKey::F3),
            4 => Some(KeyboardKey::F4),
            5 => Some(KeyboardKey::F5),
            6 => Some(KeyboardKey::F6),
            7 => Some(KeyboardKey::F7),
            8 => Some(KeyboardKey::F8),
            9 => Some(KeyboardKey::F9),
            10 => Some(KeyboardKey::F10),
            11 => Some(KeyboardKey::F11),
            12 => Some(KeyboardKey::F12),
            _ => None,
        },
        KeyCode::Char(ref character) => match character {
            '1' => Some(KeyboardKey::One),
            '2' => Some(KeyboardKey::Two),
            '3' => Some(KeyboardKey::Three),
            '4' => Some(KeyboardKey::Four),
            '5' => Some(KeyboardKey::Five),
            '6' => Some(KeyboardKey::Six),
            '7' => Some(KeyboardKey::Seven),
            '8' => Some(KeyboardKey::Eight),
            '9' => Some(KeyboardKey::Nine),
            '0' => Some(KeyboardKey::Zero),
            'a' | 'A' => Some(KeyboardKey::A),
            'b' | 'B' => Some(KeyboardKey::B),
            'c' | 'C' => Some(KeyboardKey::C),
            'd' | 'D' => Some(KeyboardKey::D),
            'e' | 'E' => Some(KeyboardKey::E),
            'f' | 'F' => Some(KeyboardKey::F),
            'g' | 'G' => Some(KeyboardKey::G),
            'h' | 'H' => Some(KeyboardKey::H),
            'i' | 'I' => Some(KeyboardKey::I),
            'j' | 'J' => Some(KeyboardKey::J),
            'k' | 'K' => Some(KeyboardKey::K),
            'l' | 'L' => Some(KeyboardKey::L),
            'm' | 'M' => Some(KeyboardKey::M),
            'n' | 'N' => Some(KeyboardKey::N),
            'o' | 'O' => Some(KeyboardKey::O),
            'p' | 'P' => Some(KeyboardKey::P),
            'q' | 'Q' => Some(KeyboardKey::Q),
            'r' | 'R' => Some(KeyboardKey::R),
            's' | 'S' => Some(KeyboardKey::S),
            't' | 'T' => Some(KeyboardKey::T),
            'u' | 'U' => Some(KeyboardKey::U),
            'v' | 'V' => Some(KeyboardKey::V),
            'w' | 'W' => Some(KeyboardKey::W),
            'x' | 'X' => Some(KeyboardKey::X),
            'y' | 'Y' => Some(KeyboardKey::Y),
            'z' | 'Z' => Some(KeyboardKey::Z),
            '\'' => Some(KeyboardKey::Apostrophe),
            ',' => Some(KeyboardKey::Comma),
            '-' => Some(KeyboardKey::Minus),
            '.' => Some(KeyboardKey::Period),
            '/' => Some(KeyboardKey::Slash),
            ';' => Some(KeyboardKey::Semicolon),
            '=' => Some(KeyboardKey::Equal),
            '[' => Some(KeyboardKey::LeftBracket),
            '\\' => Some(KeyboardKey::Backslash),
            ']' => Some(KeyboardKey::RightBracket),
            '`' => Some(KeyboardKey::Grave),
            ' ' => Some(KeyboardKey::Space),
            _ => None,
        },
        KeyCode::Null => None,
        KeyCode::Esc => Some(KeyboardKey::Escape),
        KeyCode::CapsLock => Some(KeyboardKey::CapsLock),
        KeyCode::ScrollLock => Some(KeyboardKey::ScrollLock),
        KeyCode::NumLock => Some(KeyboardKey::NumLock),
        KeyCode::PrintScreen => Some(KeyboardKey::PrintScreen),
        KeyCode::Pause => Some(KeyboardKey::Pause),
        KeyCode::Menu => Some(KeyboardKey::KbMenu),
        KeyCode::KeypadBegin => None,
        KeyCode::Media(_) => None,
        KeyCode::Modifier(_) => None, //@TODO: implement
    }
}

fn get_all_next_crossterm_events() -> Result<Vec<Event>> {
    use crossterm::event::{poll, read};
    let mut events = vec![];
    loop {
        if poll(Duration::from_secs(0))? {
            let event = read()?;
            events.push(event);
        } else {
            break;
        }
    }

    Ok(events)
}

fn decrement_key_ref_counts(hmap: &mut HashMap<KeyboardKey, usize>) -> Vec<KeyboardKey> {
    let mut removed_keys = vec![];
    // Shortcut if our length is 0. We are doing this, as this is mostly the
    // case, when no key is pressed. The hashmap iteration always has a
    // complexity of O(capacity) not O(len) due to internal implementation.
    if hmap.len() == 0 {
        return removed_keys;
    }

    hmap.retain(|key, refcount: &mut usize| {
        if *refcount > 0 {
            *refcount -= 1;
        }

        if *refcount == 0 {
            removed_keys.push(*key);
            return false;
        }

        true
    });

    removed_keys
}

impl CrosstermInputState {
    fn next_loop_fallback(&mut self, next_events: Vec<Event>) -> Result<()> {
        use crossterm::event::{KeyEvent, KeyEventKind};

        let removed_keys_down = decrement_key_ref_counts(&mut self.keys_down);
        let keys_pressed_last_update = std::mem::take(&mut self.keys_pressed_this_update);
        let keys_released_last_update = std::mem::take(&mut self.keys_released_this_update);

        for event in next_events {
            match event {
                // Handle all pressed keys
                Event::Key(KeyEvent {
                    kind: KeyEventKind::Press,
                    ref code,
                    ..
                }) => {
                    if let Some(keyboard_key) = map_crossterm_keycode_to_pixel_loop(code) {
                        // eprintln!("key DOWN handled {:?}", keyboard_key);
                        if self
                            .keys_down
                            .insert(keyboard_key, self.event_cycles_before_released)
                            .is_none()
                        {
                            // eprintln!("key PRESS handled {:?}", keyboard_key);
                            // Key is newly inserted.
                            self.keys_pressed_this_update.insert(keyboard_key);
                        }
                    } else {
                        // eprintln!("Keypress NOT mapped");
                    }
                }
                _ => {}
            }
        }

        // Fill keys, released this frame
        for removed_key in removed_keys_down {
            if !self.keys_down.contains_key(&removed_key) {
                // eprintln!("key RELEASE handled {:?}", removed_key);
                self.keys_released_this_update.insert(removed_key);
            }
        }

        Ok(())
    }

    fn next_loop_enhanced(&mut self, next_events: Vec<Event>) -> Result<()> {
        use crossterm::event::{KeyEvent, KeyEventKind};

        self.keys_pressed_this_update.drain();
        self.keys_released_this_update.drain();

        for event in next_events {
            match event {
                // Handle all pressed keys
                Event::Key(KeyEvent {
                    ref kind, ref code, ..
                }) => {
                    if let Some(keyboard_key) = map_crossterm_keycode_to_pixel_loop(code) {
                        match kind {
                            KeyEventKind::Press => {
                                // eprintln!("KEY DOWN: {:?}", keyboard_key);
                                if self
                                    .keys_down
                                    .insert(keyboard_key, self.event_cycles_before_released)
                                    .is_none()
                                {
                                    // eprintln!("KEY PRESS: {:?}", keyboard_key);
                                    self.keys_pressed_this_update.insert(keyboard_key);
                                }
                            }
                            KeyEventKind::Release => {
                                // eprintln!("KEY UP: {:?}", keyboard_key);
                                if self.keys_down.remove(&keyboard_key).is_some() {
                                    // eprintln!("KEY RELEASE: {:?}", keyboard_key);
                                    self.keys_released_this_update.insert(keyboard_key);
                                }
                            }
                            KeyEventKind::Repeat => {
                                // eprintln!("KEY REPEAT: {:?}", keyboard_key);
                                // @TODO: Not handled yet. There isn't an API in hour trait for that (yet)
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }
}

impl InputState for CrosstermInputState {
    fn begin(&mut self) -> Result<()> {
        crossterm::terminal::enable_raw_mode()?;
        if crossterm::terminal::supports_keyboard_enhancement()? {
            // eprintln!("Enhanced Terminal YEAH!");
            self.enhanced_keyboard = true;
            execute!(
                std::io::stdout(),
                PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::REPORT_EVENT_TYPES)
            )?;
        } else {
            // eprintln!("No enhanced Terminal :_(");
        }
        Ok(())
    }

    fn finish(&mut self) -> Result<()> {
        if self.enhanced_keyboard {
            execute!(std::io::stdout(), PopKeyboardEnhancementFlags)?;
            self.enhanced_keyboard = false;
        }
        crossterm::terminal::disable_raw_mode()?;
        Ok(())
    }

    fn next_loop(&mut self) -> Result<()> {
        use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

        let next_events = get_all_next_crossterm_events()?;
        for event in next_events.iter() {
            match event {
                // Handle Ctrl-C
                Event::Key(KeyEvent {
                    kind: KeyEventKind::Press,
                    code: KeyCode::Char('c') | KeyCode::Char('C'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                }) => {
                    // SIGINT exitcode
                    std::process::exit(130);
                }
                _ => {}
            }
        }

        if self.enhanced_keyboard {
            self.next_loop_enhanced(next_events)?;
        } else {
            self.next_loop_fallback(next_events)?;
        }

        Ok(())
    }
}

impl KeyboardState for CrosstermInputState {
    fn is_key_pressed(&self, key: KeyboardKey) -> bool {
        self.keys_pressed_this_update.contains(&key)
    }

    fn is_key_down(&self, key: KeyboardKey) -> bool {
        self.keys_down.contains_key(&key)
    }

    fn is_key_released(&self, key: KeyboardKey) -> bool {
        self.keys_released_this_update.contains(&key)
    }

    fn is_key_up(&self, key: KeyboardKey) -> bool {
        !self.keys_down.contains_key(&key)
    }
}
