//! Unified container projection via Pattern C const-tag dispatch.
//!
//! `BitsContainerFor<const N: u16, Sign: Signedness>` const trait
//! projecting `(strategy, logical_bits, sign)` to a concrete storage
//! type. One user-facing impl per Strategy. Bucket dispatch through
//! the `Project<TAG, Sign, BYTES, S>` helper trait whose impls are
//! Strategy-aware (Hot / Cold use min aligned native; Warm / Precise
//! use 2x logical native).
//!
//! Native bucket boundaries per Strategy:
//!
//! - **Hot / Cold**: minimum aligned per logical bit count.
//!   `1..=8 -> u8`, `9..=16 -> u16`, `17..=32 -> u32`,
//!   `33..=64 -> u64`, `65..=128 -> u128`.
//! - **Warm / Precise**: 2x logical width (one bucket up; carries
//!   single-op overflow headroom for Warm wrapping and Precise
//!   saturating semantics). `1..=8 -> u16`, `9..=16 -> u32`,
//!   `17..=32 -> u64`, `33..=64 -> u128`. **No native bucket above
//!   N=64** by design: Warm / Precise at `N=65..=128` falls into the
//!   wide bucket directly (no native u256 ladder).
//!
//! Wide bucket (above the native ladder) projects to
//! `WideBits<bytes_for(N), A>`:
//!
//! - **Hot**: `A = A16` (SSE2 / NEON 16-byte aligned baseline).
//!   #320 lands AVX-2 (`A32`) and AVX-512 (`A64`) tiers behind cfg
//!   gates and consumer opt-in per audit H1.
//! - **Cold / Warm / Precise**: `A = A1` (align-1 byte-exact, no
//!   alignment padding).
//!
//! Round 202605031400 (#316) replaces the per-N
//! `UContainerFor<N>` + `IContainerFor<N>` tables (~512 entries
//! across N x Sign x Strategy plus the older Round D MultiContainer
//! cells) with this Pattern C trait family. 48 impls total: 40
//! native (4 strategies x 5/4 buckets x 2 Sign, asymmetric per the
//! Strategy-aware boundaries above) + 8 wide (4 strategies x 2 Sign)
//! + 4 user-facing `BitsContainerFor` impls. ~98% reduction in
//! impl-block count vs. the per-N table.
//!
//! Validated by sketch 07 for the dispatch mechanism (TAG-keyed
//! Project impls dodge E0119 conflicts) and sketch 08 (production-
//! semantics audit: confirms the Strategy-aware bucket structure
//! cannot be Strategy-erased; sketch 07's simplification did not
//! match production and was repaired here).

use crate::{
    Align, Cold, Hot, Precise, Signed, Signedness, Strategy, Unsigned, Warm, WideBits, A1, A16,
};

// --- bucket vocabulary (was: an integer computed by a const fn) ------------
pub struct B8;
pub struct B16;
pub struct B32;
pub struct B64;
pub struct B128;
pub struct BWide<const BYTES: usize>;
pub trait Bucket {}
impl Bucket for B8 {}
impl Bucket for B16 {}
impl Bucket for B32 {}
impl Bucket for B64 {}
impl Bucket for B128 {}
impl<const BYTES: usize> Bucket for BWide<BYTES> {}

pub trait Family {}
pub struct HotCold;
impl Family for HotCold {}
pub struct WarmPrecise;
impl Family for WarmPrecise {}

// Width as typestate. Internal: it never appears in a public signature, which
// is what keeps this change inside one file.
pub struct Wid<const N: u16>;
pub trait WidthFor<F: Family> {
    type Bkt: Bucket;
}

macro_rules! widths {
    ($fam:ty, $( $n:literal => $bkt:ty ),* $(,)?) => {
        $( impl WidthFor<$fam> for Wid<$n> { type Bkt = $bkt; } )*
    };
}

#[diagnostic::on_unimplemented(
    message = "strategy `{Self}` does not provide a container for {N}-bit width"
)]
pub const trait BitsContainerFor<const N: u16, Sign: Signedness>: Strategy {
    type T: Copy + Clone + PartialEq + Eq + Default + core::hash::Hash + core::fmt::Debug + 'static;
}

