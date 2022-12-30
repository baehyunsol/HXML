// https://www.w3.org/TR/xml/#NT-NameStartChar
pub fn is_name_start_char(c: &u16) -> bool {
    is_alpha_low(c) || is_alpha_cap(c) || *c == ':' as u16 || *c == '_' as u16 ||
    0xc0 <= *c && *c <= 0xd6 ||
    0xd8 <= *c && *c <= 0xf6 ||
    0xf8 <= *c && *c <= 0x2ff ||
    0x370 <= *c && *c <= 0x37d ||
    0x37f <= *c && *c <= 0x1fff ||
    0x200c <= *c && *c <= 0x200d ||
    0x2070 <= *c && *c <= 0x218f ||
    0x2c00 <= *c && *c <= 0x2fef ||
    0x3001 <= *c && *c <= 0xd7ff ||
    0xf900 <= *c && *c <= 0xfdcf ||
    0xfdf0 <= *c && *c <= 0xfffd
    // || 0x10000 <= *c && *c <= 0xeffff  // not u16
}

// https://www.w3.org/TR/xml/#NT-NameChar
pub fn is_name_char(c: &u16) -> bool {
    is_name_start_char(c) || is_numeric(c) || *c == '-' as u16 || *c == '.' as u16 || *c == 0xb7 ||
    0x300 <= *c && *c <= 0x36f ||
    0x203f <= *c && *c <= 0x2040
}

// https://www.w3.org/TR/xml/#NT-Char
pub fn is_valid_char(c: &u16) -> bool {
    *c == 9 || *c == 10 || *c == 13 ||
    0x20 <= *c && *c <= 0xd7ff ||
    0xe000 <= *c && *c <= 0xfffd
    //  || 0x10000 <= *c && *c <= 0x10ffff  // not u16
}

// https://www.w3.org/TR/xml/#NT-S
#[inline]
pub fn is_whitespace(c: &u16) -> bool {
    *c == 0x20 ||
    *c == 0x9  ||
    *c == 0xa  ||
    *c == 0xd
}

#[inline]
pub fn is_alpha_low(c: &u16) -> bool {
    'a' as u16 <= *c && *c <= 'z' as u16
}

#[inline]
pub fn is_alpha_cap(c: &u16) -> bool {
    'A' as u16 <= *c && *c <= 'Z' as u16
}

#[inline]
pub fn is_numeric(c: &u16) -> bool {
    '0' as u16 <= *c && *c <= '9' as u16
}

#[inline]
pub fn is_hexadecimal(c: &u16) -> bool {
    is_numeric(c) ||
    'a' as u16 <= *c && *c <= 'f' as u16 ||
    'A' as u16 <= *c && *c <= 'F' as u16
}