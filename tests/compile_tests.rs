#[allow(unused_attributes)]
#[rustversion::attr(not(nightly), ignore)]
#[cfg_attr(feature = "std", ignore)]
#[test]
fn no_std() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/no_std/without.rs");
    t.pass("tests/no_std/with.rs");
}
