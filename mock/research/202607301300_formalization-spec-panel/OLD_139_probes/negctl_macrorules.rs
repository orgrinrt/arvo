// Negative control: can macro_rules! alone cross from a literal to a structure?
// It would have to decompose the literal into digits. It cannot: `13` is one token.
#![no_std]
pub struct Term;
pub struct D0<T>(core::marker::PhantomData<T>);
pub struct D1<T>(core::marker::PhantomData<T>);
macro_rules! nat_mr {
    (0) => {
        Term
    };
    // the only handle macro_rules has on a literal is to match it whole, so any
    // general rule would have to name every width. this is the table, spelled out.
    ($n:literal) => {
        compile_error!("macro_rules cannot split a literal into digits")
    };
}
pub type A = nat_mr!(13);
