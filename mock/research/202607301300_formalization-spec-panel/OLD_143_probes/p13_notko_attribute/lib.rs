//! P13. The workspace rules name the attribute `#[optimize_for(hot)]`. The
//! shipped notko attribute is `#[profile(Hot)]`. This compiles both against
//! the built proc-macro crate to establish which is real and whether the
//! lowercase argument the rules print is accepted.
//!
//! Expected: the shipped spelling expands; the rules' spelling does not
//! resolve; the lowercase argument is refused with a diagnostic naming where
//! a custom tier file would have to live.

#[derive(Debug)]
pub struct Oops;

#[notko_macros::profile(Hot)]
pub fn shipped_spelling(x: u32) -> Result<u32, Oops> {
    Ok(x + 1)
}

#[notko_macros::profile(hot)]
pub fn lowercase_argument(x: u32) -> Result<u32, Oops> {
    Ok(x + 1)
}
