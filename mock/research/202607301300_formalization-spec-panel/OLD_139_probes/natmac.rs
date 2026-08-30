// The crossing, as an expansion. No table, no cap, no feature gate, no flag.
// Turns a written decimal literal into 137's little-endian binary structural nat.
extern crate proc_macro;
use proc_macro::{TokenStream, TokenTree};

fn structure(mut n: u128, out: &mut String) {
    // D1<T> is 2*T+1, D0<T> is 2*T, Term is 0. Outermost digit is the LSB.
    let mut depth = 0usize;
    if n == 0 {
        out.push_str("Term");
        return;
    }
    while n > 0 {
        out.push_str(if n & 1 == 1 { "D1<" } else { "D0<" });
        n >>= 1;
        depth += 1;
    }
    out.push_str("Term");
    for _ in 0..depth {
        out.push('>');
    }
}

/// `nat!(13)` expands to `D1<D0<D1<D1<Term>>>>`.
#[proc_macro]
pub fn nat(input: TokenStream) -> TokenStream {
    // a `$i:literal` fragment arrives as an invisible-delimited Group, so flatten.
    fn first_lit(ts: TokenStream) -> Option<String> {
        for t in ts {
            match t {
                TokenTree::Literal(l) => return Some(l.to_string()),
                TokenTree::Group(g) => {
                    if let Some(s) = first_lit(g.stream()) {
                        return Some(s);
                    }
                }
                _ => {}
            }
        }
        None
    }
    let raw = first_lit(input).expect("nat! wants an integer literal");
    let n: u128 = raw
        .replace('_', "")
        .parse()
        .expect("nat! wants a plain decimal integer literal");
    let mut s = String::new();
    structure(n, &mut s);
    s.parse().unwrap()
}
