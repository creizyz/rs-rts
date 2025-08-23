use std::fmt;
use std::fmt::Formatter;
use bevy::input::ButtonInput;
use bevy::prelude::{KeyCode, Res};
use crate::core::{Serializable, SerializableEnum};

static KEY_BINDING_SEPARATOR: &str = " + ";

trait KeyInput {
    fn is_pressed(&self, keys: &Res<ButtonInput<KeyCode>>) -> bool;
    fn from_keycode(key: KeyCode) -> Option<Self>
    where
        Self: Sized;
}

enum Key {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Escape,
    Enter,
    Space,
    Backspace,
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    ArrowDown,
    Tab,
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", SerializableEnum::to_string(self))
    }
}

impl KeyInput for Key {
    fn is_pressed(&self, keys: &Res<ButtonInput<KeyCode>>) -> bool {
        match self {
            Key::A          => keys.pressed(KeyCode::KeyA),
            Key::B          => keys.pressed(KeyCode::KeyB),
            Key::C          => keys.pressed(KeyCode::KeyC),
            Key::D          => keys.pressed(KeyCode::KeyD),
            Key::E          => keys.pressed(KeyCode::KeyE),
            Key::F          => keys.pressed(KeyCode::KeyF),
            Key::G          => keys.pressed(KeyCode::KeyG),
            Key::H          => keys.pressed(KeyCode::KeyH),
            Key::I          => keys.pressed(KeyCode::KeyI),
            Key::J          => keys.pressed(KeyCode::KeyJ),
            Key::K          => keys.pressed(KeyCode::KeyK),
            Key::L          => keys.pressed(KeyCode::KeyL),
            Key::M          => keys.pressed(KeyCode::KeyM),
            Key::N          => keys.pressed(KeyCode::KeyN),
            Key::O          => keys.pressed(KeyCode::KeyO),
            Key::P          => keys.pressed(KeyCode::KeyP),
            Key::Q          => keys.pressed(KeyCode::KeyQ),
            Key::R          => keys.pressed(KeyCode::KeyR),
            Key::S          => keys.pressed(KeyCode::KeyS),
            Key::T          => keys.pressed(KeyCode::KeyT),
            Key::U          => keys.pressed(KeyCode::KeyU),
            Key::V          => keys.pressed(KeyCode::KeyV),
            Key::W          => keys.pressed(KeyCode::KeyW),
            Key::X          => keys.pressed(KeyCode::KeyX),
            Key::Y          => keys.pressed(KeyCode::KeyY),
            Key::Z          => keys.pressed(KeyCode::KeyZ),
            Key::Num0       => keys.pressed(KeyCode::Digit0),
            Key::Num1       => keys.pressed(KeyCode::Digit1),
            Key::Num2       => keys.pressed(KeyCode::Digit2),
            Key::Num3       => keys.pressed(KeyCode::Digit3),
            Key::Num4       => keys.pressed(KeyCode::Digit4),
            Key::Num5       => keys.pressed(KeyCode::Digit5),
            Key::Num6       => keys.pressed(KeyCode::Digit6),
            Key::Num7       => keys.pressed(KeyCode::Digit7),
            Key::Num8       => keys.pressed(KeyCode::Digit8),
            Key::Num9       => keys.pressed(KeyCode::Digit9),
            Key::Escape     => keys.pressed(KeyCode::Escape),
            Key::Enter      => keys.pressed(KeyCode::Enter),
            Key::Space      => keys.pressed(KeyCode::Space),
            Key::Backspace  => keys.pressed(KeyCode::Backspace),
            Key::ArrowLeft  => keys.pressed(KeyCode::ArrowLeft),
            Key::ArrowRight => keys.pressed(KeyCode::ArrowRight),
            Key::ArrowUp    => keys.pressed(KeyCode::ArrowUp),
            Key::ArrowDown  => keys.pressed(KeyCode::ArrowDown),
            Key::Tab        => keys.pressed(KeyCode::Tab),
        }
    }

