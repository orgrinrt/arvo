//! P0. What is the const-trait spelling on the pinned nightly?
//!
//! Established because the panel's premises forbid several features and the
//! workspace's own rules still write `#[const_trait]`, which this toolchain
//! rejects outright. The compiler names the replacement in its own
//! diagnostic. Recorded so no later file spends time on it.
//!
//! Build: rustc -O p0_const_trait_spelling_on_the_pin.rs
//! (from inside the repo, so rust-toolchain.toml pins nightly-2026-05-28)
#![feature(const_trait_impl)]

const trait Op {
    fn ap(a: i32, b: i32) -> i32;
}
struct S;
const impl Op for S {
    fn ap(a: i32, b: i32) -> i32 {
        a + b
    }
}
const fn twice<T: [const] Op>(a: i32) -> i32 {
    T::ap(a, a)
}
const X: i32 = twice::<S>(3);

fn main() {
    assert_eq!(X, 6);
    println!("const trait + [const] bound + generic const fn + const-eval: OK, X={X}");
    println!("spelling on this pin: `const trait T`, NOT `#[const_trait] trait T`");
}
