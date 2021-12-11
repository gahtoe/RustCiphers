pub fn caesar(plaintext: &str, shift: u8) -> String {
    plaintext.chars().map(|c| {
        if c.is_ascii_alphabetic() {
           let x = if c.is_ascii_lowercase() { b'a' } else {  b'A' };
           (x + (c as u8 + shift - x) % 26) as char
        } else {
            c
        }
    })
    .collect()
}