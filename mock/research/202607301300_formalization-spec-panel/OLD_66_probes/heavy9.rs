// same quantification as the cheap sweep at nine bits, heavier body:
// ten extra arithmetic steps per pair. the width is identical; only the
// per-pair step cost changed.
const fn sweep() -> u64 {
    let n: u64 = 1u64 << 9;
    let mut acc: u64 = 0;
    let mut a: u64 = 0;
    while a < n {
        let mut b: u64 = 0;
        while b < n {
            let mut t = a.wrapping_add(b);
            let mut k = 0;
            while k < 10 {
                t = t.wrapping_mul(3).wrapping_add(1);
                k += 1;
            }
            if t >= n {
                acc = acc.wrapping_add(1);
            }
            b += 1;
        }
        a += 1;
    }
    acc
}
pub const RESULT: u64 = sweep();
