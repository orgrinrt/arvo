// 157 P8. Is the soundness half of adequacy mechanically enforceable?
//
// CLAIM UNDER TEST
//   157 Q157-D. Soundness holds when the denotation FACTORS through the parameters the
//   type carries. The obvious mechanisation is to make the realisation map a `const fn`
//   whose arguments are exactly the type's parameters, so nothing else is in scope.
//
//   This file tests whether that is sufficient. It is not: `cfg` and module-level `const`
//   are in scope inside any function, so a realisation map can read the build without
//   naming it in its signature, and the same type then denotes two different things in
//   two builds of the same source.
//
// NEGATIVE CONTROL, stated before the run
//   The SAME program with only the LOWERING cfg-gated must produce the SAME denotation
//   under both builds. If it does not, the probe is measuring the cfg mechanism rather
//   than the R/lambda distinction and proves nothing.
//
// BUILD
//   rustc --edition 2021 -O factoring.rs -o a          && ./a
//   rustc --edition 2021 -O --cfg alt factoring.rs -o b && ./b
//   The two outputs must AGREE on the control lines and DISAGREE on the hazard line.

#![allow(dead_code)]

const W: u32 = 13;
const MAXW: u64 = (1u64 << W) - 1;

// ---------------------------------------------------------------------------
// R, the realisation map. Signature names exactly the parameters the type carries.
// Nothing about the build appears in the signature. The `cfg` inside it is invisible
// from the outside and is exactly the hole.
// ---------------------------------------------------------------------------
const fn realise_hazard(exact: u128, w: u32) -> u64 {
    let hi = ((1u128 << w) - 1) as u64;
    #[cfg(not(alt))]
    {
        if exact > hi as u128 { hi } else { exact as u64 }      // saturate
    }
    #[cfg(alt)]
    {
        (exact % (1u128 << w)) as u64                            // wrap
    }
}

// R, honest: the build cannot reach it.
const fn realise_sound(exact: u128, w: u32) -> u64 {
    let hi = ((1u128 << w) - 1) as u64;
    if exact > hi as u128 { hi } else { exact as u64 }
}

// lambda, the lowering. Two shapes, cfg-selected, both computing the same function.
// This is the thing arvo-always-optimal-internals.md licenses varying with the target.
const fn lower_a(x: u64, w: u32) -> u64 { x & ((1u64 << w) - 1) }
const fn lower_b(x: u64, w: u32) -> u64 {
    let sh = 64 - w;
    (x << sh) >> sh
}
const fn lambda(x: u64, w: u32) -> u64 {
    #[cfg(not(alt))]
    { lower_a(x, w) }
    #[cfg(alt)]
    { lower_b(x, w) }
}

fn main() {
    let build = if cfg!(alt) { "alt" } else { "base" };

    // HAZARD: R reads the build. Same type, same name, two denotations.
    let h = realise_hazard((MAXW as u128) + 1, W);
    println!("HAZARD    build={build} R(MAX+1) = {h}");

    // CONTROL 1: R does not read the build.
    let s = realise_sound((MAXW as u128) + 1, W);
    println!("CONTROL_R build={build} R(MAX+1) = {s}");

    // CONTROL 2: only the lowering varies. Denotation must not move.
    let mut acc: u64 = 0;
    let mut x: u64 = 0;
    while x < 40000 {
        acc = acc.wrapping_add(lambda(x, W));
        x += 1;
    }
    println!("CONTROL_L build={build} sum(lambda) = {acc}");

    // The certificate, evaluated in both builds. Under the hazard the separating
    // witness for the overflow policy is answered differently by the same source.
    println!("CERT      build={build} separates = {}",
             realise_hazard((MAXW as u128) + 1, W) != realise_sound((MAXW as u128) + 1, W));
}
