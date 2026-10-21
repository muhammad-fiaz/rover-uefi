//! # HID
//!
//! Native support for the Human Interface Device (HID) class, as
//! described by the USB HID specification and the HID usage tables.
//!
//! The module holds the standard usage pages and usage identifiers, the
//! report descriptor parser and the report parser.

pub mod descriptor;
pub mod report;

pub use descriptor::{
    CollectionScope, CollectionType, Field, FieldFlags, FieldIter, FieldKind, Item, ItemType,
    MAX_COLLECTION_DEPTH, MAX_USAGES, MainItemType, ReportDescriptor,
};
pub use report::{DecodedField, FieldValue, MAX_FIELD_VALUES, ReportIter};

/// The generic desktop usage page.
pub const PAGE_GENERIC_DESKTOP: u16 = 0x01;

/// The simulation controls usage page.
pub const PAGE_SIMULATION: u16 = 0x02;

/// The keyboard usage page.
pub const PAGE_KEYBOARD: u16 = 0x03;

/// The gamepad usage page.
pub const PAGE_GAMEPAD: u16 = 0x05;

/// The keyboard and keypad usage page.
pub const PAGE_KEYBOARD_KEYPAD: u16 = 0x07;

/// The light emitting diode usage page.
pub const PAGE_LEDS: u16 = 0x08;

/// The button usage page.
pub const PAGE_BUTTON: u16 = 0x09;

/// The consumer usage page.
pub const PAGE_CONSUMER: u16 = 0x0C;

/// The digitizer usage page.
pub const PAGE_DIGITIZER: u16 = 0x0D;

/// The unicode usage page.
pub const PAGE_UNICODE: u16 = 0x10;

/// The vendor defined usage pages start here.
pub const PAGE_VENDOR_DEFINED: u16 = 0xFF00;

/// The pointer usage of the generic desktop page.
pub const USAGE_POINTER: u16 = 0x01;

/// The mouse usage of the generic desktop page.
pub const USAGE_MOUSE: u16 = 0x02;

/// The joystick usage of the generic desktop page.
pub const USAGE_JOYSTICK: u16 = 0x03;

/// The gamepad usage of the generic desktop page.
pub const USAGE_GAMEPAD: u16 = 0x04;

/// The keyboard usage of the generic desktop page.
pub const USAGE_KEYBOARD: u16 = 0x05;

/// The X axis usage of the generic desktop page.
pub const USAGE_X: u16 = 0x08;

/// The Y axis usage of the generic desktop page.
pub const USAGE_Y: u16 = 0x09;

/// The Z axis usage of the generic desktop page.
pub const USAGE_Z: u16 = 0x0A;

/// The Rx axis usage of the generic desktop page.
pub const USAGE_RX: u16 = 0x0B;

/// The Ry axis usage of the generic desktop page.
pub const USAGE_RY: u16 = 0x0C;

/// The Rz axis usage of the generic desktop page.
pub const USAGE_RZ: u16 = 0x0D;

/// The slider usage of the generic desktop page.
pub const USAGE_SLIDER: u16 = 0x0E;

/// The dial usage of the generic desktop page.
pub const USAGE_DIAL: u16 = 0x0F;

/// The wheel usage of the generic desktop page.
pub const USAGE_WHEEL: u16 = 0x10;

/// The first button of the button usage page.
pub const USAGE_BUTTON_1: u16 = 0x01;

/// The second button of the button usage page.
pub const USAGE_BUTTON_2: u16 = 0x02;

/// The third button of the button usage page.
pub const USAGE_BUTTON_3: u16 = 0x03;

/// The fourth button of the button usage page.
pub const USAGE_BUTTON_4: u16 = 0x04;

/// The fifth button of the button usage page.
pub const USAGE_BUTTON_5: u16 = 0x05;

/// The no event usage of the keyboard and keypad page.
pub const USAGE_NO_EVENT: u16 = 0x00;

/// The error roll over usage of the keyboard and keypad page.
pub const USAGE_ERROR_ROLL_OVER: u16 = 0x01;

