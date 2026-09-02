//! The arm that does not compile, kept out of the build on purpose.
//!
//! `mock/PRINCIPLES.md.tmpl:220` names `IFixed` in `arvo-format`. This is the
//! consumer importing it. The stderr beside this directory is what it produces.

use arvo_format::IFixed;

pub struct Signed(IFixed);
