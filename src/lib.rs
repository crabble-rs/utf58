#[derive(Debug, PartialEq, Eq)]
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
    } else if matches!(c, 'a'..='z') {
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
            ('🌈', (Quibble(0), vec![])),
            ('a', (Quibble(0b00001), vec![])),
            ('b', (Quibble(0b00010), vec![])),
            ('A', (Quibble::MULTIBYTE_1, vec![b'A'])),
            ('B', (Quibble::MULTIBYTE_1, vec![b'B'])),
            ('あ', (Quibble::MULTIBYTE_2, vec![0x42, 0x30])),
            ('😭', (Quibble::MULTIBYTE_3, vec![0x2d, 0xf6, 0x01])),
        ];

        for (c, result) in tests {
            let encoded = super::encode_utf58(c);
            assert_eq!(encoded, result);
        }
    }
}
