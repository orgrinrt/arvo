//! Sketch: can a property carried in the type select an algorithm, and is that
//! worth anything over the alternatives?
//!
//! Three modules, three questions, all answered from emitted assembly rather
//! than from reasoning.
//!
//! - `encodings`: at one choice with one predicate, how do typestate,
//!   const-generic `bool`, `const fn` and a runtime `bool` compare? Answer:
//!   the first three are byte-identical and LLVM aliases them to one symbol.
//! - `breadth`: with three properties, nested branches and four algorithms,
//!   do the instantiations stay merged? Answer: no, they diverge completely.
//! - `constfn_limit`: can a `const fn` discriminate on a type at all?
//!   Answer: no, and that is what rules it out at breadth.
//!
//! Full evidence and the asm excerpts are in FINDINGS.md.
//!
//! The modules are `#[path]`-included rather than rewritten so that what is
//! recorded here is exactly what was compiled and measured.

#![allow(dead_code)]

pub mod encodings {
    include!("encodings.rs");
}

pub mod breadth {
    include!("breadth.rs");
}

pub mod shape_kernels {
    include!("shape_kernels.rs");
}

pub mod microkernel_selection {
    include!("microkernel_selection.rs");
}
