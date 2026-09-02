//! The positive control for `the_position.rs`.
//!
//! Identical invocation, identical shape, one name changed to something arvo
//! does export. It compiles, so the `E0432` beside it is a fact about `USize`
//! and not about how rustc was called here.

use arvo_format::Width;

pub enum LinkError {
    PathNotFound,
    LoadFailed { platform_code: Width },
}
