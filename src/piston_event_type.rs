
extern crate piston;

use {
    Key,
    EventType,
};

impl Key for piston::keyboard::Key {
    fn id(&self) -> uint {
        *self as uint
    }
}

impl Key for piston::mouse::Button {
    fn id(&self) -> uint {
        // add the last enum item in piston::keyboard::Key
        *self as uint + piston::keyboard::Space as uint + 1
    }
}

impl EventType for piston::event::Event {
    fn is_press_key(&self, key: &Key) -> bool {
        match *self {
            piston::event::KeyPressed(k) if k.id() == key.id() => {
                true
            },
            piston::event::MouseButtonPressed(k) if k.id() == key.id() => {
                true
            },
            _ => {
                false
            },
        }
    }
    fn is_release_key(&self, key: &Key) -> bool {
        match *self {
            piston::event::KeyReleased(k) if k.id() == key.id() => {
                true
            },
            piston::event::MouseButtonReleased(k) if k.id() == key.id() => {
                true
            },
            _ => {
                false
            }
        }
    }
}

