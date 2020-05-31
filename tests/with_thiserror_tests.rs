use anyhow::anyhow;
use displaydoc::Display;
use std::error::Error as StdError;
use std::io;
use thiserror::Error;

fn assert_display<T: std::fmt::Display>(input: T, expected: &'static str) {
    let out = format!("{}", input);
    assert_eq!(expected, out);
}

#[test]
fn test_transparent_for_enum() {
    #[derive(Display, Error, Debug)]
    enum MyError {
        /// Doc for Variant.
        #[display(transparent)]
        Variant(anyhow::Error),
    }

    let var = MyError::Variant(anyhow!("inner").context("outer"));
    assert_display(&var, "outer");
    assert_eq!(var.source().unwrap().to_string(), "inner")
}

#[test]
fn test_transparent_for_struct() {
    #[derive(Display, Error, Debug)]
    #[error(transparent)]
    struct Error(ErrorKind);

    #[derive(Display, Error, Debug)]
    enum ErrorKind {
        #[display("E0")]
        E0,
        #[display("E1")]
        E1(#[from] io::Error),
    }

    let error = Error(ErrorKind::E0);
    assert_eq!("E0", error.to_string());
    assert!(error.source().is_none());

    let io = io::Error::new(io::ErrorKind::Other, "oh no!");
    let error = Error(ErrorKind::from(io));
    assert_eq!("E1", error.to_string());
    error.source().unwrap().downcast_ref::<io::Error>().unwrap();
}

#[test]
fn test_errordoc_for_enum() {
    #[derive(Display, Error, Debug)]
    enum MyError {
        /// I'm not a doc for Variant
        #[display("I'm a doc for Variant")]
        Variant,
    }
    assert_display(MyError::Variant, "I'm a doc for Variant");
}

#[test]
fn test_errordoc_for_struct() {
    #[derive(Display, Error, Debug)]
    #[display("I'm a doc for MyError")]
    struct MyError {
        /// I'm not a doc for MyError
        variant: u8,
    }
    assert_display(MyError { variant: 42 }, "I'm a doc for MyError");
}

#[test]
fn test_thiserror_implicit_and_source_works() {
    #[derive(Display, Error, Debug)]
    #[error("implicit source")]
    struct ImplicitSource {
        source: io::Error,
    }

    #[derive(Display, Error, Debug)]
    #[error("explicit source")]
    struct ExplicitSource {
        source: String,
        #[source]
        io: io::Error,
    }

    let io = io::Error::new(io::ErrorKind::Other, "oh no!");
    let error = ImplicitSource { source: io };
    error.source().unwrap().downcast_ref::<io::Error>().unwrap();

    let io = io::Error::new(io::ErrorKind::Other, "oh no!");
    let error = ExplicitSource {
        source: String::new(),
        io,
    };
    error.source().unwrap().downcast_ref::<io::Error>().unwrap();
}
