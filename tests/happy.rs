use displaydoc::Display;

#[derive(Display)]
enum Happy {
    /// I really like Variant1
    Variant1,
    /// Variant2 is pretty swell 2
    Variant2,
    /// Variant3 is okay {sometimes}
    Variant3 { sometimes: &'static str },
    /**
     * Variant4 wants to have a lot of lines
     *
     * Lets see how this works out for it
     */
    Variant4,
    /// Variant5 just has {0} many problems
    /// but multi line comments aren't one of them
    Variant5(u32),
}

fn assert_display<T: std::fmt::Display>(input: T, expected: &'static str) {
    let out = format!("{}", input);
    assert_eq!(expected, out);
}

#[test]
fn does_it_print() {
    assert_display(Happy::Variant1, "I really like Variant1");
    assert_display(Happy::Variant2, "Variant2 is pretty swell 2");
    assert_display(Happy::Variant3 { sometimes: "hi" }, "Variant3 is okay hi");
    assert_display(
        Happy::Variant4,
        "Variant4 wants to have a lot of lines\n\n Lets see how this works out for it",
    );
    assert_display(Happy::Variant5(2), "Variant5 just has 2 many problems");
}
