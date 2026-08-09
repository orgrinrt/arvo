// a loop that vectorises cleanly when the backend can prove the bound.
// the question this probe answers is about the toolchain, not about arvo:
// does `--emit=asm` on the rlib show the same vectorisation the linked
// binary ends up with, when lto is on?

#[inline(never)]
pub fn accumulate(xs: &[u32], out: &mut [u32]) {
    let n = xs.len().min(out.len());
    for i in 0..n {
        out[i] = out[i].wrapping_add(xs[i].wrapping_mul(3));
    }
}

#[inline(never)]
pub fn saturating_pass(xs: &[u16], out: &mut [u16]) {
    let n = xs.len().min(out.len());
    for i in 0..n {
        out[i] = xs[i].saturating_add(out[i]);
    }
}

// calibration: shapes whose vectorisation is not in doubt.
// if none of these vectorise either, the probe setup is wrong,
// not the toolchain.

#[inline(never)]
pub fn zip_add(a: &[u32], b: &[u32], o: &mut [u32]) {
    for ((o, a), b) in o.iter_mut().zip(a.iter()).zip(b.iter()) {
        *o = a.wrapping_add(*b);
    }
}

#[inline(never)]
pub fn zip_sat(a: &[u16], b: &[u16], o: &mut [u16]) {
    for ((o, a), b) in o.iter_mut().zip(a.iter()).zip(b.iter()) {
        *o = a.saturating_add(*b);
    }
}

#[inline(never)]
pub fn sum_f32(xs: &[f32]) -> f32 {
    xs.iter().sum()
}
