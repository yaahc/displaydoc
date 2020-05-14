use anyhow::anyhow;
use displaydoc::Display;
use std::error::Error as _;

use thiserror::Error;

fn assert_display<T: std::fmt::Display>(input: T, expected: &'static str) {
    let out = format!("{}", input);
    assert_eq!(expected, out);
}

#[test]
fn prioritize_thiserror_transparent_for_enum() {
    #[displaydoc(with_thiserror = true)]
    #[derive(Display, Error, Debug)]
    enum MyError {
        /// Doc for Variant.
        #[error(transparent)]
        Variant(anyhow::Error),
    }

    let var = MyError::Variant(anyhow!("inner").context("outer"));
    assert_display(&var, "outer");
    assert_eq!(var.source().unwrap().to_string(), "inner")
}

#[test]
fn prioritize_thiserror_transparent_for_struct() {
    #[displaydoc(with_thiserror = true)]
    #[derive(Display, Error, Debug)]
    #[error(transparent)]
    struct MyError {
        /// Doc for Variant.
        variant: anyhow::Error,
    }

    let var = MyError {
        variant: anyhow!("inner").context("outer"),
    };
    assert_display(&var, "outer");
    assert_eq!(var.source().unwrap().to_string(), "inner")
}

#[test]
fn prioritize_thiserror_errordoc_for_enum() {
    #[displaydoc(with_thiserror = true)]
    #[derive(Display, Error, Debug)]
    enum MyError {
        /// I'm not a doc for Variant
        #[error("I'm a doc for Variant")]
        Variant,
    }
    assert_display(MyError::Variant, "I'm a doc for Variant");
}

#[test]
fn prioritize_thiserror_errordoc_for_struct() {
    #[displaydoc(with_thiserror = true)]
    #[derive(Display, Error, Debug)]
    #[error("I'm a doc for MyError")]
    struct MyError {
        /// I'm not a doc for Variant
        variant: u8,
    }
    assert_display(MyError { variant: 42 }, "I'm a doc for MyError");
}
