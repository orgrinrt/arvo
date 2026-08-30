#![feature(const_trait_impl)]
use union::*;
type Sat = Number<Fix<13, 3, Unsigned>, Warm, Warm>;
type Prec = Number<Fix<13, 3, Unsigned>, Precise, Precise>;
type PrecB = Number<Fix<13, 3, Unsigned>, Precise, DeliveredAs<Precise, AsBottom>>;

#[unsafe(no_mangle)]
pub extern "C" fn u_sat(xs: *const u16, n: usize) -> u16 {
    let mut acc = 0u16;
    for i in 0..n {
        let v = unsafe { *xs.add(i) };
        acc = match add::<Sat>(acc, v, 0, 4095) {
            Total(t) => t,
        };
    }
    acc
}
#[unsafe(no_mangle)]
pub extern "C" fn u_prec(xs: *const u16, n: usize) -> u16 {
    let mut acc = 0u16;
    for i in 0..n {
        let v = unsafe { *xs.add(i) };
        acc = match add::<Prec>(acc, v, 0, 4095) {
            Fallible::Ok(t) => t,
            Fallible::Refused => return u16::MAX,
        };
    }
    acc
}
#[unsafe(no_mangle)]
pub extern "C" fn u_bot(xs: *const u16, n: usize) -> u16 {
    let mut acc = Poison {
        v: 0u16,
        bottom: false,
    };
    for i in 0..n {
        let v = unsafe { *xs.add(i) };
        let r = add::<PrecB>(acc.v, v, 0, 4095);
        acc = Poison {
            v: r.v,
            bottom: acc.bottom | r.bottom,
        };
    }
    if acc.bottom {
        u16::MAX
    } else {
        acc.v
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn u_raw(xs: *const u16, n: usize) -> u16 {
    let mut acc = 0u16;
    for i in 0..n {
        let v = unsafe { *xs.add(i) };
        acc = core::cmp::min(acc.saturating_add(v), 4095);
    }
    acc
}
fn main() {}

// control: the SAME absorbing-bottom delivery, with the bottom living in the
// numeral's spare pattern instead of in a companion flag. Logical width 12
// (4095 max) in a 16-bit container, so 0xFFFF is a free sentinel.
const BOT: u16 = u16::MAX;
#[unsafe(no_mangle)]
pub extern "C" fn u_bot_inpayload(xs: *const u16, n: usize) -> u16 {
    let mut acc: u16 = 0;
    for i in 0..n {
        let v = unsafe { *xs.add(i) };
        let s = acc.wrapping_add(v);
        let over = (s > 4095) | (acc == BOT) | (v == BOT);
        acc = if over { BOT } else { s };
    }
    acc
}
