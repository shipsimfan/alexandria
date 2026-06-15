use crate::{
    EventKind, input::KeyCode, window::subsystem::linux::wayland::seat_listener::Keyboard,
};

impl<UserEvent: 'static + Send> Keyboard<UserEvent> {
    /// Get the event for a given key
    pub fn key(&mut self, key: u32, pressed: bool, is_repeat: bool) -> EventKind<UserEvent> {
        let key_code = key_code_from_key(key);
        if pressed {
            self.modifiers.apply_key_down(key_code);
            EventKind::KeyDown {
                window_id: None,
                key_mod: self.modifiers,
                key_code: key_code,
                scan_code: key,
                is_repeat,
            }
        } else {
            self.modifiers.apply_key_up(key_code);
            EventKind::KeyUp {
                window_id: None,
                key_mod: self.modifiers,
                key_code: key_code,
                scan_code: key,
            }
        }
    }
}

fn key_code_from_key(key: u32) -> KeyCode {
    match key {
        14 => KeyCode::Backspace,
        15 => KeyCode::Tab,
        28 => KeyCode::Enter,
        1 => KeyCode::Escape,

        57 => KeyCode::Space,
        40 => KeyCode::Quote,
        51 => KeyCode::Comma,
        12 => KeyCode::Minus,
        52 => KeyCode::Period,
        53 => KeyCode::Slash,

        11 => KeyCode::_0,
        2 => KeyCode::_1,
        3 => KeyCode::_2,
        4 => KeyCode::_3,
        5 => KeyCode::_4,
        6 => KeyCode::_5,
        7 => KeyCode::_6,
        8 => KeyCode::_7,
        9 => KeyCode::_8,
        10 => KeyCode::_9,

        39 => KeyCode::SemiColon,
        13 => KeyCode::Equals,
        26 => KeyCode::OpenBracket,
        43 => KeyCode::Backslash,
        27 => KeyCode::CloseBracket,
        41 => KeyCode::Tilde,

        30 => KeyCode::A,
        48 => KeyCode::B,
        46 => KeyCode::C,
        32 => KeyCode::D,
        18 => KeyCode::E,
        33 => KeyCode::F,
        34 => KeyCode::G,
        35 => KeyCode::H,
        23 => KeyCode::I,
        36 => KeyCode::J,
        37 => KeyCode::K,
        38 => KeyCode::L,
        50 => KeyCode::M,
        49 => KeyCode::N,
        24 => KeyCode::O,
        25 => KeyCode::P,
        16 => KeyCode::Q,
        19 => KeyCode::R,
        31 => KeyCode::S,
        20 => KeyCode::T,
        22 => KeyCode::U,
        47 => KeyCode::V,
        17 => KeyCode::W,
        45 => KeyCode::X,
        21 => KeyCode::Y,
        44 => KeyCode::Z,

        42 => KeyCode::LeftShift,
        54 => KeyCode::RightShift,
        29 => KeyCode::LeftControl,
        97 => KeyCode::RightControl,
        56 => KeyCode::LeftAlt,
        100 => KeyCode::RightAlt,
        125 => KeyCode::LeftSuper,
        126 => KeyCode::RightSuper,

        105 => KeyCode::Left,
        103 => KeyCode::Up,
        106 => KeyCode::Right,
        108 => KeyCode::Down,

        59 => KeyCode::F1,
        60 => KeyCode::F2,
        61 => KeyCode::F3,
        62 => KeyCode::F4,
        63 => KeyCode::F5,
        64 => KeyCode::F6,
        65 => KeyCode::F7,
        66 => KeyCode::F8,
        67 => KeyCode::F9,
        68 => KeyCode::F10,
        87 => KeyCode::F11,
        88 => KeyCode::F12,

        58 => KeyCode::CapsLock,
        70 => KeyCode::ScrollLock,
        69 => KeyCode::NumLock,

        99 => KeyCode::PrintScreen,
        119 => KeyCode::Pause,
        110 => KeyCode::Insert,
        111 => KeyCode::Delete,
        102 => KeyCode::Home,
        107 => KeyCode::End,
        104 => KeyCode::PageUp,
        109 => KeyCode::PageDown,

        82 => KeyCode::Np0,
        79 => KeyCode::Np1,
        80 => KeyCode::Np2,
        81 => KeyCode::Np3,
        75 => KeyCode::Np4,
        76 => KeyCode::Np5,
        77 => KeyCode::Np6,
        71 => KeyCode::Np7,
        72 => KeyCode::Np8,
        73 => KeyCode::Np9,
        83 => KeyCode::NpDecimal,
        98 => KeyCode::NpDivide,
        55 => KeyCode::NpMultiply,
        74 => KeyCode::NpSubtract,
        78 => KeyCode::NpAdd,
        96 => KeyCode::NpEnter,

        127 => KeyCode::Menu,

        _ => {
            #[cfg(debug_assertions)]
            println!("Unknown key: {}", key);
            KeyCode::Unknown
        }
    }
}
