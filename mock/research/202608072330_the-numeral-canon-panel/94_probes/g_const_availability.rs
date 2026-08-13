// Probe G. I15 says never a runtime check, and everything reaches one lowered
// path. Probes A, B and F all selected on a const ARITY. Real folds take a
// slice whose length is a runtime value, so the question the earlier probes
// dodged is: what happens to the selection when the region fact it reads is
// not const-available?
//
// Two entry points, identical apart from where the arity comes from. The
// emitted code answers it directly.
//
// Build:
//   rustc --edition 2024 -O --emit asm -C panic=abort -o g_const_availability.s \
//         g_const_availability.rs

#![no_std]
#![crate_type = "lib"]

#[inline(never)]
pub fn arm_seq(v: &[u32]) -> u32 {
    let mut a: u32 = 0;
    for &x in v {
        a = a.wrapping_add(x);
    }
    a
}

#[inline(never)]
pub fn arm_lanes(v: &[u32]) -> u32 {
    let mut p: [u32; 4] = [0; 4];
    let c = v.len() / 4;
    for i in 0..c {
        p[0] = p[0].wrapping_add(v[i * 4]);
        p[1] = p[1].wrapping_add(v[i * 4 + 1]);
        p[2] = p[2].wrapping_add(v[i * 4 + 2]);
        p[3] = p[3].wrapping_add(v[i * 4 + 3]);
    }
    let mut a = p[0]
        .wrapping_add(p[1])
        .wrapping_add(p[2])
        .wrapping_add(p[3]);
    let mut i = c * 4;
    while i < v.len() {
        a = a.wrapping_add(v[i]);
        i += 1;
    }
    a
}

/// The arity is a type-level fact, as it is wherever sizes are const. The
/// selection is a monomorphisation-time constant.
#[inline(always)]
pub fn fold_const<const ARITY: usize>(v: &[u32; ARITY]) -> u32 {
    if ARITY >= 16 {
        arm_lanes(v)
    } else {
        arm_seq(v)
    }
}

/// The arity is a runtime fact. Same selection rule, same arms.
#[inline(always)]
pub fn fold_runtime(v: &[u32]) -> u32 {
    if v.len() >= 16 {
        arm_lanes(v)
    } else {
        arm_seq(v)
    }
}

#[unsafe(no_mangle)]
pub fn site_const_long(v: &[u32; 64]) -> u32 {
    fold_const::<64>(v)
}

#[unsafe(no_mangle)]
pub fn site_const_short(v: &[u32; 4]) -> u32 {
    fold_const::<4>(v)
}

#[unsafe(no_mangle)]
pub fn site_runtime(v: &[u32]) -> u32 {
    fold_runtime(v)
}
