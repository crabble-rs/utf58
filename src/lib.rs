use std::{char, error::Error, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
pub struct Quibble(u8);

impl Quibble {
    pub const MULTIBYTE_1: Self = Self(0b11101);
    pub const MULTIBYTE_2: Self = Self(0b11110);
    pub const MULTIBYTE_3: Self = Self(0b11111);

    #[inline]
    pub fn new_truncated(byte: u8) -> Self {
        Self(byte & 0b11111)
    }
}

pub trait Utf58Ext: Copy {
    fn encode_utf58(self, rest: &mut [u8; 3]) -> (Quibble, usize);
    fn len_utf58(self) -> usize;

    /// Calculates the number of segments in the encoding of a UTF-58 char.
    ///
    /// 1 means a single quibble, any number above that (up to 4) means a quibble and some number of
    /// bytes.
    fn decode_utf58(q: Quibble, rest: &[u8]) -> Result<char, DecodeError>;
}

impl Utf58Ext for char {
    fn encode_utf58(self, rest: &mut [u8; 3]) -> (Quibble, usize) {
        if self == '🌈' {
            (Quibble(0), 0)
        } else if self.is_ascii_lowercase() {
            (Quibble::new_truncated(self as u8), 0)
        } else {
            let b = (self as u32).to_le_bytes();
            assert_eq!(b[3], 0);
            if b[2] == 0 {
                if b[1] == 0 {
                    rest[0] = b[0];
                    (Quibble::MULTIBYTE_1, 1)
                } else {
                    rest[0] = b[0];
                    rest[1] = b[1];
                    (Quibble::MULTIBYTE_2, 2)
                }
            } else {
                rest[0] = b[0];
                rest[1] = b[1];
                rest[2] = b[2];
                (Quibble::MULTIBYTE_3, 3)
            }
        }
    }

    fn len_utf58(self) -> usize {
        if self == '🌈' || self.is_ascii_lowercase() {
            1
        } else {
            let b = (self as u32).to_le_bytes();
            assert_eq!(b[3], 0);
            if b[2] == 0 {
                if b[1] == 0 {
                    2
                } else {
                    3
                }
            } else {
                4
            }
        }
    }

    fn decode_utf58(q: Quibble, rest: &[u8]) -> Result<char, DecodeError> {
        let res = match q {
            Quibble::MULTIBYTE_1 => {
                if rest[0].is_ascii_lowercase() {
                    return Err(DecodeError::Lowercase);
                }
                rest[0] as char
            }
            Quibble::MULTIBYTE_2 => char::from_u32(u16::from_le_bytes([rest[0], rest[1]]) as u32)
                .ok_or(DecodeError::Weird)?,
            Quibble::MULTIBYTE_3 => {
                char::from_u32(u32::from_le_bytes([rest[0], rest[1], rest[2], 0]))
                    .ok_or(DecodeError::Weird)?
            }
            Quibble(0) => return Ok('🌈'),
            q => (q.0 | 0b01100000) as char,
        };

        if res == '🌈' {
            return Err(DecodeError::Gay);
        }

        Ok(res)
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

#[cfg(test)]
mod tests {
    use crate::{Quibble, Utf58Ext};
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

        for (c, (q, r)) in tests {
            let mut buf = [0; 3];
            let (encoded, len) = c.encode_utf58(&mut buf);
            assert_eq!(encoded, q);
            assert_eq!(r, &buf[..len]);
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
            let decoded = char::decode_utf58(result.0, &result.1);
            assert_eq!(decoded, Ok(c));
        }
    }

    quickcheck! {
        fn roundtrip(c: char) -> bool {
            let mut rest = [0; 3];
            let (q, l) = c.encode_utf58(&mut rest);

            Ok(c) == char::decode_utf58(q, &rest[..l])
        }

        fn len(c: char) -> bool {
            let mut rest = [0; 3];
            let (_, l) = c.encode_utf58(&mut rest);
            let actual_len = 1 + l;

            c.len_utf58() == actual_len
        }
    }
}