    fn from_keycode(key: KeyCode) -> Option<Self>
    where
        Self: Sized
    {
        match key {
            KeyCode::KeyA       =>  Some(Key::A),
            KeyCode::KeyB       =>  Some(Key::B),
            KeyCode::KeyC       =>  Some(Key::C),
            KeyCode::KeyD       =>  Some(Key::D),
            KeyCode::KeyE       =>  Some(Key::E),
            KeyCode::KeyF       =>  Some(Key::F),
            KeyCode::KeyG       =>  Some(Key::G),
            KeyCode::KeyH       =>  Some(Key::H),
            KeyCode::KeyI       =>  Some(Key::I),
            KeyCode::KeyJ       =>  Some(Key::J),
            KeyCode::KeyK       =>  Some(Key::K),
            KeyCode::KeyL       =>  Some(Key::L),
            KeyCode::KeyM       =>  Some(Key::M),
            KeyCode::KeyN       =>  Some(Key::N),
            KeyCode::KeyO       =>  Some(Key::O),
            KeyCode::KeyP       =>  Some(Key::P),
            KeyCode::KeyQ       =>  Some(Key::Q),
            KeyCode::KeyR       =>  Some(Key::R),
            KeyCode::KeyS       =>  Some(Key::S),
            KeyCode::KeyT       =>  Some(Key::T),
            KeyCode::KeyU       =>  Some(Key::U),
            KeyCode::KeyV       =>  Some(Key::V),
            KeyCode::KeyW       =>  Some(Key::W),
            KeyCode::KeyX       =>  Some(Key::X),
            KeyCode::KeyY       =>  Some(Key::Y),
            KeyCode::KeyZ       =>  Some(Key::Z),
            KeyCode::Digit0     =>  Some(Key::Num0),
            KeyCode::Digit1     =>  Some(Key::Num1),
            KeyCode::Digit2     =>  Some(Key::Num2),
            KeyCode::Digit3     =>  Some(Key::Num3),
            KeyCode::Digit4     =>  Some(Key::Num4),
            KeyCode::Digit5     =>  Some(Key::Num5),
            KeyCode::Digit6     =>  Some(Key::Num6),
            KeyCode::Digit7     =>  Some(Key::Num7),
            KeyCode::Digit8     =>  Some(Key::Num8),
            KeyCode::Digit9     =>  Some(Key::Num9),
            KeyCode::Escape     =>  Some(Key::Escape),
            KeyCode::Enter      =>  Some(Key::Enter),
            KeyCode::Space      =>  Some(Key::Space),
            KeyCode::Backspace  =>  Some(Key::Backspace),
            KeyCode::ArrowLeft  =>  Some(Key::ArrowLeft),
            KeyCode::ArrowRight =>  Some(Key::ArrowRight),
            KeyCode::ArrowUp    =>  Some(Key::ArrowUp),
            KeyCode::ArrowDown  =>  Some(Key::ArrowDown),
            KeyCode::Tab        =>  Some(Key::Tab),
            _ => None,
        }
    }
}

impl SerializableEnum for Key {
    fn to_string(&self) -> &'static str {
        match self {
            Key::A          => "A",
            Key::B          => "B",
            Key::C          => "C",
            Key::D          => "D",
            Key::E          => "E",
            Key::F          => "F",
            Key::G          => "G",
            Key::H          => "H",
            Key::I          => "I",
            Key::J          => "J",
            Key::K          => "K",
            Key::L          => "L",
            Key::M          => "M",
            Key::N          => "N",
            Key::O          => "O",
            Key::P          => "P",
            Key::Q          => "Q",
            Key::R          => "R",
            Key::S          => "S",
            Key::T          => "T",
            Key::U          => "U",
            Key::V          => "V",
            Key::W          => "W",
            Key::X          => "X",
            Key::Y          => "Y",
            Key::Z          => "Z",
            Key::Num0       => "0",
            Key::Num1       => "1",
            Key::Num2       => "2",
            Key::Num3       => "3",
            Key::Num4       => "4",
            Key::Num5       => "5",
            Key::Num6       => "6",
            Key::Num7       => "7",
            Key::Num8       => "8",
            Key::Num9       => "9",
            Key::Escape     => "Escape",
            Key::Enter      => "Enter",
            Key::Space      => "Space",
            Key::Backspace  => "Backspace",
            Key::ArrowLeft  => "ArrowLeft",
            Key::ArrowRight => "ArrowRight",
            Key::ArrowUp    => "ArrowUp",
            Key::ArrowDown  => "ArrowDown",
            Key::Tab        => "Tab",
        }
    }

    fn from_string(string: &str) -> Option<Self>
    where
        Self: Sized
    {
        match string {
            "A"          =>  Some(Key::A),
            "B"          =>  Some(Key::B),
            "C"          =>  Some(Key::C),
            "D"          =>  Some(Key::D),
            "E"          =>  Some(Key::E),
            "F"          =>  Some(Key::F),
            "G"          =>  Some(Key::G),
            "H"          =>  Some(Key::H),
            "I"          =>  Some(Key::I),
            "J"          =>  Some(Key::J),
            "K"          =>  Some(Key::K),
            "L"          =>  Some(Key::L),
            "M"          =>  Some(Key::M),
            "N"          =>  Some(Key::N),
            "O"          =>  Some(Key::O),
            "P"          =>  Some(Key::P),
            "Q"          =>  Some(Key::Q),
            "R"          =>  Some(Key::R),
            "S"          =>  Some(Key::S),
            "T"          =>  Some(Key::T),
            "U"          =>  Some(Key::U),
            "V"          =>  Some(Key::V),
            "W"          =>  Some(Key::W),
            "X"          =>  Some(Key::X),
            "Y"          =>  Some(Key::Y),
            "Z"          =>  Some(Key::Z),
            "0"          =>  Some(Key::Num0),
            "1"          =>  Some(Key::Num1),
            "2"          =>  Some(Key::Num2),
            "3"          =>  Some(Key::Num3),
            "4"          =>  Some(Key::Num4),
            "5"          =>  Some(Key::Num5),
            "6"          =>  Some(Key::Num6),
            "7"          =>  Some(Key::Num7),
            "8"          =>  Some(Key::Num8),
            "9"          =>  Some(Key::Num9),
            "Escape"     =>  Some(Key::Escape),
            "Enter"      =>  Some(Key::Enter),
            "Space"      =>  Some(Key::Space),
            "Backspace"  =>  Some(Key::Backspace),
            "ArrowLeft"  =>  Some(Key::ArrowLeft),
            "ArrowRight" =>  Some(Key::ArrowRight),
            "ArrowUp"    =>  Some(Key::ArrowUp),
            "ArrowDown"  =>  Some(Key::ArrowDown),
            "Tab"        =>  Some(Key::Tab),
            _ => None,
        }
    }
}

