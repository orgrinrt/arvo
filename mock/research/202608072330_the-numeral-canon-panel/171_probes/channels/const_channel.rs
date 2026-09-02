// Probe P2. Does the const-evaluation channel distinguish the same pair, and does
// it survive a RELEASE build?
//
// P1 found the debug overflow panic as a binding-free channel and found none at
// release. Const evaluation is the obvious candidate for one that survives
// release, because const-eval checks arithmetic regardless of the profile.
//
// THE CASE THAT MUST FAIL
//   C-D  The wide arm must COMPILE as a const. If neither compiles, the probe is
//        about const-eval being strict rather than about the two arms differing.
//   C-E  A const of the wide arm must equal the runtime value, so the const path
//        is computing the same function.
//
// The narrow arm as a const is in `const_channel_narrow_MUST_NOT_COMPILE.rs` and
// is expected to be REFUSED. That refusal is the result.

const A: i32 = 1_500_000_000;
const B: i32 = 1_400_000_000;
const C: i32 = 2_000_000_000;

const fn wide(a: i32, b: i32, c: i32) -> i32 {
    let t: i64 = a as i64 + b as i64;
    (t - c as i64) as i32
}

const WIDE: i32 = wide(A, B, C);

fn main() {
    println!(
        "profile = {}",
        if cfg!(debug_assertions) {
            "debug"
        } else {
            "release"
        }
    );
    println!("C-D  the wide arm compiles as a const: yes, WIDE = {WIDE}");
    println!(
        "C-E  matches the runtime value {}: {}",
        wide(A, B, C),
        WIDE == wide(A, B, C)
    );
}
