pub struct Quibble(pub u8);

impl Quibble {
    pub const MULTIBYTE_1: Self = Self(0b11101);
    pub const MULTIBYTE_2: Self = Self(0b11110);
    pub const MULTIBYTE_3: Self = Self(0b11111);

    pub fn new_truncated(byte: u8) -> Self {
        Self(byte & 0b1111)
    }
}

pub fn encode_utf58(c: char) -> (Quibble, Vec<u8>) {
    if c == '🌈' {
        (Quibble(0), vec![])
    } else if matches!('a'..='z', c) {
        (Quibble::new_truncated(c as u8), vec![])
    } else {
        let b = (c as u32).to_le_bytes();
        assert_eq!(b[3], 0);
        if b[2] == 0 {
            if b[1] == 0 {
                (Quibble::MULTIBYTE_1, vec![b[0]])
            } else {
                (Quibble::MULTIBYTE_2, vec![b[0], b[1]])
            }
        } else {
            (Quibble::MULTIBYTE_3, vec![b[0], b[1], b[2]])
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Quibble;

    #[test]
    fn encoding() {
        let tests = [
            ('🌈', (0, vec![])),
            ('a', (0b00001, vec![])),
            ('b', (0b00010, vec![])),
            ('A', (Quibble::MULTIBYTE_1.0, vec![b'A'])),
            ('B', (Quibble::MULTIBYTE_1.0, vec![b'B'])),
            ('ö', (Quibble::MULTIBYTE_1.0, vec![b'B'])),

        ];
    }
}
