//! This library provides a convenient derive macro for the standard library's
//! [`core::fmt::Display`] trait.
//!
//! [`core::fmt::Display`]: https://doc.rust-lang.org/std/fmt/trait.Display.html
//!
//! ```toml
//! [dependencies]
//! displaydoc = "0.2"
//! ```
//!
//! *Compiler support: requires rustc 1.45+*
//!
//! <br>
//!
//! ## Example
//!
//! ```rust
//! use std::io;
//! use displaydoc::Display;
//! use thiserror::Error;
//!
//! #[derive(Display, Error, Debug)]
//! pub enum DataStoreError {
//!     /// data store disconnected
//!     Disconnect(#[source] io::Error),
//!     /// the data for key `{0}` is not available
//!     Redaction(String),
//!     /// invalid header (expected {expected:?}, found {found:?})
//!     InvalidHeader {
//!         expected: String,
//!         found: String,
//!     },
//!     /// unknown data store error
//!     Unknown,
//! }
//! ```
//!
//! <br>
//!
//! ## Details
//!
//! - A `Display` impl is generated for your type if you provide doc comment
//!   messages on the struct or each variant of your enum, as shown above in the
//!   example.
//!
//!   The messages support a shorthand for interpolating fields from the error.
//!
//!     - `/// {var}` ⟶ `write!("{}", self.var)`
//!     - `/// {0}` ⟶ `write!("{}", self.0)`
//!     - `/// {var:?}` ⟶ `write!("{:?}", self.var)`
//!     - `/// {0:?}` ⟶ `write!("{:?}", self.0)`
//!
//! - Two optional attributes can be added to your types next to the derive:
//!
//!     - `#[ignore_extra_doc_attributes]` makes the macro ignore any doc
//!       comment attributes (or `///` lines) after the first. Multi-line
//!       comments using `///` are otherwise treated as an error, so use this
//!       attribute or consider switching to block doc comments (`/** */`).
//!
//!     - `#[prefix_enum_doc_attributes]` combines the doc comment message on
//!       your enum itself with the messages for each variant, in the format
//!       “enum: variant”. When added to an enum, the doc comment on the enum
//!       becomes mandatory. When added to any other type, it has no effect.
//!
//! - In case you want to have an independent doc comment, the
//!   `#[displaydoc("...")` atrribute may be used on the variant or struct to
//!   override it.
//!
//! <br>
//!
//! ## FAQ
//!
//! 1. **Is this crate `no_std` compatible?**
//!     * Yes! This crate implements the `core::fmt::Display` trait not the `std::fmt::Display` trait so it should work in `std` and `no_std` environments. Just add `default-features = false`.
//!
//! 2. **Does this crate work with `Path` and `PathBuf` via the `Display` trait?**
//!     * Yuuup. This crate uses @dtolnay's [autoref specialization technique](https://github.com/dtolnay/case-studies/blob/master/autoref-specialization/README.md) to add a special trait for types to get the display impl, it then specializes for `Path` and `PathBuf` and when either of these types are found it calls `self.display()` to get a `std::path::Display<'_>` type which can be used with the Display format specifier!
#![doc(html_root_url = "https://docs.rs/displaydoc/0.2.3")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(
    rust_2018_idioms,
    unreachable_pub,
    bad_style,
    const_err,
    dead_code,
    improper_ctypes,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    private_in_public,
    unconditional_recursion,
    unused,
    unused_allocation,
    unused_comparisons,
    unused_parens,
    while_true
)]
#![allow(clippy::try_err)]

#[allow(unused_extern_crates)]
extern crate proc_macro;

mod attr;
mod expand;
mod fmt;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

/// Derive macro for implementing `Display` via doc comment attributes
#[proc_macro_derive(
    Display,
    attributes(ignore_extra_doc_attributes, prefix_enum_doc_attributes, displaydoc)
)]
pub fn derive_error(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand::derive(&input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
