#[allow(clippy::wrong_self_convention)]
pub fn to_lower(s: String) -> String {
    s.chars()
    .map(|c| {
        if c as u8 >= b'A' && c as u8 <= b'Z' {
            (c as u8 + (b'a' - b'A')) as char
        } else {
            c
        }
    })
    .collect()
}
