//! P4: 133's cost two, built. The structural encoding cannot reach `[u8; <B as Nat>::V]`
//! because a type is not an array length. But it does not need an array: a repr(C)
//! word cons has the size and alignment by construction, and its add is structural
//! recursion. Zero features, zero flags.
#![no_std]
#![crate_type = "lib"]

// --- the structural payload ---------------------------------------------------
#[derive(Clone, Copy)]
#[repr(C)]
pub struct WNil;
#[derive(Clone, Copy)]
#[repr(C)]
pub struct WCons<T> {
    pub w: u64,
    pub rest: T,
}

/// Add with an incoming carry, defined by structural recursion on the payload.
pub trait WAdd: Copy {
    fn add_c(self, o: Self, carry: bool) -> (Self, bool);
}
impl WAdd for WNil {
    #[inline]
    fn add_c(self, _o: Self, carry: bool) -> (Self, bool) {
        (WNil, carry)
    }
}
impl<T: WAdd> WAdd for WCons<T> {
    #[inline]
    fn add_c(self, o: Self, carry: bool) -> (Self, bool) {
        let (s, c) = self.w.carrying_add(o.w, carry);
        let (rest, c2) = self.rest.add_c(o.rest, c);
        (WCons { w: s, rest }, c2)
    }
}

pub type W1 = WCons<WNil>;
pub type W2 = WCons<W1>;
pub type W3 = WCons<W2>;
pub type W4 = WCons<W3>;
pub type W8 = WCons<WCons<WCons<WCons<W4>>>>;
pub type W16 = WCons<WCons<WCons<WCons<WCons<WCons<WCons<WCons<W8>>>>>>>>;

// --- a byte tail, for a byte-exact footprint ----------------------------------
#[derive(Clone, Copy)]
#[repr(C)]
pub struct BNil;
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct BCons<T> {
    pub b: u8,
    pub rest: T,
}
pub trait BAdd: Copy {
    fn add_c(self, o: Self, carry: bool) -> (Self, bool);
}
impl BAdd for BNil {
    #[inline]
    fn add_c(self, _o: Self, carry: bool) -> (Self, bool) {
        (BNil, carry)
    }
}
impl<T: BAdd> BAdd for BCons<T> {
    #[inline]
    fn add_c(self, o: Self, carry: bool) -> (Self, bool) {
        let (s, c) = self.b.carrying_add(o.b, carry);
        let (rest, c2) = self.rest.add_c(o.rest, c);
        (BCons { b: s, rest }, c2)
    }
}

/// A byte-exact wide payload: whole words, then a byte tail.
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct Ragged<W, B> {
    pub words: W,
    pub tail: B,
}
impl<W: WAdd, B: BAdd> Ragged<W, B> {
    #[inline]
    pub fn add(self, o: Self) -> Self {
        let (words, c) = self.words.add_c(o.words, false);
        let (tail, _) = self.tail.add_c(o.tail, c);
        Ragged { words, tail }
    }
}

// --- hand-written bars ---------------------------------------------------------
#[unsafe(no_mangle)]
pub fn bar_192(a: [u64; 3], b: [u64; 3]) -> [u64; 3] {
    let (r0, c) = a[0].carrying_add(b[0], false);
    let (r1, c) = a[1].carrying_add(b[1], c);
    let (r2, _) = a[2].carrying_add(b[2], c);
    [r0, r1, r2]
}
#[unsafe(no_mangle)]
pub fn bar_256(a: [u64; 4], b: [u64; 4]) -> [u64; 4] {
    let (r0, c) = a[0].carrying_add(b[0], false);
    let (r1, c) = a[1].carrying_add(b[1], c);
    let (r2, c) = a[2].carrying_add(b[2], c);
    let (r3, _) = a[3].carrying_add(b[3], c);
    [r0, r1, r2, r3]
}
#[unsafe(no_mangle)]
pub fn bar_512(a: [u64; 8], b: [u64; 8]) -> [u64; 8] {
    let mut out = [0u64; 8];
    let mut c = false;
    let mut i = 0;
    while i < 8 {
        let (s, cc) = a[i].carrying_add(b[i], c);
        out[i] = s;
        c = cc;
        i += 1;
    }
    out
}
#[unsafe(no_mangle)]
pub fn bar_1024(a: [u64; 16], b: [u64; 16]) -> [u64; 16] {
    let mut out = [0u64; 16];
    let mut c = false;
    let mut i = 0;
    while i < 16 {
        let (s, cc) = a[i].carrying_add(b[i], c);
        out[i] = s;
        c = cc;
        i += 1;
    }
    out
}

// --- structural sites -----------------------------------------------------------
#[unsafe(no_mangle)]
pub fn st_192(a: W3, b: W3) -> W3 {
    a.add_c(b, false).0
}
#[unsafe(no_mangle)]
pub fn st_256(a: W4, b: W4) -> W4 {
    a.add_c(b, false).0
}
#[unsafe(no_mangle)]
pub fn st_512(a: W8, b: W8) -> W8 {
    a.add_c(b, false).0
}
#[unsafe(no_mangle)]
pub fn st_1024(a: W16, b: W16) -> W16 {
    a.add_c(b, false).0
}

// 136 bits: two words plus one byte, byte-exact at 17 bytes
pub type R136 = Ragged<W2, BCons<BNil>>;
#[unsafe(no_mangle)]
pub fn st_136(a: R136, b: R136) -> R136 {
    a.add(b)
}

// 200 bits: three words plus one byte, byte-exact at 25 bytes
pub type R200 = Ragged<W3, BCons<BNil>>;
#[unsafe(no_mangle)]
pub fn st_200(a: R200, b: R200) -> R200 {
    a.add(b)
}

// --- footprint, asserted rather than argued ------------------------------------
const _: () = assert!(core::mem::size_of::<W3>() == 24);
const _: () = assert!(core::mem::size_of::<W16>() == 128);
const _: () = assert!(core::mem::align_of::<W3>() == 8);
const _: () = assert!(core::mem::size_of::<R136>() == 17);
const _: () = assert!(core::mem::align_of::<R136>() == 1);
const _: () = assert!(core::mem::size_of::<R200>() == 25);
const _: () = assert!(core::mem::align_of::<R200>() == 1);

// The value the structural payload denotes is readable in value position, which is
// what makes the byte count available to anything that needs it as a number.
pub trait WLen {
    const BYTES: usize;
}
impl WLen for WNil {
    const BYTES: usize = 0;
}
impl<T: WLen> WLen for WCons<T> {
    const BYTES: usize = 8 + T::BYTES;
}
impl WLen for BNil {
    const BYTES: usize = 0;
}
impl<T: WLen> WLen for BCons<T> {
    const BYTES: usize = 1 + T::BYTES;
}
impl<W: WLen, B: WLen> WLen for Ragged<W, B> {
    const BYTES: usize = W::BYTES + B::BYTES;
}
const _: () = assert!(<R200 as WLen>::BYTES == 25);
const _: () = assert!(<W16 as WLen>::BYTES == 128);
