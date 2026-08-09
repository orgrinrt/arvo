#![no_std]
#![allow(dead_code)]
use core::marker::PhantomData;
pub struct Warm;
pub trait Container: Copy {
    const BITS: u32;
}
impl Container for u128 {
    const BITS: u32 = 128;
}
pub struct Fx<const P: u32, C: Container, S> {
    raw: C,
    _s: PhantomData<S>,
}
impl<const P: u32, C: Container, S> Clone for Fx<P, C, S> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<const P: u32, C: Container, S> Copy for Fx<P, C, S> {}
macro_rules! UF { ($i:literal, $f:literal) => { Fx<{ $i + $f }, u128, Warm> }; }
pub fn mul<const P: u32, const Q: u32, const R: u32, C: Container, S>(
    a: Fx<P, C, S>,
    _b: Fx<Q, C, S>,
) -> Fx<R, C, S> {
    const {
        assert!(
            R == P + Q,
            "mul: output precision must equal the sum of the input precisions"
        )
    }
    Fx {
        raw: a.raw,
        _s: PhantomData,
    }
}
pub fn c0(x: UF!(1000, 0), y: UF!(1000, 0)) -> Fx<{ 1000 + 1000 }, u128, Warm> {
    mul(x, y)
}
const _: () = assert!(1000 + 1000 == 2001);
pub fn c1(x: UF!(1000, 0), y: UF!(1041, 0)) -> Fx<{ 1000 + 1041 }, u128, Warm> {
    mul(x, y)
}
const _: () = assert!(1000 + 1041 == 2041);
pub fn c2(x: UF!(1000, 0), y: UF!(1082, 0)) -> Fx<{ 1000 + 1082 }, u128, Warm> {
    mul(x, y)
}
const _: () = assert!(1000 + 1082 == 2082);
pub fn c3(x: UF!(1000, 0), y: UF!(1123, 0)) -> Fx<{ 1000 + 1123 }, u128, Warm> {
    mul(x, y)
}
const _: () = assert!(1000 + 1123 == 2123);
pub fn c4(x: UF!(1000, 0), y: UF!(1164, 0)) -> Fx<{ 1000 + 1164 }, u128, Warm> {
    mul(x, y)
}
const _: () = assert!(1000 + 1164 == 2164);
pub fn c5(x: UF!(1000, 0), y: UF!(1205, 0)) -> Fx<{ 1000 + 1205 }, u128, Warm> {
    mul(x, y)
}
const _: () = assert!(1000 + 1205 == 2205);
pub fn c6(x: UF!(1000, 0), y: UF!(1246, 0)) -> Fx<{ 1000 + 1246 }, u128, Warm> {
    mul(x, y)
}
const _: () = assert!(1000 + 1246 == 2246);
pub fn c7(x: UF!(1000, 0), y: UF!(1287, 0)) -> Fx<{ 1000 + 1287 }, u128, Warm> {
    mul(x, y)
}
const _: () = assert!(1000 + 1287 == 2287);
pub fn c8(x: UF!(1037, 0), y: UF!(1000, 0)) -> Fx<{ 1037 + 1000 }, u128, Warm> {
    mul(x, y)
}
const _: () = assert!(1037 + 1000 == 2037);
pub fn c9(x: UF!(1037, 0), y: UF!(1041, 0)) -> Fx<{ 1037 + 1041 }, u128, Warm> {
    mul(x, y)
}
const _: () = assert!(1037 + 1041 == 2078);
pub fn c10(x: UF!(1037, 0), y: UF!(1082, 0)) -> Fx<{ 1037 + 1082 }, u128, Warm> {
    mul(x, y)
}
const _: () = assert!(1037 + 1082 == 2119);
pub fn c11(x: UF!(1037, 0), y: UF!(1123, 0)) -> Fx<{ 1037 + 1123 }, u128, Warm> {
    mul(x, y)
}
const _: () = assert!(1037 + 1123 == 2160);
pub fn c12(x: UF!(1037, 0), y: UF!(1164, 0)) -> Fx<{ 1037 + 1164 }, u128, Warm> {
    mul(x, y)
}
const _: () = assert!(1037 + 1164 == 2201);
pub fn c13(x: UF!(1037, 0), y: UF!(1205, 0)) -> Fx<{ 1037 + 1205 }, u128, Warm> {
    mul(x, y)
}
const _: () = assert!(1037 + 1205 == 2242);
pub fn c14(x: UF!(1037, 0), y: UF!(1246, 0)) -> Fx<{ 1037 + 1246 }, u128, Warm> {
    mul(x, y)
}
const _: () = assert!(1037 + 1246 == 2283);
pub fn c15(x: UF!(1037, 0), y: UF!(1287, 0)) -> Fx<{ 1037 + 1287 }, u128, Warm> {
    mul(x, y)
}
const _: () = assert!(1037 + 1287 == 2324);
pub fn c16(x: UF!(1074, 0), y: UF!(1000, 0)) -> Fx<{ 1074 + 1000 }, u128, Warm> {
    mul(x, y)
}
const _: () = assert!(1074 + 1000 == 2074);
pub fn c17(x: UF!(1074, 0), y: UF!(1041, 0)) -> Fx<{ 1074 + 1041 }, u128, Warm> {
    mul(x, y)
}
const _: () = assert!(1074 + 1041 == 2115);
pub fn c18(x: UF!(1074, 0), y: UF!(1082, 0)) -> Fx<{ 1074 + 1082 }, u128, Warm> {
    mul(x, y)
}
const _: () = assert!(1074 + 1082 == 2156);
pub fn c19(x: UF!(1074, 0), y: UF!(1123, 0)) -> Fx<{ 1074 + 1123 }, u128, Warm> {
    mul(x, y)
}
const _: () = assert!(1074 + 1123 == 2197);
pub fn c20(x: UF!(1074, 0), y: UF!(1164, 0)) -> Fx<{ 1074 + 1164 }, u128, Warm> {
    mul(x, y)
}
const _: () = assert!(1074 + 1164 == 2238);
pub fn c21(x: UF!(1074, 0), y: UF!(1205, 0)) -> Fx<{ 1074 + 1205 }, u128, Warm> {
    mul(x, y)
}
const _: () = assert!(1074 + 1205 == 2279);
pub fn c22(x: UF!(1074, 0), y: UF!(1246, 0)) -> Fx<{ 1074 + 1246 }, u128, Warm> {
    mul(x, y)
}
const _: () = assert!(1074 + 1246 == 2320);
pub fn c23(x: UF!(1074, 0), y: UF!(1287, 0)) -> Fx<{ 1074 + 1287 }, u128, Warm> {
    mul(x, y)
}
const _: () = assert!(1074 + 1287 == 2361);
pub fn c24(x: UF!(1111, 0), y: UF!(1000, 0)) -> Fx<{ 1111 + 1000 }, u128, Warm> {
    mul(x, y)
}
const _: () = assert!(1111 + 1000 == 2111);
pub fn c25(x: UF!(1111, 0), y: UF!(1041, 0)) -> Fx<{ 1111 + 1041 }, u128, Warm> {
    mul(x, y)
}
const _: () = assert!(1111 + 1041 == 2152);
pub fn c26(x: UF!(1111, 0), y: UF!(1082, 0)) -> Fx<{ 1111 + 1082 }, u128, Warm> {
    mul(x, y)
}
const _: () = assert!(1111 + 1082 == 2193);
pub fn c27(x: UF!(1111, 0), y: UF!(1123, 0)) -> Fx<{ 1111 + 1123 }, u128, Warm> {
    mul(x, y)
}
const _: () = assert!(1111 + 1123 == 2234);
pub fn c28(x: UF!(1111, 0), y: UF!(1164, 0)) -> Fx<{ 1111 + 1164 }, u128, Warm> {
    mul(x, y)
}
const _: () = assert!(1111 + 1164 == 2275);
pub fn c29(x: UF!(1111, 0), y: UF!(1205, 0)) -> Fx<{ 1111 + 1205 }, u128, Warm> {
    mul(x, y)
}
const _: () = assert!(1111 + 1205 == 2316);
pub fn c30(x: UF!(1111, 0), y: UF!(1246, 0)) -> Fx<{ 1111 + 1246 }, u128, Warm> {
    mul(x, y)
}
const _: () = assert!(1111 + 1246 == 2357);
pub fn c31(x: UF!(1111, 0), y: UF!(1287, 0)) -> Fx<{ 1111 + 1287 }, u128, Warm> {
    mul(x, y)
}
const _: () = assert!(1111 + 1287 == 2398);
pub fn c32(x: UF!(1148, 0), y: UF!(1000, 0)) -> Fx<{ 1148 + 1000 }, u128, Warm> {
    mul(x, y)
}
const _: () = assert!(1148 + 1000 == 2148);
pub fn c33(x: UF!(1148, 0), y: UF!(1041, 0)) -> Fx<{ 1148 + 1041 }, u128, Warm> {
    mul(x, y)
}
const _: () = assert!(1148 + 1041 == 2189);
pub fn c34(x: UF!(1148, 0), y: UF!(1082, 0)) -> Fx<{ 1148 + 1082 }, u128, Warm> {
    mul(x, y)
}
const _: () = assert!(1148 + 1082 == 2230);
pub fn c35(x: UF!(1148, 0), y: UF!(1123, 0)) -> Fx<{ 1148 + 1123 }, u128, Warm> {
    mul(x, y)
}
const _: () = assert!(1148 + 1123 == 2271);
pub fn c36(x: UF!(1148, 0), y: UF!(1164, 0)) -> Fx<{ 1148 + 1164 }, u128, Warm> {
    mul(x, y)
}
const _: () = assert!(1148 + 1164 == 2312);
pub fn c37(x: UF!(1148, 0), y: UF!(1205, 0)) -> Fx<{ 1148 + 1205 }, u128, Warm> {
    mul(x, y)
}
const _: () = assert!(1148 + 1205 == 2353);
pub fn c38(x: UF!(1148, 0), y: UF!(1246, 0)) -> Fx<{ 1148 + 1246 }, u128, Warm> {
    mul(x, y)
}
const _: () = assert!(1148 + 1246 == 2394);
pub fn c39(x: UF!(1148, 0), y: UF!(1287, 0)) -> Fx<{ 1148 + 1287 }, u128, Warm> {
    mul(x, y)
}
const _: () = assert!(1148 + 1287 == 2435);
pub fn c40(x: UF!(1185, 0), y: UF!(1000, 0)) -> Fx<{ 1185 + 1000 }, u128, Warm> {
    mul(x, y)
}
const _: () = assert!(1185 + 1000 == 2185);
pub fn c41(x: UF!(1185, 0), y: UF!(1041, 0)) -> Fx<{ 1185 + 1041 }, u128, Warm> {
    mul(x, y)
}
const _: () = assert!(1185 + 1041 == 2226);
pub fn c42(x: UF!(1185, 0), y: UF!(1082, 0)) -> Fx<{ 1185 + 1082 }, u128, Warm> {
    mul(x, y)
}
const _: () = assert!(1185 + 1082 == 2267);
pub fn c43(x: UF!(1185, 0), y: UF!(1123, 0)) -> Fx<{ 1185 + 1123 }, u128, Warm> {
    mul(x, y)
}
const _: () = assert!(1185 + 1123 == 2308);
pub fn c44(x: UF!(1185, 0), y: UF!(1164, 0)) -> Fx<{ 1185 + 1164 }, u128, Warm> {
    mul(x, y)
}
const _: () = assert!(1185 + 1164 == 2349);
pub fn c45(x: UF!(1185, 0), y: UF!(1205, 0)) -> Fx<{ 1185 + 1205 }, u128, Warm> {
    mul(x, y)
}
const _: () = assert!(1185 + 1205 == 2390);
pub fn c46(x: UF!(1185, 0), y: UF!(1246, 0)) -> Fx<{ 1185 + 1246 }, u128, Warm> {
    mul(x, y)
}
const _: () = assert!(1185 + 1246 == 2431);
pub fn c47(x: UF!(1185, 0), y: UF!(1287, 0)) -> Fx<{ 1185 + 1287 }, u128, Warm> {
    mul(x, y)
}
const _: () = assert!(1185 + 1287 == 2472);
pub fn c48(x: UF!(1222, 0), y: UF!(1000, 0)) -> Fx<{ 1222 + 1000 }, u128, Warm> {
    mul(x, y)
}
const _: () = assert!(1222 + 1000 == 2222);
pub fn c49(x: UF!(1222, 0), y: UF!(1041, 0)) -> Fx<{ 1222 + 1041 }, u128, Warm> {
    mul(x, y)
}
const _: () = assert!(1222 + 1041 == 2263);
pub fn c50(x: UF!(1222, 0), y: UF!(1082, 0)) -> Fx<{ 1222 + 1082 }, u128, Warm> {
    mul(x, y)
}
const _: () = assert!(1222 + 1082 == 2304);
pub fn c51(x: UF!(1222, 0), y: UF!(1123, 0)) -> Fx<{ 1222 + 1123 }, u128, Warm> {
    mul(x, y)
}
const _: () = assert!(1222 + 1123 == 2345);
pub fn c52(x: UF!(1222, 0), y: UF!(1164, 0)) -> Fx<{ 1222 + 1164 }, u128, Warm> {
    mul(x, y)
}
const _: () = assert!(1222 + 1164 == 2386);
pub fn c53(x: UF!(1222, 0), y: UF!(1205, 0)) -> Fx<{ 1222 + 1205 }, u128, Warm> {
    mul(x, y)
}
const _: () = assert!(1222 + 1205 == 2427);
pub fn c54(x: UF!(1222, 0), y: UF!(1246, 0)) -> Fx<{ 1222 + 1246 }, u128, Warm> {
    mul(x, y)
}
const _: () = assert!(1222 + 1246 == 2468);
pub fn c55(x: UF!(1222, 0), y: UF!(1287, 0)) -> Fx<{ 1222 + 1287 }, u128, Warm> {
    mul(x, y)
}
const _: () = assert!(1222 + 1287 == 2509);
pub fn c56(x: UF!(1259, 0), y: UF!(1000, 0)) -> Fx<{ 1259 + 1000 }, u128, Warm> {
    mul(x, y)
}
const _: () = assert!(1259 + 1000 == 2259);
pub fn c57(x: UF!(1259, 0), y: UF!(1041, 0)) -> Fx<{ 1259 + 1041 }, u128, Warm> {
    mul(x, y)
}
const _: () = assert!(1259 + 1041 == 2300);
pub fn c58(x: UF!(1259, 0), y: UF!(1082, 0)) -> Fx<{ 1259 + 1082 }, u128, Warm> {
    mul(x, y)
}
const _: () = assert!(1259 + 1082 == 2341);
pub fn c59(x: UF!(1259, 0), y: UF!(1123, 0)) -> Fx<{ 1259 + 1123 }, u128, Warm> {
    mul(x, y)
}
const _: () = assert!(1259 + 1123 == 2382);
pub fn c60(x: UF!(1259, 0), y: UF!(1164, 0)) -> Fx<{ 1259 + 1164 }, u128, Warm> {
    mul(x, y)
}
const _: () = assert!(1259 + 1164 == 2423);
pub fn c61(x: UF!(1259, 0), y: UF!(1205, 0)) -> Fx<{ 1259 + 1205 }, u128, Warm> {
    mul(x, y)
}
const _: () = assert!(1259 + 1205 == 2464);
pub fn c62(x: UF!(1259, 0), y: UF!(1246, 0)) -> Fx<{ 1259 + 1246 }, u128, Warm> {
    mul(x, y)
}
const _: () = assert!(1259 + 1246 == 2505);
pub fn c63(x: UF!(1259, 0), y: UF!(1287, 0)) -> Fx<{ 1259 + 1287 }, u128, Warm> {
    mul(x, y)
}
const _: () = assert!(1259 + 1287 == 2546);
