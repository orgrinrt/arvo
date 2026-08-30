//! Probe 7. Whether `Precise`'s door is a preference or is forced.
//!
//! `Precise`'s shipped meaning is "arithmetic is saturating: overflow clamps to
//! logical min/max rather than wrapping" (arvo-strategy/src/lib.rs:135-139).
//! A float operation under `Precise` therefore has to deliver the format's
//! largest finite magnitude where an IEEE operation delivers infinity. This
//! probe asks the host what it actually delivers.
//!
//! MEASURED (a fact about this host under the entry control state), not
//! reasoned.

fn main() {
    let big = f32::MAX;
    let over = big * 2.0;
    let under = -big * 2.0;
    println!(
        "f32::MAX * 2.0      = {over} (is_infinite {})",
        over.is_infinite()
    );
    println!(
        "-f32::MAX * 2.0     = {under} (is_infinite {})",
        under.is_infinite()
    );
    println!("f32::MAX saturating = {big}");
    assert!(over.is_infinite() && over.is_sign_positive());
    assert!(under.is_infinite() && under.is_sign_negative());
    // The saturating answer `Precise` owes is a DIFFERENT VALUE from what the
    // instruction delivers, so this is not a lowering under the design's own
    // definition (58:798-806) at any pinning of the environment. The one
    // control state under which IEEE itself saturates is a directed rounding
    // mode, and it saturates only on the side rounding moves toward, so it is
    // not `Precise`'s two-sided clamp either.
    assert_ne!(over.to_bits(), big.to_bits());
    println!("probe_7: the host delivers infinity where `Precise` owes saturation");
}
