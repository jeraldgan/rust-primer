# Strings

Strings were a confusing point for me when I started learning Rust without any systems-language experience.

Generally, there are two types of strings in Rust you'll encounter

`&str` and `String`

Think of

- `&str` as a fixed-length array of bytes
- `String` as a variable-length array of bytes (usually UTF-8)

## Why?

- Performance
- C interop
