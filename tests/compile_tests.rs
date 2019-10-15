#[test]
fn runner() {
    no_std_test();
}

#[rustversion::nightly]
fn no_std_test() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/no_std/without.rs");
    t.pass("tests/no_std/with.rs");
    panic!()
}

#[rustversion::not(nightly)]
fn no_std_test() {}
