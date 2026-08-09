#![no_std]
#![feature(min_generic_const_args, generic_const_args)]
#![allow(incomplete_features)]
pub struct W<const N: u32>;
pub trait Nat {
    const VAL: u32;
}
impl<const N: u32> Nat for W<N> {
    const VAL: u32 = N;
}
pub trait Add2<const I: u32, const F: u32> {
    type const SUM: u32;
}
pub struct Adder;
impl<const I: u32, const F: u32> Add2<I, F> for Adder {
    type const SUM: u32 = const { I + F };
}
pub type PrecisionOf<const I: u32, const F: u32> = W<{ <Adder as Add2<I, F>>::SUM }>;
const _: () = assert!(<PrecisionOf<1000, 1000> as Nat>::VAL == 9999);
const _: () = assert!(<PrecisionOf<1001, 1031> as Nat>::VAL == 2032);
const _: () = assert!(<PrecisionOf<1002, 1038> as Nat>::VAL == 2040);
const _: () = assert!(<PrecisionOf<1003, 1045> as Nat>::VAL == 2048);
const _: () = assert!(<PrecisionOf<1004, 1052> as Nat>::VAL == 2056);
const _: () = assert!(<PrecisionOf<1005, 1059> as Nat>::VAL == 2064);
const _: () = assert!(<PrecisionOf<1006, 1002> as Nat>::VAL == 2008);
const _: () = assert!(<PrecisionOf<1007, 1009> as Nat>::VAL == 2016);
const _: () = assert!(<PrecisionOf<1008, 1016> as Nat>::VAL == 2024);
const _: () = assert!(<PrecisionOf<1009, 1023> as Nat>::VAL == 2032);
const _: () = assert!(<PrecisionOf<1010, 1030> as Nat>::VAL == 2040);
const _: () = assert!(<PrecisionOf<1011, 1037> as Nat>::VAL == 2048);
const _: () = assert!(<PrecisionOf<1012, 1044> as Nat>::VAL == 2056);
const _: () = assert!(<PrecisionOf<1013, 1051> as Nat>::VAL == 2064);
const _: () = assert!(<PrecisionOf<1014, 1058> as Nat>::VAL == 2072);
const _: () = assert!(<PrecisionOf<1015, 1001> as Nat>::VAL == 2016);
const _: () = assert!(<PrecisionOf<1016, 1008> as Nat>::VAL == 2024);
const _: () = assert!(<PrecisionOf<1017, 1015> as Nat>::VAL == 2032);
const _: () = assert!(<PrecisionOf<1018, 1022> as Nat>::VAL == 2040);
const _: () = assert!(<PrecisionOf<1019, 1029> as Nat>::VAL == 2048);
const _: () = assert!(<PrecisionOf<1020, 1036> as Nat>::VAL == 2056);
const _: () = assert!(<PrecisionOf<1021, 1043> as Nat>::VAL == 2064);
const _: () = assert!(<PrecisionOf<1022, 1050> as Nat>::VAL == 2072);
const _: () = assert!(<PrecisionOf<1023, 1057> as Nat>::VAL == 2080);
const _: () = assert!(<PrecisionOf<1024, 1000> as Nat>::VAL == 2024);
const _: () = assert!(<PrecisionOf<1025, 1007> as Nat>::VAL == 2032);
const _: () = assert!(<PrecisionOf<1026, 1014> as Nat>::VAL == 2040);
const _: () = assert!(<PrecisionOf<1027, 1021> as Nat>::VAL == 2048);
const _: () = assert!(<PrecisionOf<1028, 1028> as Nat>::VAL == 2056);
const _: () = assert!(<PrecisionOf<1029, 1035> as Nat>::VAL == 2064);
const _: () = assert!(<PrecisionOf<1030, 1042> as Nat>::VAL == 2072);
const _: () = assert!(<PrecisionOf<1031, 1049> as Nat>::VAL == 2080);
const _: () = assert!(<PrecisionOf<1032, 1056> as Nat>::VAL == 2088);
const _: () = assert!(<PrecisionOf<1033, 1063> as Nat>::VAL == 2096);
const _: () = assert!(<PrecisionOf<1034, 1006> as Nat>::VAL == 2040);
const _: () = assert!(<PrecisionOf<1035, 1013> as Nat>::VAL == 2048);
const _: () = assert!(<PrecisionOf<1036, 1020> as Nat>::VAL == 2056);
const _: () = assert!(<PrecisionOf<1037, 1027> as Nat>::VAL == 2064);
const _: () = assert!(<PrecisionOf<1038, 1034> as Nat>::VAL == 2072);
const _: () = assert!(<PrecisionOf<1039, 1041> as Nat>::VAL == 2080);
const _: () = assert!(<PrecisionOf<1040, 1048> as Nat>::VAL == 2088);
const _: () = assert!(<PrecisionOf<1041, 1055> as Nat>::VAL == 2096);
const _: () = assert!(<PrecisionOf<1042, 1062> as Nat>::VAL == 2104);
const _: () = assert!(<PrecisionOf<1043, 1005> as Nat>::VAL == 2048);
const _: () = assert!(<PrecisionOf<1044, 1012> as Nat>::VAL == 2056);
const _: () = assert!(<PrecisionOf<1045, 1019> as Nat>::VAL == 2064);
const _: () = assert!(<PrecisionOf<1046, 1026> as Nat>::VAL == 2072);
const _: () = assert!(<PrecisionOf<1047, 1033> as Nat>::VAL == 2080);
const _: () = assert!(<PrecisionOf<1048, 1040> as Nat>::VAL == 2088);
const _: () = assert!(<PrecisionOf<1049, 1047> as Nat>::VAL == 2096);
const _: () = assert!(<PrecisionOf<1050, 1054> as Nat>::VAL == 2104);
const _: () = assert!(<PrecisionOf<1051, 1061> as Nat>::VAL == 2112);
const _: () = assert!(<PrecisionOf<1052, 1004> as Nat>::VAL == 2056);
const _: () = assert!(<PrecisionOf<1053, 1011> as Nat>::VAL == 2064);
const _: () = assert!(<PrecisionOf<1054, 1018> as Nat>::VAL == 2072);
const _: () = assert!(<PrecisionOf<1055, 1025> as Nat>::VAL == 2080);
const _: () = assert!(<PrecisionOf<1056, 1032> as Nat>::VAL == 2088);
const _: () = assert!(<PrecisionOf<1057, 1039> as Nat>::VAL == 2096);
const _: () = assert!(<PrecisionOf<1058, 1046> as Nat>::VAL == 2104);
const _: () = assert!(<PrecisionOf<1059, 1053> as Nat>::VAL == 2112);
const _: () = assert!(<PrecisionOf<1060, 1060> as Nat>::VAL == 2120);
const _: () = assert!(<PrecisionOf<1061, 1003> as Nat>::VAL == 2064);
const _: () = assert!(<PrecisionOf<1062, 1010> as Nat>::VAL == 2072);
const _: () = assert!(<PrecisionOf<1063, 1017> as Nat>::VAL == 2080);
pub fn takes_2064(_: W<2064>) {}
pub fn canon() {
    takes_2064(PrecisionOf::<1032, 1032> {});
    takes_2064(PrecisionOf::<1000, 1064> {});
}