pub trait Project<B: Bucket, Sign: Signedness, S: Strategy>: crate::sealed::Sealed {
    type T: Copy + Clone + PartialEq + Eq + Default + core::hash::Hash + core::fmt::Debug + 'static;
}
pub struct Picker;
impl crate::sealed::Sealed for Picker {}

macro_rules! project_native {
    ($s:ty, $( $b:ty => ($u:ty, $i:ty) ),* $(,)?) => { $(
        impl Project<$b, Unsigned, $s> for Picker { type T = $u; }
        impl Project<$b, Signed,   $s> for Picker { type T = $i; }
    )* };
}
project_native!(Hot,  B8 => (u8,i8), B16 => (u16,i16), B32 => (u32,i32), B64 => (u64,i64), B128 => (u128,i128));
project_native!(Cold, B8 => (u8,i8), B16 => (u16,i16), B32 => (u32,i32), B64 => (u64,i64), B128 => (u128,i128));
project_native!(Warm,    B16 => (u16,i16), B32 => (u32,i32), B64 => (u64,i64), B128 => (u128,i128));
project_native!(Precise, B16 => (u16,i16), B32 => (u32,i32), B64 => (u64,i64), B128 => (u128,i128));

impl<Sign: Signedness, const BYTES: usize> Project<BWide<BYTES>, Sign, Hot> for Picker {
    type T = WideBits<BYTES, A16>;
}
impl<Sign: Signedness, const BYTES: usize> Project<BWide<BYTES>, Sign, Cold> for Picker {
    type T = WideBits<BYTES, A1>;
}
impl<Sign: Signedness, const BYTES: usize> Project<BWide<BYTES>, Sign, Warm> for Picker {
    type T = WideBits<BYTES, A1>;
}
impl<Sign: Signedness, const BYTES: usize> Project<BWide<BYTES>, Sign, Precise> for Picker {
    type T = WideBits<BYTES, A1>;
}

const impl<const N: u16, Sign: Signedness> BitsContainerFor<N, Sign> for Hot
where
    Wid<N>: WidthFor<HotCold>,
    Picker: Project<<Wid<N> as WidthFor<HotCold>>::Bkt, Sign, Hot>,
{
    type T = <Picker as Project<<Wid<N> as WidthFor<HotCold>>::Bkt, Sign, Hot>>::T;
}

const impl<const N: u16, Sign: Signedness> BitsContainerFor<N, Sign> for Cold
where
    Wid<N>: WidthFor<HotCold>,
    Picker: Project<<Wid<N> as WidthFor<HotCold>>::Bkt, Sign, Cold>,
{
    type T = <Picker as Project<<Wid<N> as WidthFor<HotCold>>::Bkt, Sign, Cold>>::T;
}

const impl<const N: u16, Sign: Signedness> BitsContainerFor<N, Sign> for Warm
where
    Wid<N>: WidthFor<WarmPrecise>,
    Picker: Project<<Wid<N> as WidthFor<WarmPrecise>>::Bkt, Sign, Warm>,
{
    type T = <Picker as Project<<Wid<N> as WidthFor<WarmPrecise>>::Bkt, Sign, Warm>>::T;
}

const impl<const N: u16, Sign: Signedness> BitsContainerFor<N, Sign> for Precise
where
    Wid<N>: WidthFor<WarmPrecise>,
    Picker: Project<<Wid<N> as WidthFor<WarmPrecise>>::Bkt, Sign, Precise>,
{
    type T = <Picker as Project<<Wid<N> as WidthFor<WarmPrecise>>::Bkt, Sign, Precise>>::T;
}

const _: fn() = || {
    fn _b<A: Align>() {}
    _b::<A1>();
    _b::<A16>();
};

