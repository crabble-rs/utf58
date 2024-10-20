# UTF-58

A UTF-58 encoder and decoder.
UTF-58 (pronounced fifty-eight) is an encoding for arbitrary Unicode codepoints that uses an initial 5 bits (called a quibble),
and then up to 3 bytes.

This is useful when wanting to encode a Unicode codepoint in a way that leaves 3 bits of space for additional data.

UTF-58 is kinda ASCII-compatible (as in, the first quibble represents the truncated ASCII value) for lowercase `a`-`z`.

For more information, check the [official specification](https://github.com/crabble-rs/utf58/blob/main/spec.md).
