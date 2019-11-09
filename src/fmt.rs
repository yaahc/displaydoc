use crate::attr::Display;
use proc_macro2::TokenStream;
use quote::quote_spanned;
use syn::{Ident, LitStr};

impl Display {
    // Transform `"error {var}"` to `"error {}", var`.
    pub fn expand_shorthand(&mut self) {
        let span = self.fmt.span();
        let fmt = self.fmt.value();
        let mut read = fmt.as_str();
        let mut out = String::new();
        let mut args = TokenStream::new();

        while let Some(brace) = read.find('{') {
            out += &read[..=brace];
            read = &read[brace + 1..];

            // skip cases where we find a {{
            if read.starts_with('{') {
                out.push('{');
                read = &read[1..];
                continue;
            }

            let next = match read.chars().next() {
                Some(next) => next,
                None => return,
            };

            let var = match next {
                '0'..='9' => take_int(&mut read),
                'a'..='z' | 'A'..='Z' | '_' => take_ident(&mut read),
                _ => return,
            };

            let ident = Ident::new(&var, span);

            let next = match read.chars().next() {
                Some(next) => next,
                None => return,
            };

            let arg = if core::cfg!(feature = "std") && next == '}' {
                quote_spanned!(span=> , (&#ident).get_display())
            } else {
                quote_spanned!(span=> , #ident)
            };

            args.extend(arg);
        }

        out += read;
        self.fmt = LitStr::new(&out, self.fmt.span());
        self.args = args;
    }
}

fn take_int(read: &mut &str) -> String {
    let mut int = String::new();
    int.push('_');
    for (i, ch) in read.char_indices() {
        match ch {
            '0'..='9' => int.push(ch),
            _ => {
                *read = &read[i..];
                break;
            }
        }
    }
    int
}

fn take_ident(read: &mut &str) -> String {
    let mut ident = String::new();
    for (i, ch) in read.char_indices() {
        match ch {
            'a'..='z' | 'A'..='Z' | '_' => ident.push(ch),
            _ => {
                *read = &read[i..];
                break;
            }
        }
    }
    ident
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use proc_macro2::Span;

    fn assert(input: &str, fmt: &str, args: &str) {
        let mut display = Display {
            fmt: LitStr::new(input, Span::call_site()),
            args: TokenStream::new(),
        };
        display.expand_shorthand();
        assert_eq!(fmt, display.fmt.value());
        assert_eq!(args, display.args.to_string());
    }

    #[test]
    fn test_expand() {
        assert("error {var}", "error {}", ", ( & var ) . get_display ( )");
        assert("fn main() {{ }}", "fn main() {{ }}", "");
        assert(
            "{v} {v:?} {0} {0:?}",
            "{} {:?} {} {:?}",
            ", ( & v ) . get_display ( ) , v , ( & _0 ) . get_display ( ) , _0",
        );

        // assert("The path {0.display()}", "The path {}", "0.display()");
        // assert("The path {0.display():?}", "The path {:?}", "0.display()");
    }
}
