use std::char;
use std::time::Duration;

use crate::rustbox::rustbox::{RustBox, Event};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Key {
    Tab,
    Enter,
    Esc,
    Backspace,
    Right,
    Left,
    Up,
    Down,
    Delete,
    Insert,

    Home,
    End,
    PageUp,
    PageDown,

    CtrlLeft,
    CtrlRight,
    AltLeft,
    AltRight,

    Char(char),
    Ctrl(char),
    F(u32),
    Unknown(u16),
}

impl Key {
    pub fn from_special_code(code: u16) -> Option<Key> {
        match code {
            1 => Some(Key::Ctrl('a')),
            2 => Some(Key::Ctrl('b')),
            3 => Some(Key::Ctrl('c')),
            4 => Some(Key::Ctrl('d')),
            5 => Some(Key::Ctrl('e')),
            6 => Some(Key::Ctrl('f')),
            7 => Some(Key::Ctrl('g')),
            8 => Some(Key::Ctrl('h')),
            9 => Some(Key::Tab),
            10 => Some(Key::Ctrl('j')),
            11 => Some(Key::Ctrl('k')),
            12 => Some(Key::Ctrl('l')),
            13 => Some(Key::Enter),
            14 => Some(Key::Ctrl('n')),
            15 => Some(Key::Ctrl('o')),
            16 => Some(Key::Ctrl('p')),
            17 => Some(Key::Ctrl('q')),
            18 => Some(Key::Ctrl('r')),
            19 => Some(Key::Ctrl('s')),
            20 => Some(Key::Ctrl('t')),
            21 => Some(Key::Ctrl('u')),
            22 => Some(Key::Ctrl('v')),
            23 => Some(Key::Ctrl('w')),
            24 => Some(Key::Ctrl('x')),
            25 => Some(Key::Ctrl('y')),
            26 => Some(Key::Ctrl('z')),
            27 => Some(Key::Esc),
            28 => Some(Key::Ctrl('\\')),
            29 => Some(Key::Ctrl(']')),
            30 => Some(Key::Ctrl('6')),
            31 => Some(Key::Ctrl('/')),
            32 => Some(Key::Char(' ')),
            127 => Some(Key::Backspace),
            65514 => Some(Key::Right),
            65515 => Some(Key::Left),
            65516 => Some(Key::Down),
            65517 => Some(Key::Up),
            65535 => Some(Key::F(1)),
            65534 => Some(Key::F(2)),
            65533 => Some(Key::F(3)),
            65532 => Some(Key::F(4)),
            65531 => Some(Key::F(5)),
            65530 => Some(Key::F(6)),
            65529 => Some(Key::F(7)),
            65528 => Some(Key::F(8)),
            65527 => Some(Key::F(9)),
            65526 => Some(Key::F(10)),
            65525 => Some(Key::F(11)),
            65524 => Some(Key::F(12)),
            65523 => Some(Key::Insert),
            65522 => Some(Key::Delete),
            65521 => Some(Key::Home),
            65520 => Some(Key::End),
            65519 => Some(Key::PageUp),
            65518 => Some(Key::PageDown),
            _ => None,
        }
    }

    pub fn from_chord(rb: &mut RustBox, start: u16) -> Option<Key> {
        let chord = Key::get_chord(rb, start);

        match chord.as_str() {
            "\x1b[1;3C" => Some(Key::AltRight),
            "\x1b[1;3D" => Some(Key::AltLeft),
            "\x1b[1;5C" => Some(Key::CtrlRight),
            "\x1b[1;5D" => Some(Key::CtrlLeft),
            _ => {
                Key::from_special_code(start)
            }
        }
    }

    pub fn get_chord(rb: &mut RustBox, start: u16) -> String {
        // Copy any data waiting to a string
        // There may be a cleaner way to do this?
        let mut chord = char::from_u32(u32::from(start)).unwrap().to_string();
        while let Ok(Event::KeyEventRaw(_, _, ch)) = rb.peek_event(Duration::from_secs(0), true) {
            chord.push(char::from_u32(ch).unwrap())
        }

        chord
    }

    pub fn from_event(rb: &mut RustBox, event: Event) -> Option<Key> {
        match event {
            Event::KeyEventRaw(_, k, ch) => {
                match k {
                    0 => char::from_u32(ch).map(Key::Char),
                    0x1b => Key::from_chord(rb, 0x1b),
                    a => Key::from_special_code(a)
                }
            }
            _ => None
        }
    }
}
