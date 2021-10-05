use displaydoc::Display;

#[derive(Display)]
enum EmptyInside {}

static_assertions::assert_impl_all!(label; EmptyInside, core::fmt::Display);
