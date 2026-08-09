//! P5. Is a type parameter default an inference fallback, or a syntactic
//! elision on a written type path?
//!
//! P4 showed a trait parameter default does not settle an inference variable.
//! This checks the remaining shape: a struct parameter default, taken where
//! the type is written, and then the same struct with the parameter left to
//! inference at a call site with two candidate impls.
//!
//! Expected: the written path elides to the default; the inference variable
//! does not. If both hold, defaults are syntactic and no defaulting mechanism
//! reaches an inference variable at all.

#![no_std]

pub struct Keep;
pub struct Erase;

pub struct Out<T, P = Keep>(pub T, core::marker::PhantomData<P>);

/// Written path with the parameter elided. The default should apply here.
pub fn written_path(x: u32) -> Out<u32> {
    Out(x, core::marker::PhantomData)
}

/// Confirms the elided form really is `Out<u32, Keep>` and not something else.
pub fn confirm_written(x: u32) -> Out<u32, Keep> {
    written_path(x)
}

pub trait Pick<P> {
    fn pick(self) -> u32;
}

impl Pick<Keep> for u32 {
    fn pick(self) -> u32 {
        self
    }
}

impl Pick<Erase> for u32 {
    fn pick(self) -> u32 {
        0
    }
}

/// The parameter is an inference variable here, with a default nowhere it can
/// be reached from. Nothing in the language supplies one.
pub fn inference_variable(x: u32) -> u32 {
    x.pick()
}
