// p6 (a): a minimal proc-macro crate, sysroot `proc_macro` only, no syn/quote.
// The shape file 61's notation vehicle established. Body deliberately small,
// because the question is what a consumer pays for *reaching* a proc-macro
// crate at all, not what a large one costs.
extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn bitfield_pm(input: TokenStream) -> TokenStream {
    // parse `Name : N { f : w , ... }` and emit derived offsets. Enough body to
    // be a real crate; the measurement is dominated by the crate kind.
    let toks: Vec<_> = input.into_iter().collect();
    let mut out = String::new();
    out.push_str("// emitted\n");
    for t in &toks {
        out.push_str(&format!("// {}\n", t));
    }
    out.parse().unwrap()
}
