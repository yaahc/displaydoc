use displaydoc::Display;

#[derive(Display)]
#[doc = include_str!("non_string_doc_expr.txt")]
struct FakeType;

fn main() {}
