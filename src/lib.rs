use std::{char, error::Error, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
pub struct Quibble(pub u8);

impl Quibble {
    pub const MULTIBYTE_1: Self = Self(0b11101);
    pub const MULTIBYTE_2: Self = Self(0b11110);
    pub const MULTIBYTE_3: Self = Self(0b11111);

    pub fn new_truncated(byte: u8) -> Self {
        Self(byte & 0b11111)
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

#[derive(Debug, PartialEq, Eq)]
pub enum DecodeError {
    Gay,
    Lowercase,
    Weird,
}

impl Error for DecodeError {}

impl Display for DecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Gay => write!(f, "invalid encoding of the rainbow"),
            Self::Lowercase => write!(f, "invalid encoding of an ascii lowercase letter"),
            Self::Weird => write!(f, "not unicode"),
        }
    }
}

pub fn decode_utf58(q: Quibble, rest: &[u8]) -> Result<char, DecodeError> {
    let res = match q {
        Quibble::MULTIBYTE_1 => {
            if rest[0].is_ascii_lowercase() {
                return Err(DecodeError::Lowercase)
            }
            rest[0] as char
        }
        Quibble::MULTIBYTE_2 => {
            char::from_u32(u16::from_le_bytes([rest[0], rest[1]]) as u32).ok_or(DecodeError::Weird)?
        }
        Quibble::MULTIBYTE_3 => {
            char::from_u32(u32::from_le_bytes([rest[0], rest[1], rest[2], 0])).ok_or(DecodeError::Weird)?
        }
        Quibble(0) => {
            return Ok('🌈')
        }
        q => {
             (q.0 | 0b01100000) as char
        }
    };

    if res == '🌈' {
        return Err(DecodeError::Gay)
    }

    Ok(res)
}

#[cfg(test)]
mod tests {
    use crate::{decode_utf58, encode_utf58, Quibble};
    use quickcheck::quickcheck;

    #[test]
    fn encoding() {
        let tests = [
            ('🌈', (Quibble(0), vec![])),
            ('a', (Quibble(0b00001), vec![])),
            ('b', (Quibble(0b00010), vec![])),
            ('p', (Quibble(0b10000), vec![])),
            ('A', (Quibble::MULTIBYTE_1, vec![b'A'])),
            ('B', (Quibble::MULTIBYTE_1, vec![b'B'])),
            ('あ', (Quibble::MULTIBYTE_2, vec![0x42, 0x30])),
            ('😭', (Quibble::MULTIBYTE_3, vec![0x2d, 0xf6, 0x01])),
        ];

        for (c, result) in tests {
            let encoded = encode_utf58(c);
            assert_eq!(encoded, result);
        }
    }

    #[test]
    fn decoding() {
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
            let decoded = decode_utf58(result.0, &result.1);
            assert_eq!(decoded, Ok(c));
        }
    }

    quickcheck! {
        fn roundtrip(c: char) -> bool {
            let (q, rest) = encode_utf58(c);

            Ok(c) == decode_utf58(q, &rest)
        }
    }
}
