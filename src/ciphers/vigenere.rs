pub fn vigenere(plaintext: &str, key: &str) -> String {
    let key: String = key.chars().filter(|&c| c.is_ascii_alphabetic()).collect();
    let key = key.to_ascii_lowercase();
    let key_len = key.len();
    if key_len == 0 {
        return String::from(plaintext);
    }

    let mut idx = 0;

    plaintext.chars().map(|c| {
        if c.is_ascii_alphabetic() {
            let x = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            let k = key.as_bytes()[idx % key_len] - b'a';
            idx += 1;
            (x + (c as u8 + k - x) % 26) as char
        } else {
            c
        }
    }).collect()
}