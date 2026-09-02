//! The arm that does not compile, kept out of the build.
//!
//! A shipped point of the parameterisation has no `Debug`, so the ordinary route
//! does not reach it. Its stderr is committed beside this file.

use arvo_format::points::Integer;

pub fn render_a_point() {
    let _ = format_args!("{:?}", Integer::<32>);
}
