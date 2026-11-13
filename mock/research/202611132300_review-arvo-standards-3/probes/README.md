# Review 3 probes for `work/arvo-standards` at `366d2e15`

Ad-hoc quick spikes, not benches. One crate with its own workspace and a path dependency on
`mock/crates/arvo-format`. Run from this directory with
`CARGO_TARGET_DIR=/home/user/target CARGO_INCREMENTAL=0 cargo test -- --nocapture`, nightly-2026-05-28.

`src/lib.rs`, raw output:

    [("add b32", false), ("mul b32", false), ("div b32", false), ("add d32", false),
     ("fi32 -> b32", false), ("b32 -> fi32", false), ("b16 -> b32", false), ("fi8 -> fi16 control", true)]
    slots::is_admissible true, is_admissible_format true, is_sign_and_magnitude false

So every operation verdict and `is_convertible` in both directions refuse the interchange formats,
with a fixed-point conversion as the control that must pass, and the two generic verdicts admit
`Interchange<UnsignedBinaryRationals, 24, 127>` while the significand's own verdict refuses it.

`tests/fimath_every_width.rs` is `tests/matlab_fimath_parity.rs` copied at `366d2e15` with its 13
`shape!` invocations replaced by all 128 shapes, signed and unsigned at word lengths 1 to 64, fraction
length 0. Raw output: `test result: ok. 256 passed; 0 failed`, the method arms and the planted arms at
every shape, about five seconds to build and run.