enum Modifier {
    Shift,
    Ctrl,
    Alt,
    Super,
}

impl fmt::Display for Modifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", SerializableEnum::to_string(self))
    }
}
impl KeyInput for Modifier {
    fn is_pressed(&self, keys: &Res<ButtonInput<KeyCode>>) -> bool {
        match self {
            Modifier::Shift => keys.pressed(KeyCode::ShiftLeft)   || keys.pressed(KeyCode::ShiftRight),
            Modifier::Ctrl  => keys.pressed(KeyCode::ControlLeft) || keys.pressed(KeyCode::ControlRight),
            Modifier::Alt   => keys.pressed(KeyCode::AltLeft)     || keys.pressed(KeyCode::AltRight),
            Modifier::Super => keys.pressed(KeyCode::SuperLeft)   || keys.pressed(KeyCode::SuperRight),
        }
    }

    fn from_keycode(key: KeyCode) -> Option<Self>
    where
        Self: Sized
    {
        match key {
            KeyCode::ShiftLeft   | KeyCode::ShiftRight   => Some(Self::Shift),
            KeyCode::ControlLeft | KeyCode::ControlRight => Some(Self::Ctrl),
            KeyCode::AltLeft     | KeyCode::AltRight     => Some(Self::Alt),
            KeyCode::SuperLeft   | KeyCode::SuperRight   => Some(Self::Super),
            _ => None,
        }
    }
}

impl SerializableEnum for Modifier {
    fn to_string(&self) -> &'static str {
        match self {
            Modifier::Shift => "Shift",
            Modifier::Ctrl  => "Ctrl",
            Modifier::Alt   => "Alt",
            Modifier::Super => "Super",
        }
    }

    fn from_string(string: &str) -> Option<Self>
    where
        Self: Sized
    {
        match string {
            "Shift" => Some(Self::Shift),
            "Ctrl"  => Some(Self::Ctrl),
            "Alt"   => Some(Self::Alt),
            "Super" => Some(Self::Super),
            _ => None,
        }
    }
}

pub struct KeyBinding {
    key: Key,
    modifiers: Vec<Modifier>,
}

impl fmt::Display for KeyBinding {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Serializable::to_string(self))
    }
}

impl KeyBinding {
    pub fn is_pressed(&self, keys: &Res<ButtonInput<KeyCode>>) -> bool {
        self.modifiers.iter().all(|modifier| modifier.is_pressed(keys)) && self.key.is_pressed(keys)
    }

    pub fn from_keycodes(keys: Vec<KeyCode>) -> Option<Self> {
        let mut key = None;
        let mut modifiers = Vec::new();

        for k in keys {
            if let Some(m) = Modifier::from_keycode(k) {
                modifiers.push(m);
            } else if let Some(k) = Key::from_keycode(k) {
                if key.is_none() {
                    key = Some(k);
                }
            }
        }

        match key {
            Some(key) => Some(KeyBinding { key, modifiers }),
            _ => None,
        }
    }
}

impl Serializable for KeyBinding {
    fn to_string(&self) -> String {
        self.modifiers
            .iter()
            .map(|x| SerializableEnum::to_string(x))
            .chain(std::iter::once(SerializableEnum::to_string(&self.key)))
            .collect::<Vec<_>>()
            .join(KEY_BINDING_SEPARATOR)
    }

    fn from_string(string: &str) -> Option<Self>
    where
        Self: Sized
    {
        let parts: Vec<&str> = string.split(KEY_BINDING_SEPARATOR).collect();
        let key = parts
            .last()
            .and_then(|&x| Key::from_string(x));
        let modifiers: Vec<Modifier> = parts
            .iter()
            .take(parts.len().saturating_sub(1))
            .filter_map(|&x| Modifier::from_string(x))
            .collect();
        key.map(|key| KeyBinding { key, modifiers })
    }
}

pub trait Command: SerializableEnum {

}

struct CommandBinding<T>
where
    T: Command,
{
    action: T,
    bindings: Vec<KeyBinding>
}

impl<T> CommandBinding<T>
where
    T: Command,
{
    fn is_pressed(&self, keys: &Res<ButtonInput<KeyCode>>) -> bool {
        self.bindings.iter().any(|binding| binding.is_pressed(keys))
    }
}

pub trait CommandHandler<T>
where
    T: Command,
{
    fn handle(&mut self, commands: &Vec<T>);
}