// --- the generated table (this is the piece that does not scale) ----------
widths!(HotCold, 1 => B8, 2 => B8, 3 => B8, 4 => B8, 5 => B8, 6 => B8, 7 => B8, 8 => B8, 9 => B16, 10 => B16, 11 => B16, 12 => B16, 13 => B16, 14 => B16, 15 => B16, 16 => B16, 17 => B32, 18 => B32, 19 => B32, 20 => B32, 21 => B32, 22 => B32, 23 => B32, 24 => B32, 25 => B32, 26 => B32, 27 => B32, 28 => B32, 29 => B32, 30 => B32, 31 => B32, 32 => B32, 33 => B64, 34 => B64, 35 => B64, 36 => B64, 37 => B64, 38 => B64, 39 => B64, 40 => B64, 41 => B64, 42 => B64, 43 => B64, 44 => B64, 45 => B64, 46 => B64, 47 => B64, 48 => B64, 49 => B64, 50 => B64, 51 => B64, 52 => B64, 53 => B64, 54 => B64, 55 => B64, 56 => B64, 57 => B64, 58 => B64, 59 => B64, 60 => B64, 61 => B64, 62 => B64, 63 => B64, 64 => B64, 65 => B128, 66 => B128, 67 => B128, 68 => B128, 69 => B128, 70 => B128, 71 => B128, 72 => B128, 73 => B128, 74 => B128, 75 => B128, 76 => B128, 77 => B128, 78 => B128, 79 => B128, 80 => B128, 81 => B128, 82 => B128, 83 => B128, 84 => B128, 85 => B128, 86 => B128, 87 => B128, 88 => B128, 89 => B128, 90 => B128, 91 => B128, 92 => B128, 93 => B128, 94 => B128, 95 => B128, 96 => B128, 97 => B128, 98 => B128, 99 => B128, 100 => B128, 101 => B128, 102 => B128, 103 => B128, 104 => B128, 105 => B128, 106 => B128, 107 => B128, 108 => B128, 109 => B128, 110 => B128, 111 => B128, 112 => B128, 113 => B128, 114 => B128, 115 => B128, 116 => B128, 117 => B128, 118 => B128, 119 => B128, 120 => B128, 121 => B128, 122 => B128, 123 => B128, 124 => B128, 125 => B128, 126 => B128, 127 => B128, 128 => B128, 129 => BWide<17>, 130 => BWide<17>, 131 => BWide<17>, 132 => BWide<17>, 133 => BWide<17>, 134 => BWide<17>, 135 => BWide<17>, 136 => BWide<17>, 137 => BWide<18>, 138 => BWide<18>, 139 => BWide<18>, 140 => BWide<18>, 141 => BWide<18>, 142 => BWide<18>, 143 => BWide<18>, 144 => BWide<18>, 145 => BWide<19>, 146 => BWide<19>, 147 => BWide<19>, 148 => BWide<19>, 149 => BWide<19>, 150 => BWide<19>, 151 => BWide<19>, 152 => BWide<19>, 153 => BWide<20>, 154 => BWide<20>, 155 => BWide<20>, 156 => BWide<20>, 157 => BWide<20>, 158 => BWide<20>, 159 => BWide<20>, 160 => BWide<20>, 161 => BWide<21>, 162 => BWide<21>, 163 => BWide<21>, 164 => BWide<21>, 165 => BWide<21>, 166 => BWide<21>, 167 => BWide<21>, 168 => BWide<21>, 169 => BWide<22>, 170 => BWide<22>, 171 => BWide<22>, 172 => BWide<22>, 173 => BWide<22>, 174 => BWide<22>, 175 => BWide<22>, 176 => BWide<22>, 177 => BWide<23>, 178 => BWide<23>, 179 => BWide<23>, 180 => BWide<23>, 181 => BWide<23>, 182 => BWide<23>, 183 => BWide<23>, 184 => BWide<23>, 185 => BWide<24>, 186 => BWide<24>, 187 => BWide<24>, 188 => BWide<24>, 189 => BWide<24>, 190 => BWide<24>, 191 => BWide<24>, 192 => BWide<24>, 193 => BWide<25>, 194 => BWide<25>, 195 => BWide<25>, 196 => BWide<25>, 197 => BWide<25>, 198 => BWide<25>, 199 => BWide<25>, 200 => BWide<25>, 201 => BWide<26>, 202 => BWide<26>, 203 => BWide<26>, 204 => BWide<26>, 205 => BWide<26>, 206 => BWide<26>, 207 => BWide<26>, 208 => BWide<26>, 209 => BWide<27>, 210 => BWide<27>, 211 => BWide<27>, 212 => BWide<27>, 213 => BWide<27>, 214 => BWide<27>, 215 => BWide<27>, 216 => BWide<27>, 217 => BWide<28>, 218 => BWide<28>, 219 => BWide<28>, 220 => BWide<28>, 221 => BWide<28>, 222 => BWide<28>, 223 => BWide<28>, 224 => BWide<28>, 225 => BWide<29>, 226 => BWide<29>, 227 => BWide<29>, 228 => BWide<29>, 229 => BWide<29>, 230 => BWide<29>, 231 => BWide<29>, 232 => BWide<29>, 233 => BWide<30>, 234 => BWide<30>, 235 => BWide<30>, 236 => BWide<30>, 237 => BWide<30>, 238 => BWide<30>, 239 => BWide<30>, 240 => BWide<30>, 241 => BWide<31>, 242 => BWide<31>, 243 => BWide<31>, 244 => BWide<31>, 245 => BWide<31>, 246 => BWide<31>, 247 => BWide<31>, 248 => BWide<31>, 249 => BWide<32>, 250 => BWide<32>, 251 => BWide<32>, 252 => BWide<32>, 253 => BWide<32>, 254 => BWide<32>, 255 => BWide<32>, 256 => BWide<32>);
widths!(WarmPrecise, 1 => B16, 2 => B16, 3 => B16, 4 => B16, 5 => B16, 6 => B16, 7 => B16, 8 => B16, 9 => B32, 10 => B32, 11 => B32, 12 => B32, 13 => B32, 14 => B32, 15 => B32, 16 => B32, 17 => B64, 18 => B64, 19 => B64, 20 => B64, 21 => B64, 22 => B64, 23 => B64, 24 => B64, 25 => B64, 26 => B64, 27 => B64, 28 => B64, 29 => B64, 30 => B64, 31 => B64, 32 => B64, 33 => B128, 34 => B128, 35 => B128, 36 => B128, 37 => B128, 38 => B128, 39 => B128, 40 => B128, 41 => B128, 42 => B128, 43 => B128, 44 => B128, 45 => B128, 46 => B128, 47 => B128, 48 => B128, 49 => B128, 50 => B128, 51 => B128, 52 => B128, 53 => B128, 54 => B128, 55 => B128, 56 => B128, 57 => B128, 58 => B128, 59 => B128, 60 => B128, 61 => B128, 62 => B128, 63 => B128, 64 => B128, 65 => BWide<9>, 66 => BWide<9>, 67 => BWide<9>, 68 => BWide<9>, 69 => BWide<9>, 70 => BWide<9>, 71 => BWide<9>, 72 => BWide<9>, 73 => BWide<10>, 74 => BWide<10>, 75 => BWide<10>, 76 => BWide<10>, 77 => BWide<10>, 78 => BWide<10>, 79 => BWide<10>, 80 => BWide<10>, 81 => BWide<11>, 82 => BWide<11>, 83 => BWide<11>, 84 => BWide<11>, 85 => BWide<11>, 86 => BWide<11>, 87 => BWide<11>, 88 => BWide<11>, 89 => BWide<12>, 90 => BWide<12>, 91 => BWide<12>, 92 => BWide<12>, 93 => BWide<12>, 94 => BWide<12>, 95 => BWide<12>, 96 => BWide<12>, 97 => BWide<13>, 98 => BWide<13>, 99 => BWide<13>, 100 => BWide<13>, 101 => BWide<13>, 102 => BWide<13>, 103 => BWide<13>, 104 => BWide<13>, 105 => BWide<14>, 106 => BWide<14>, 107 => BWide<14>, 108 => BWide<14>, 109 => BWide<14>, 110 => BWide<14>, 111 => BWide<14>, 112 => BWide<14>, 113 => BWide<15>, 114 => BWide<15>, 115 => BWide<15>, 116 => BWide<15>, 117 => BWide<15>, 118 => BWide<15>, 119 => BWide<15>, 120 => BWide<15>, 121 => BWide<16>, 122 => BWide<16>, 123 => BWide<16>, 124 => BWide<16>, 125 => BWide<16>, 126 => BWide<16>, 127 => BWide<16>, 128 => BWide<16>, 129 => BWide<17>, 130 => BWide<17>, 131 => BWide<17>, 132 => BWide<17>, 133 => BWide<17>, 134 => BWide<17>, 135 => BWide<17>, 136 => BWide<17>, 137 => BWide<18>, 138 => BWide<18>, 139 => BWide<18>, 140 => BWide<18>, 141 => BWide<18>, 142 => BWide<18>, 143 => BWide<18>, 144 => BWide<18>, 145 => BWide<19>, 146 => BWide<19>, 147 => BWide<19>, 148 => BWide<19>, 149 => BWide<19>, 150 => BWide<19>, 151 => BWide<19>, 152 => BWide<19>, 153 => BWide<20>, 154 => BWide<20>, 155 => BWide<20>, 156 => BWide<20>, 157 => BWide<20>, 158 => BWide<20>, 159 => BWide<20>, 160 => BWide<20>, 161 => BWide<21>, 162 => BWide<21>, 163 => BWide<21>, 164 => BWide<21>, 165 => BWide<21>, 166 => BWide<21>, 167 => BWide<21>, 168 => BWide<21>, 169 => BWide<22>, 170 => BWide<22>, 171 => BWide<22>, 172 => BWide<22>, 173 => BWide<22>, 174 => BWide<22>, 175 => BWide<22>, 176 => BWide<22>, 177 => BWide<23>, 178 => BWide<23>, 179 => BWide<23>, 180 => BWide<23>, 181 => BWide<23>, 182 => BWide<23>, 183 => BWide<23>, 184 => BWide<23>, 185 => BWide<24>, 186 => BWide<24>, 187 => BWide<24>, 188 => BWide<24>, 189 => BWide<24>, 190 => BWide<24>, 191 => BWide<24>, 192 => BWide<24>, 193 => BWide<25>, 194 => BWide<25>, 195 => BWide<25>, 196 => BWide<25>, 197 => BWide<25>, 198 => BWide<25>, 199 => BWide<25>, 200 => BWide<25>, 201 => BWide<26>, 202 => BWide<26>, 203 => BWide<26>, 204 => BWide<26>, 205 => BWide<26>, 206 => BWide<26>, 207 => BWide<26>, 208 => BWide<26>, 209 => BWide<27>, 210 => BWide<27>, 211 => BWide<27>, 212 => BWide<27>, 213 => BWide<27>, 214 => BWide<27>, 215 => BWide<27>, 216 => BWide<27>, 217 => BWide<28>, 218 => BWide<28>, 219 => BWide<28>, 220 => BWide<28>, 221 => BWide<28>, 222 => BWide<28>, 223 => BWide<28>, 224 => BWide<28>, 225 => BWide<29>, 226 => BWide<29>, 227 => BWide<29>, 228 => BWide<29>, 229 => BWide<29>, 230 => BWide<29>, 231 => BWide<29>, 232 => BWide<29>, 233 => BWide<30>, 234 => BWide<30>, 235 => BWide<30>, 236 => BWide<30>, 237 => BWide<30>, 238 => BWide<30>, 239 => BWide<30>, 240 => BWide<30>, 241 => BWide<31>, 242 => BWide<31>, 243 => BWide<31>, 244 => BWide<31>, 245 => BWide<31>, 246 => BWide<31>, 247 => BWide<31>, 248 => BWide<31>, 249 => BWide<32>, 250 => BWide<32>, 251 => BWide<32>, 252 => BWide<32>, 253 => BWide<32>, 254 => BWide<32>, 255 => BWide<32>, 256 => BWide<32>);

#[inline(always)]
pub const fn tag_hot_cold(n: u16) -> usize {
    let n = n as usize;
    if n <= 8 {
        0
    } else if n <= 16 {
        1
    } else if n <= 32 {
        2
    } else if n <= 64 {
        3
    } else if n <= 128 {
        4
    } else {
        5
    }
}
#[inline(always)]
pub const fn tag_warm_precise(n: u16) -> usize {
    let n = n as usize;
    if n <= 8 {
        0
    } else if n <= 16 {
        1
    } else if n <= 32 {
        2
    } else if n <= 64 {
        3
    } else {
        5
    }
}
#[inline(always)]
pub const fn bytes_for_u16(n: u16) -> usize {
    (n as usize).div_ceil(8)
}
