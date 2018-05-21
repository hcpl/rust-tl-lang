use std::num;


pub(crate) fn is_decimal_digit(c: char) -> bool {
    char::is_digit(c, 10)
}


pub(crate) fn is_hex_digit(c: char) -> bool {
    char::is_digit(c, 16)
}

pub(crate) fn u32_from_hex_str(s: &str) -> Result<u32, num::ParseIntError> {
    u32::from_str_radix(s, 16)
}
