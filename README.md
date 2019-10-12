derive(DisplayDoc)
==================

[![Latest Version](https://img.shields.io/crates/v/thiserror.svg)](https://crates.io/crates/thiserror)
[![Rust Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/thiserror)

This library provides a convenient derive macro for the standard library's
[`std::fmt::Display`] trait.

[`std::fmt::Display`]: https://doc.rust-lang.org/std/fmt/trait.Display.html

```toml
[dependencies]
displaydoc = "1.0"
```

*Compiler support: requires rustc 1.31+*

<br>

## Example

```rust
use displaydoc::DisplayDoc;
use thiserror::Error;

#[derive(DisplayDoc, Error, Debug)]
pub enum DataStoreError {
    /// data store disconnected
    Disconnect(#[source] io::Error),
    /// the data for key `{0}` is not available
    Redaction(String),
    /// invalid header (expected {expected:?}, found {found:?})
    InvalidHeader {
        expected: String,
        found: String,
    },
    /// unknown data store error
    Unknown,
}
```

<br>

## Details

- Thiserror deliberately does not appear in your public API. You get the same
  thing as if you had written an implementation of `std::error::Error` by hand,
  and switching from handwritten impls to thiserror or vice versa is not a
  breaking change.

- Errors may be enums, structs with named fields, tuple structs, or unit
  structs.

- A `Display` impl is generated for your type if you provide doc comment
  messages on the struct or each variant of your enum, as shown above in the
  example.

  The messages support a shorthand for interpolating fields from the error.

    - `/// {var}` ⟶ `write!("{}", self.var)`
    - `/// {0}` ⟶ `write!("{}", self.0)`
    - `/// {var:?}` ⟶ `write!("{:?}", self.var)`
    - `/// {0:?}` ⟶ `write!("{:?}", self.0)`

<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>

