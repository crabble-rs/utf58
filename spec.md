# UTF-58

This specification describes the UTF-58 (pronounced fifty-eight) encoding.
UTF-58 encodes an arbitrary Unicode[^Unicode] scalar values.

## Definitions

- Quibble: An 5-bit value. It can store values in the range from 0 (inclusive) to 32 (exclusive) 
- Byte: An 8-bit value. It can store values in the range from 0 (inclusive) to 256 (exclusive)
- Unicode Scalar Value: A Unicode codepoint


## Goals

The goal of UTF-58 is to encode a Unicode scalar value into a single quibble and up to 3 bytes.
On traditional 8-bit-based computers, this encoding leaves space for 3 bits to be used for extra operations.
Additionally, UTF-58 aims to be fully compatible with 5 bit alphabetical lowercase english letters (like in ASCII).

## Sequences

While this specification only describes the encoding for a single codepoint, arbitrary codepoints can be chained after another in a sequence to form a string.
In 8-bit based storage, this leaves 3 bits of extra space for every codepoint.

## Encoding

UTF-58 is a variable-length encoding. The initial quibble fully encodes the length of the encoded value.

The following table represents all possible values of the quibble:

| code  | meaning           |
| ----- | ----------------- |
| 00000 | rainbow (U+1F308) |
| 00001 | a                 |
| 00010 | b                 |
| 00011 | c                 |
| 00100 | d                 |
| 00101 | e                 |
| 00110 | f                 |
| 01000 | g                 |
| 01001 | h                 |
| 01010 | i                 |
| 01011 | j                 |
| 01100 | k                 |
| 01101 | l                 |
| 01110 | m                 |
| 01111 | n                 |
| 10000 | o                 |
| 10001 | p                 |
| 10010 | q                 |
| 10011 | r                 |
| 10100 | s                 |
| 10101 | t                 |
| 10110 | u                 |
| 11000 | v                 |
| 11001 | w                 |
| 11010 | x                 |
| 11011 | y                 |
| 11100 | z                 |
| 11101 | cont 1 more bytes |
| 11110 | cont 2 more bytes | 
| 11111 | cont 3 more bytes |

The last 3 quibbles signal a *continuation*, meaning that bytes will be used.
The length of the bytes depends on the value of the scalar value.

If the scalar value is smaller than 256, it is encoded as a single byte, with 11101 as the quibble.
If the scalar value is bigger than 255 but smaller than 65536, it is encoded as two bytes, with 11110 as the quibble.
The bytes are encoded as little-endian, meaning the least significant byte comes first.
If the scalar value is bigger than 65535, it is encoded as three bytes, with 11111 as the quibble.
The bytes are also encoded as little-endian, meaning the least significant byte is first, while the most significant byte comes last.
As scalar values can only be up to 24 bits, this is sufficient.

## Canonical Encodings

As the rainbow (U+1F308) and the lowercase ASCII letters (a-z) are encoded in a special way, but are also valid Unicode codepoints, there are two ways to encode them.
The single-quibble encoding of these codepoints is considered to be the *canonical* encoding, and MUST be used to encode these codepoints.
Encoding the rainbow (U+1F308) or the lowercase ASCII letters in their non-canoinical encoding (so, with quibble 11101 or 11111) is invalid.
The behavior of the decoder is unspecified in this case. Decoders MAY report errors or treat it as the equivalent codepoints, but SHOULD NOT crash.

## References

[^Unicode]: https://unicode.org