/// The first letter key usage of the keyboard and keypad page.
pub const USAGE_KEY_A: u16 = 0x04;

/// The last letter key usage of the keyboard and keypad page.
pub const USAGE_KEY_Z: u16 = 0x1D;

/// The first digit key usage of the keyboard and keypad page.
pub const USAGE_KEY_1: u16 = 0x1E;

/// The last digit key usage of the keyboard and keypad page.
pub const USAGE_KEY_9: u16 = 0x26;

/// The zero digit key usage of the keyboard and keypad page.
pub const USAGE_KEY_0: u16 = 0x27;

/// The enter key usage of the keyboard and keypad page.
pub const USAGE_KEY_ENTER: u16 = 0x28;

/// The escape key usage of the keyboard and keypad page.
pub const USAGE_KEY_ESCAPE: u16 = 0x29;

/// The backspace key usage of the keyboard and keypad page.
pub const USAGE_KEY_BACKSPACE: u16 = 0x2A;

/// The tab key usage of the keyboard and keypad page.
pub const USAGE_KEY_TAB: u16 = 0x2B;

/// The space key usage of the keyboard and keypad page.
pub const USAGE_KEY_SPACE: u16 = 0x2C;

/// The caps lock key usage of the keyboard and keypad page.
pub const USAGE_KEY_CAPS_LOCK: u16 = 0x39;

/// The first function key usage of the keyboard and keypad page.
pub const USAGE_KEY_F1: u16 = 0x3A;

/// The last function key usage of the keyboard and keypad page.
pub const USAGE_KEY_F12: u16 = 0x45;

/// The left control modifier usage of the keyboard and keypad page.
pub const USAGE_MODIFIER_LEFT_CONTROL: u16 = 0xE0;

/// The left shift modifier usage of the keyboard and keypad page.
pub const USAGE_MODIFIER_LEFT_SHIFT: u16 = 0xE1;

/// The left alt modifier usage of the keyboard and keypad page.
pub const USAGE_MODIFIER_LEFT_ALT: u16 = 0xE2;

/// The left gui modifier usage of the keyboard and keypad page.
pub const USAGE_MODIFIER_LEFT_GUI: u16 = 0xE3;

/// The right control modifier usage of the keyboard and keypad page.
pub const USAGE_MODIFIER_RIGHT_CONTROL: u16 = 0xE4;

/// The right shift modifier usage of the keyboard and keypad page.
pub const USAGE_MODIFIER_RIGHT_SHIFT: u16 = 0xE5;

/// The right alt modifier usage of the keyboard and keypad page.
pub const USAGE_MODIFIER_RIGHT_ALT: u16 = 0xE6;

/// The right gui modifier usage of the keyboard and keypad page.
pub const USAGE_MODIFIER_RIGHT_GUI: u16 = 0xE7;

/// The consumer control usage of the consumer page.
pub const USAGE_CONSUMER_CONTROL: u16 = 0x01;

/// The play usage of the consumer page.
pub const USAGE_CONSUMER_PLAY: u16 = 0xB0;

/// The pause usage of the consumer page.
pub const USAGE_CONSUMER_PAUSE: u16 = 0xB1;

/// The record usage of the consumer page.
pub const USAGE_CONSUMER_RECORD: u16 = 0xB2;

/// The fast forward usage of the consumer page.
pub const USAGE_CONSUMER_FAST_FORWARD: u16 = 0xB3;

/// The rewind usage of the consumer page.
pub const USAGE_CONSUMER_REWIND: u16 = 0xB4;

/// The volume increment usage of the consumer page.
pub const USAGE_CONSUMER_VOLUME_INCREMENT: u16 = 0xE9;

/// The volume decrement usage of the consumer page.
pub const USAGE_CONSUMER_VOLUME_DECREMENT: u16 = 0xEA;

/// The mute usage of the consumer page.
pub const USAGE_CONSUMER_MUTE: u16 = 0xE2;
