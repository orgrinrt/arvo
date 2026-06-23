// Sub-problem B: Precise saturation at the LOGICAL bound, not the container bound.
// Precise is always DoubleLogical: container = 2x logical width, so for logical N the
// container has >= 2N bits, hence N-1 < container_bits and 1<<(N-1) never overflows.
// No widening needed; only clamp.

// Generic signed clamp to logical width N, in the i128-superset domain then narrow per container.
// We demonstrate with a const-generic N and a concrete container type via a small trait shape.

// signed logical bounds in the container type C (here shown for i16, the N=8 Precise container)
const fn i_logical_max_i16(n: u16) -> i16 {
    (1i16 << (n - 1)) - 1
}
const fn i_logical_min_i16(n: u16) -> i16 {
    -(1i16 << (n - 1))
}
const fn clamp_logical_i16(v: i16, n: u16) -> i16 {
    let lo = i_logical_min_i16(n);
    let hi = i_logical_max_i16(n);
    if v < lo { lo } else if v > hi { hi } else { v }
}

// fixed-point multiply for Precise i16 container at logical N, FRAC: saturating at logical bound.
const fn i_mul_fixed_precise_i16(a: i16, b: i16, n: u16, frac: u32) -> i16 {
    // container holds the product (2x), so plain widen-free product then rescale, then clamp.
    let prod = (a as i32).wrapping_mul(b as i32);
    let rescaled = (prod >> frac) as i16; // i16 container holds it for DoubleLogical in-range
    clamp_logical_i16(rescaled, n)
}

// generalization to add/sub/div: same clamp tail.
const fn i_add_precise_i16(a: i16, b: i16, n: u16) -> i16 {
    clamp_logical_i16(a.saturating_add(b), n)
}
const fn i_sub_precise_i16(a: i16, b: i16, n: u16) -> i16 {
    clamp_logical_i16(a.saturating_sub(b), n)
}

fn main() {
    // Target: Precise N=8 i16, raw 100 * raw 100 at FRAC=4 => clamp to logical max 127.
    const T: i16 = i_mul_fixed_precise_i16(100, 100, 8, 4);
    assert_eq!(T, 127, "logical max clamp");

    // In-range (no saturation): raw 8 * raw 8 at FRAC=4 = 64>>4 = 4
    const T2: i16 = i_mul_fixed_precise_i16(8, 8, 8, 4);
    assert_eq!(T2, 4);

    // Negative clamp: raw -100 * raw 100 at FRAC=4 = -625 -> clamp to logical min -128
    const T3: i16 = i_mul_fixed_precise_i16(-100, 100, 8, 4);
    assert_eq!(T3, -128, "logical min clamp");

    // add generalization: 100 + 100 = 200 -> clamp 127
    const A: i16 = i_add_precise_i16(100, 100, 8);
    assert_eq!(A, 127);

    // sub generalization: -100 - 100 = -200 -> clamp -128
    const S: i16 = i_sub_precise_i16(-100, 100, 8);
    assert_eq!(S, -128);

    // bound math sanity for N=8: max=127 min=-128
    assert_eq!(i_logical_max_i16(8), 127);
    assert_eq!(i_logical_min_i16(8), -128);

    println!("ALL B PASS");
}
