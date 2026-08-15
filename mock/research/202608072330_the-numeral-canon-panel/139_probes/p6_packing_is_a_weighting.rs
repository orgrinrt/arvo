// p6: is the storage-minimising concern a POLICY or a WEIGHTING?
//
// The brief asks whether the storage-minimising, speed-first, accuracy-first
// and imitate-the-native concerns are four strategies, four axes, or a mix.
// For the storage one the question has a testable form: does packing a value
// into exactly its declared width, rather than padding it to a native slot,
// change any ANSWER? If it never does, packing buys space and costs time and
// is therefore purely a weighting, with zero policy content. If it does, it is
// a policy and belongs on the answer-visible side.
//
// Three representations of the same declared W-bit type:
//   A  packed at an arbitrary bit offset in a bitstream
//   B  padded into the next native slot
//   C  packed one bit SHORT, which is the control
//
// PREDICTION, recorded before the first run:
//   U1 A and B agree on every operation over the whole domain, for every W and
//      every bit offset. Packing is a representation choice below the declared
//      width's semantics, so it cannot be observed in a value.
//   U2 C disagrees, at a rate rising with how much of the domain needs the
//      missing bit.
//
// CONTROLS:
//   C1 the round trip must hold for A: extract(insert(v)) == v for every v and
//      every offset. If insertion is lossy the agreement in U1 would be
//      agreement between two broken things.
//   C2 C must be REPORTED as differing. A comparator that cannot see a
//      one-bit-short field is not measuring anything in U1.
//   C3 the comparison must be non-vacuous: count how many results are nonzero
//      and how many saturate.

fn mask(w: u32) -> u64 {
    if w >= 64 {
        u64::MAX
    } else {
        (1u64 << w) - 1
    }
}

// Insert `v` (already reduced to w bits) at bit offset `off` in a 128-bit
// two-word bitstream, and read it back.
fn insert(stream: &mut [u64; 2], off: u32, w: u32, v: u64) {
    let v = v & mask(w);
    let lo = off / 64;
    let sh = off % 64;
    stream[lo as usize] &= !(mask(w) << sh);
    stream[lo as usize] |= v << sh;
    if sh + w > 64 {
        let rest = sh + w - 64;
        stream[lo as usize + 1] &= !mask(rest);
        stream[lo as usize + 1] |= v >> (64 - sh);
    }
}

fn extract(stream: &[u64; 2], off: u32, w: u32) -> u64 {
    let lo = off / 64;
    let sh = off % 64;
    let mut v = stream[lo as usize] >> sh;
    if sh + w > 64 {
        v |= stream[lo as usize + 1] << (64 - sh);
    }
    v & mask(w)
}

fn sign_extend(v: u64, w: u32) -> i64 {
    let sh = 64 - w;
    ((v << sh) as i64) >> sh
}

// The declared policy, identical for all three representations: saturating,
// truncating, signed.
fn op_apply(kind: u32, a: i64, b: i64, w: u32, f: u32) -> i64 {
    let hi = (1i64 << (w - 1)) - 1;
    let lo = -(1i64 << (w - 1));
    let r = match kind {
        0 => a + b,
        1 => a - b,
        _ => {
            let p = (a as i128) * (b as i128);
            (p >> f) as i64
        }
    };
    r.clamp(lo, hi)
}

fn main() {
    let mut failures = 0usize;
    println!("packed against padded, one declared policy, exhaustive over the domain");
    println!();

    for w in [3u32, 5, 6, 7, 11] {
        for f in [0u32, 2] {
            if f >= w {
                continue;
            }
            for off in [0u32, 1, 7, 13, 59, 61] {
                let n_vals = 1i64 << w;
                let lo = -(1i64 << (w - 1));
                let vals: Vec<i64> = (0..n_vals).map(|i| lo + i).collect();

                // C1: round trip
                let mut rt_fail = 0u64;
                for &v in &vals {
                    let mut s = [0u64; 2];
                    insert(&mut s, off, w, v as u64);
                    if sign_extend(extract(&s, off, w), w) != v {
                        rt_fail += 1;
                    }
                }

                let (mut n, mut diff_ab, mut diff_ac) = (0u64, 0u64, 0u64);
                let (mut nonzero, mut saturating) = (0u64, 0u64);
                for &a in &vals {
                    for &b in &vals {
                        for kind in 0..3u32 {
                            n += 1;
                            let want = op_apply(kind, a, b, w, f);

                            // A: packed at `off`, round-tripped through storage
                            let mut sa = [0u64; 2];
                            insert(&mut sa, off, w, a as u64);
                            insert(&mut sa, off + w, w, b as u64);
                            let ra = op_apply(
                                kind,
                                sign_extend(extract(&sa, off, w), w),
                                sign_extend(extract(&sa, off + w, w), w),
                                w,
                                f,
                            );
                            let mut sa2 = [0u64; 2];
                            insert(&mut sa2, off, w, ra as u64);
                            let ra = sign_extend(extract(&sa2, off, w), w);

                            // B: padded into a native 16-bit slot
                            let pad = |v: i64| -> i64 { sign_extend((v as u64) & mask(w), w) };
                            let rb = pad(op_apply(kind, pad(a), pad(b), w, f));

                            // C: packed one bit short
                            let wc = w - 1;
                            let mut sc = [0u64; 2];
                            insert(&mut sc, off, wc, a as u64);
                            insert(&mut sc, off + wc, wc, b as u64);
                            let rc = op_apply(
                                kind,
                                sign_extend(extract(&sc, off, wc), wc),
                                sign_extend(extract(&sc, off + wc, wc), wc),
                                w,
                                f,
                            );

                            if ra != rb {
                                diff_ab += 1;
                            }
                            if ra != rc {
                                diff_ac += 1;
                            }
                            if ra != 0 {
                                nonzero += 1;
                            }
                            let raw = match kind {
                                0 => a + b,
                                1 => a - b,
                                _ => (((a as i128) * (b as i128)) >> f) as i64,
                            };
                            if raw != want {
                                saturating += 1;
                            }
                            let _ = want;
                        }
                    }
                }

                let flag = if diff_ab != 0 {
                    failures += 1;
                    " <-- U1 FAILED"
                } else {
                    ""
                };
                println!(
                    "  W={w} F={f} off={off:>2}: ops={n:>6} packed-vs-padded differ={diff_ab} | one-bit-short control differs={diff_ac} | nonzero={nonzero} saturating={saturating} roundtrip failures={rt_fail}{flag}"
                );
                if rt_fail != 0 {
                    println!("    C1 FAILED: packed storage is lossy, so U1 proves nothing");
                    failures += 1;
                }
                if diff_ac == 0 {
                    println!("    C2 FAILED: the one-bit-short control was not detected");
                    failures += 1;
                }
                if nonzero == 0 {
                    println!("    C3 FAILED: every result is zero, the agreement is vacuous");
                    failures += 1;
                }
            }
        }
    }

    println!();
    println!("control failures: {failures}");
    if failures > 0 {
        std::process::exit(1);
    }
}
