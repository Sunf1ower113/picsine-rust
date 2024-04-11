#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CipherError {
    pub validation: bool,
    pub expected: String
}

impl CipherError {
    pub fn new(validation: bool, expected: String) -> CipherError {
        CipherError {
            validation,
            expected
        }
    }
}
pub fn cipher(original: &str, ciphered: &str) -> Option<Result<bool, CipherError>> {
    if original.is_empty() || ciphered.is_empty() {
        return None;
    }
    let mut chip = String::from("");
    for c in original.chars() {
        let n: u8 = u8::from(c as u8);
        let mut a: u8 = 0;
        if c.is_alphabetic() {
            if c.is_uppercase() {
                a = u8::from('A' as u8);
            } else {
                a = u8::from('a' as u8);
            }
            chip.push(char::from(25 - (n - a) % 25 + a))
        } else { chip.push(c) }
    }
    if chip != ciphered {
        return Some(Err(CipherError::new(false, chip)));
    }
    return Some(Ok(true))
}
