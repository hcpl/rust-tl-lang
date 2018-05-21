use synom::Synom;
use utils::{is_hex_digit, u32_from_hex_str};


/// A 32-bit number which identifies a TL combinator.
#[derive(Debug)]
pub struct Id {
    id: u32,
}

impl Id {
    pub fn new(id: u32) -> Self {
        Id { id }
    }
}

impl Synom for Id {
    named!(parse_str(&str) -> Id, do_parse!(
        // Doesn't work for `storage.fileJpeg#7efe0e = storage.FileType;`
        //id: map_res!(take_while_m_n!(8, 8, is_hex_digit), u32_from_hex_str) >>
        id: map_res!(take_while!(is_hex_digit), u32_from_hex_str) >>

        (Id { id })
    ));
}
