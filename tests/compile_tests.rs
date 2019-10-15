#[rustversion::attr(not(nightly), ignore)]
#[test]
fn no_std() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/no_std/without.rs");
    t.pass("tests/no_std/with.rs");
}
