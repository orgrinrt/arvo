# 270_probes

`rustc -O p1_half_up_breaks_translation_equivariance_at_zero.rs -o /tmp/p1_270 && /tmp/p1_270`

Output captured in `p1.out`. Standalone rustc script; the three rounding
rules it exercises (floor, ceil, the shipped away-from-zero half_up, and the
alternate toward-positive-infinity half_up) are transcribed from
`mock/crates/arvo-format/src/apply.rs`'s `round_slot`, not imported from the
crate, because the crate is `no_std` and the probe needs `println!`. Every
transcribed arm is cited against the source line it copies in the probe's own
header comment.
