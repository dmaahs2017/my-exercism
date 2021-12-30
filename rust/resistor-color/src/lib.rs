use int_enum::IntEnum;
use enum_iterator::IntoEnumIterator;
#[repr(usize)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum, IntoEnumIterator)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}

pub fn color_to_value(_color: ResistorColor) -> usize {
    _color.int_value()
}

pub fn value_to_color_string(value: usize) -> String {
    use ResistorColor::*;
    if let Ok(color) = ResistorColor::from_int(value) {
        match color {
            Black => "Black".to_string(),
            Brown => "Brown".to_string(),
            Red => "Red".to_string(),
            Orange  => "Orange".to_string(),
            Yellow  => "Yellow".to_string(),
            Green  => "Green".to_string(),
            Blue  => "Blue".to_string(),
            Violet  => "Violet".to_string(),
            Grey  => "Grey".to_string(),
            White  => "White".to_string(),
        }
    } else {
        "value out of range".to_string()
    }
}

pub fn colors() -> Vec<ResistorColor> {
    ResistorColor::into_enum_iter().collect()
}
