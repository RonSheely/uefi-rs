// SPDX-License-Identifier: MIT OR Apache-2.0

//! HII Protocols

pub mod config;
pub mod database;

use crate::{Char16, Guid, newtype_enum};

pub type HiiHandle = *mut core::ffi::c_void;

/// EFI_HII_PACKAGE_HEADER
#[derive(Debug)]
#[repr(C)]
pub struct HiiPackageHeader {
    pub length_and_type: u32,
    pub data: [u8; 0],
}

/// EFI_HII_PACKAGE_LIST_HEADER
#[derive(Debug)]
#[repr(C)]
pub struct HiiPackageListHeader {
    pub package_list_guid: Guid,
    pub package_length: u32,
}

newtype_enum! {
    /// EFI_KEY: A physical key on a keyboard.
    pub enum Key: u32 => {
        LCTRL = 0,
        A0 = 1,
        LALT = 2,
        SPACEBAR = 3,
        A2 = 4,
        A3 = 5,
        A4 = 6,
        RCTRL = 7,
        LEFT_ARROW = 8,
        DOWN_ARROW = 9,
        RIGHT_ARROW = 10,
        ZERO = 11,
        PERIOD = 12,
        ENTER = 13,
        LSHIFT = 14,
        B0 = 15,
        B1 = 16,
        B2 = 17,
        B3 = 18,
        B4 = 19,
        B5 = 20,
        B6 = 21,
        B7 = 22,
        B8 = 23,
        B9 = 24,
        B10 = 25,
        RSHIFT = 26,
        UP_ARROW = 27,
        ONE = 28,
        TWO = 29,
        THREE = 30,
        CAPS_LOCK = 31,
        C1 = 32,
        C2 = 33,
        C3 = 34,
        C4 = 35,
        C5 = 36,
        C6 = 37,
        C7 = 38,
        C8 = 39,
        C9 = 40,
        C10 = 41,
        C11 = 42,
        C12 = 43,
        FOUR = 44,
        FIVE = 45,
        SIX = 46,
        PLUS = 47,
        TAB = 48,
        D1 = 49,
        D2 = 50,
        D3 = 51,
        D4 = 52,
        D5 = 53,
        D6 = 54,
        D7 = 55,
        D8 = 56,
        D9 = 57,
        D10 = 58,
        D11 = 59,
        D12 = 60,
        D13 = 61,
        DEL = 62,
        END = 63,
        PG_DN = 64,
        SEVEN = 65,
        EIGHT = 66,
        NINE = 67,
        E0 = 68,
        E1 = 69,
        E2 = 70,
        E3 = 71,
        E4 = 72,
        E5 = 73,
        E6 = 74,
        E7 = 75,
        E8 = 76,
        E9 = 77,
        E10 = 78,
        E11 = 79,
        E12 = 80,
        BACK_SPACE = 81,
        INS = 82,
        HOME = 83,
        PG_UP = 84,
        NLCK = 85,
        SLASH = 86,
        ASTERISK = 87,
        MINUS = 88,
        ESC = 89,
        F1 = 90,
        F2 = 91,
        F3 = 92,
        F4 = 93,
        F5 = 94,
        F6 = 95,
        F7 = 96,
        F8 = 97,
        F9 = 98,
        F10 = 99,
        F11 = 100,
        F12 = 101,
        PRINT = 102,
        SLCK = 103,
        PAUSE = 104,
        INTL0 = 105,
        INTL1 = 106,
        INTL2 = 107,
        INTL3 = 108,
        INTL4 = 109,
        INTL5 = 110,
        INTL6 = 111,
        INTL7 = 112,
        INTL8 = 113,
        INTL9 = 114,
    }
}

// NOTE: This has no associated type in UEFI; They are all top-level defines.
newtype_enum! {
    /// Key modifier values
    pub enum Modifier: u16 => {
        NULL = 0x0000,
        LEFT_CONTROL = 0x0001,
        RIGHT_CONTROL = 0x0002,
        LEFT_ALT = 0x0003,
        RIGHT_ALT = 0x0004,
        ALT_GR = 0x0005,
        INSERT = 0x0006,
        DELETE = 0x0007,
        PAGE_DOWN = 0x0008,
        PAGE_UP = 0x0009,
        HOME = 0x000A,
        END = 0x000B,
        LEFT_SHIFT = 0x000C,
        RIGHT_SHIFT = 0x000D,
        CAPS_LOCK = 0x000E,
        NUM_LOCK = 0x000F,
        LEFT_ARROW = 0x0010,
        RIGHT_ARROW = 0x0011,
        DOWN_ARROW = 0x0012,
        UP_ARROW = 0x0013,
        NS_KEY = 0x0014,
        NS_KEY_DEPENDENCY = 0x0015,
        FUNCTION_KEY_ONE = 0x0016,
        FUNCTION_KEY_TWO = 0x0017,
        FUNCTION_KEY_THREE = 0x0018,
        FUNCTION_KEY_FOUR = 0x0019,
        FUNCTION_KEY_FIVE = 0x001A,
        FUNCTION_KEY_SIX = 0x001B,
        FUNCTION_KEY_SEVEN = 0x001C,
        FUNCTION_KEY_EIGHT = 0x001D,
        FUNCTION_KEY_NINE = 0x001E,
        FUNCTION_KEY_TEN = 0x001F,
        FUNCTION_KEY_ELEVEN = 0x0020,
        FUNCTION_KEY_TWELVE = 0x0021,
        PRINT = 0x0022,
        SYS_REQUEST = 0x0023,
        SCROLL_LOCK = 0x0024,
        PAUSE = 0x0025,
        BREAK = 0x0026,
        LEFT_LOGO = 0x0027,
        RIGHT_LOGO = 0x0028,
        MENU = 0x0029,
    }
}

/// EFI_KEY_DESCRIPTOR
#[derive(Debug)]
#[repr(C)]
pub struct KeyDescriptor {
    pub key: Key,
    pub unicode: Char16,
    pub shifted_unicode: Char16,
    pub alt_gr_unicode: Char16,
    pub shifted_alt_gr_unicode: Char16,
    pub modifier: u16,
    pub affected_attribute: u16,
}
