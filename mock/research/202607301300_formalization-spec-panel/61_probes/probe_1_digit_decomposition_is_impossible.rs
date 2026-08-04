// Does macro_rules! have ANY route from an atomic literal token (`37`) to its
// constituent digit tokens (`3`, `7`)? Three attempts, all through legitimate
// macro_rules! machinery, none of them token-hacks.

// Attempt A: match the literal, then try to further match it as two `tt`s.
// A single `:literal` fragment, once captured, is opaque to further matching
// inside the SAME arm; but maybe a second macro invocation on the raw tokens
// (before `:literal` capture even happens) can see two tokens if the lexer
// produced two.
macro_rules! attempt_a {
    ($a:tt $b:tt) => {
        concat!("two toks: ", stringify!($a), ",", stringify!($b))
    };
    ($one:tt) => {
        concat!("one tok: ", stringify!($one))
    };
}

// Attempt B: stringify the literal, then try to feed the STRING back through
// a macro that expects multiple tokens. stringify! produces one new string
// literal; test whether that string literal is itself decomposable.
macro_rules! attempt_b_inner {
    ($a:tt $b:tt) => {
        concat!("string decomposed: ", stringify!($a), ",", stringify!($b))
    };
    ($one:tt) => {
        concat!("string stayed one tok: ", stringify!($one))
    };
}
macro_rules! attempt_b {
    ($lit:literal) => {
        attempt_b_inner!($lit)
    };
}

fn main() {
    // A: 37 as one macro argument. If the lexer produced two tokens (`3`,
    // `7`), the first arm matches. If one, the second arm matches.
    println!("A(37)   = {}", attempt_a!(37));
    println!("A(3.14) = {}", attempt_a!(3.14));

    // B: same test routed through a captured :literal and stringify!, in
    // case capturing-then-restringifying somehow exposes structure the raw
    // token stream did not.
    println!("B(37)   = {}", attempt_b!(37));
}
