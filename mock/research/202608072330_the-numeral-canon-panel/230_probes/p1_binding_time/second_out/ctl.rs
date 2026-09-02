// One `const fn` body, called at both evaluation sites with the same arguments.
//
// The free-standing consts in `matrix.rs` establish that the two sites disagree
// on an expression. This asks the sharper question for a design whose
// operations are `const fn` and whose consumers call them at both sites: does
// ONE function body, unchanged, behave differently depending on where it is
// called from?
//
// `CALL_SITE` is set by the caller. `const` builds the const-context arm, which
// must refuse if the two sites differ; `run` builds the runtime arm, which
// reports what it computed.
#[allow(dead_code)]
const fn saturating_looking_add(a: u8, b: u8) -> u8 {
    // Deliberately the naive body a design would write before deciding a
    // policy: no checked call, no wrapping call, just the operator.
    a + b
}

#[cfg(feature = "const_site")]
const OUT: u8 = saturating_looking_add(200, 55);

fn main() {
    #[cfg(feature = "const_site")]
    println!("const {}", OUT);
    #[cfg(not(feature = "const_site"))]
    {
        let a = std::hint::black_box(200u8);
        let b = std::hint::black_box(55u8);
        println!("run {}", saturating_looking_add(a, b));
    }
}
