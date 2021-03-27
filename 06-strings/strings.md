# Strings

Strings were a confusing point for me when I started learning Rust without any systems-language experience.

Generally, there are two types of strings in Rust you'll encounter

`&str` (slice) and `String`

Think of

- `&str` as a fixed-length array of bytes
  - Like a C++ `char*`, but UTF-8

- `String` as a variable-length array of bytes (usually UTF-8)
  - Has a underlying buffer, owns the buffer
  - Like a C++ `std::String`

## Why?

- Performance
- C interop
