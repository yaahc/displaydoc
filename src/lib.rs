extern crate proc_macro;

use proc_macro::TokenStream;
use watt::WasmMacro;

static MACRO: WasmMacro = WasmMacro::new(WASM);
static WASM: &[u8] = include_bytes!("displaydoc.wasm");

#[proc_macro_derive(Display)]
pub fn derive_error(input: TokenStream) -> TokenStream {
    MACRO.proc_macro("derive_error", input)
}
