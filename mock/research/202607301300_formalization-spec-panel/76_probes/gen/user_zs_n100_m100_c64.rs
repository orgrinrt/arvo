#![no_std]
use mach::*;
pub type Ib0 = Pz<H>;
pub type Fb0 = Z;
pub type Wb0 = Pz<H>;
pub type N0 = UFixed<Ib0, Fb0, Hot>;
pub const W0: usize = <Wb0 as Nat>::VAL;
const _: () = assert!(W0 == 1);
pub fn one_ok_0()
where
    N0: HasOne,
{
}
pub fn int_like_0()
where
    Fb0: IsZero,
{
}
pub type Ib1 = Pz<O<H>>;
pub type Fb1 = Z;
pub type Wb1 = Pz<O<H>>;
pub type N1 = UFixed<Ib1, Fb1, Warm>;
pub const W1: usize = <Wb1 as Nat>::VAL;
const _: () = assert!(W1 == 2);
pub fn one_ok_1()
where
    N1: HasOne,
{
}
pub fn int_like_1()
where
    Fb1: IsZero,
{
}
pub type Ib2 = Pz<I<H>>;
pub type Fb2 = Z;
pub type Wb2 = Pz<I<H>>;
pub type N2 = UFixed<Ib2, Fb2, Cold>;
pub const W2: usize = <Wb2 as Nat>::VAL;
const _: () = assert!(W2 == 3);
pub fn one_ok_2()
where
    N2: HasOne,
{
}
pub fn int_like_2()
where
    Fb2: IsZero,
{
}
pub type Ib3 = Pz<O<O<H>>>;
pub type Fb3 = Z;
pub type Wb3 = Pz<O<O<H>>>;
pub type N3 = UFixed<Ib3, Fb3, Hot>;
pub const W3: usize = <Wb3 as Nat>::VAL;
const _: () = assert!(W3 == 4);
pub fn one_ok_3()
where
    N3: HasOne,
{
}
pub fn int_like_3()
where
    Fb3: IsZero,
{
}
pub type Ib4 = Pz<I<O<H>>>;
pub type Fb4 = Z;
pub type Wb4 = Pz<I<O<H>>>;
pub type N4 = UFixed<Ib4, Fb4, Warm>;
pub const W4: usize = <Wb4 as Nat>::VAL;
const _: () = assert!(W4 == 5);
pub fn one_ok_4()
where
    N4: HasOne,
{
}
pub fn int_like_4()
where
    Fb4: IsZero,
{
}
pub type Ib5 = Pz<O<I<H>>>;
pub type Fb5 = Z;
pub type Wb5 = Pz<O<I<H>>>;
pub type N5 = UFixed<Ib5, Fb5, Cold>;
pub const W5: usize = <Wb5 as Nat>::VAL;
const _: () = assert!(W5 == 6);
pub fn one_ok_5()
where
    N5: HasOne,
{
}
pub fn int_like_5()
where
    Fb5: IsZero,
{
}
pub type Ib6 = Pz<I<I<H>>>;
pub type Fb6 = Z;
pub type Wb6 = Pz<I<I<H>>>;
pub type N6 = UFixed<Ib6, Fb6, Hot>;
pub const W6: usize = <Wb6 as Nat>::VAL;
const _: () = assert!(W6 == 7);
pub fn one_ok_6()
where
    N6: HasOne,
{
}
pub fn int_like_6()
where
    Fb6: IsZero,
{
}
pub type Ib7 = Pz<I<I<O<H>>>>;
pub type Fb7 = Z;
pub type Wb7 = Pz<I<I<O<H>>>>;
pub type N7 = UFixed<Ib7, Fb7, Warm>;
pub const W7: usize = <Wb7 as Nat>::VAL;
const _: () = assert!(W7 == 11);
pub fn one_ok_7()
where
    N7: HasOne,
{
}
pub fn int_like_7()
where
    Fb7: IsZero,
{
}
pub type Ib8 = Pz<O<I<I<H>>>>;
pub type Fb8 = Z;
pub type Wb8 = Pz<O<I<I<H>>>>;
pub type N8 = UFixed<Ib8, Fb8, Cold>;
pub const W8: usize = <Wb8 as Nat>::VAL;
const _: () = assert!(W8 == 14);
pub fn one_ok_8()
where
    N8: HasOne,
{
}
pub fn int_like_8()
where
    Fb8: IsZero,
{
}
pub type Ib9 = Pz<O<O<O<O<H>>>>>;
pub type Fb9 = Z;
pub type Wb9 = Pz<O<O<O<O<H>>>>>;
pub type N9 = UFixed<Ib9, Fb9, Hot>;
pub const W9: usize = <Wb9 as Nat>::VAL;
const _: () = assert!(W9 == 16);
pub fn one_ok_9()
where
    N9: HasOne,
{
}
pub fn int_like_9()
where
    Fb9: IsZero,
{
}
pub type Ib10 = Pz<I<I<O<I<H>>>>>;
pub type Fb10 = Z;
pub type Wb10 = Pz<I<I<O<I<H>>>>>;
pub type N10 = UFixed<Ib10, Fb10, Warm>;
pub const W10: usize = <Wb10 as Nat>::VAL;
const _: () = assert!(W10 == 27);
pub fn one_ok_10()
where
    N10: HasOne,
{
}
pub fn int_like_10()
where
    Fb10: IsZero,
{
}
pub type Ib11 = Pz<O<O<I<I<H>>>>>;
pub type Fb11 = Z;
pub type Wb11 = Pz<O<O<I<I<H>>>>>;
pub type N11 = UFixed<Ib11, Fb11, Cold>;
pub const W11: usize = <Wb11 as Nat>::VAL;
const _: () = assert!(W11 == 28);
pub fn one_ok_11()
where
    N11: HasOne,
{
}
pub fn int_like_11()
where
    Fb11: IsZero,
{
}
pub type Ib12 = Pz<O<O<O<O<O<O<H>>>>>>>;
pub type Fb12 = Z;
pub type Wb12 = Pz<O<O<O<O<O<O<H>>>>>>>;
pub type N12 = UFixed<Ib12, Fb12, Hot>;
pub const W12: usize = <Wb12 as Nat>::VAL;
const _: () = assert!(W12 == 64);
pub fn one_ok_12()
where
    N12: HasOne,
{
}
pub fn int_like_12()
where
    Fb12: IsZero,
{
}
pub type Ib13 = Z;
pub type Fb13 = Pz<O<O<O<O<H>>>>>;
pub type Wb13 = Pz<O<O<O<O<H>>>>>;
pub type N13 = UFixed<Ib13, Fb13, Warm>;
pub const W13: usize = <Wb13 as Nat>::VAL;
const _: () = assert!(W13 == 16);
pub fn frac_like_13()
where
    Fb13: NonZero,
{
}
pub type Ib14 = Pz<O<O<O<H>>>>;
pub type Fb14 = Z;
pub type Wb14 = Pz<O<O<O<H>>>>;
pub type N14 = UFixed<Ib14, Fb14, Cold>;
pub const W14: usize = <Wb14 as Nat>::VAL;
const _: () = assert!(W14 == 8);
pub fn one_ok_14()
where
    N14: HasOne,
{
}
pub fn int_like_14()
where
    Fb14: IsZero,
{
}
pub type Ib15 = Pz<I<O<O<H>>>>;
pub type Fb15 = Z;
pub type Wb15 = Pz<I<O<O<H>>>>;
pub type N15 = UFixed<Ib15, Fb15, Hot>;
pub const W15: usize = <Wb15 as Nat>::VAL;
const _: () = assert!(W15 == 9);
pub fn one_ok_15()
where
    N15: HasOne,
{
}
pub fn int_like_15()
where
    Fb15: IsZero,
{
}
pub type Ib16 = Pz<O<I<O<H>>>>;
pub type Fb16 = Z;
pub type Wb16 = Pz<O<I<O<H>>>>;
pub type N16 = UFixed<Ib16, Fb16, Warm>;
pub const W16: usize = <Wb16 as Nat>::VAL;
const _: () = assert!(W16 == 10);
pub fn one_ok_16()
where
    N16: HasOne,
{
}
pub fn int_like_16()
where
    Fb16: IsZero,
{
}
pub type Ib17 = Pz<O<O<I<H>>>>;
pub type Fb17 = Z;
pub type Wb17 = Pz<O<O<I<H>>>>;
pub type N17 = UFixed<Ib17, Fb17, Cold>;
pub const W17: usize = <Wb17 as Nat>::VAL;
const _: () = assert!(W17 == 12);
pub fn one_ok_17()
where
    N17: HasOne,
{
}
pub fn int_like_17()
where
    Fb17: IsZero,
{
}
pub type Ib18 = Pz<I<O<I<H>>>>;
pub type Fb18 = Z;
pub type Wb18 = Pz<I<O<I<H>>>>;
pub type N18 = UFixed<Ib18, Fb18, Hot>;
pub const W18: usize = <Wb18 as Nat>::VAL;
const _: () = assert!(W18 == 13);
pub fn one_ok_18()
where
    N18: HasOne,
{
}
pub fn int_like_18()
where
    Fb18: IsZero,
{
}
pub type Ib19 = Pz<O<I<O<O<H>>>>>;
pub type Fb19 = Z;
pub type Wb19 = Pz<O<I<O<O<H>>>>>;
pub type N19 = UFixed<Ib19, Fb19, Warm>;
pub const W19: usize = <Wb19 as Nat>::VAL;
const _: () = assert!(W19 == 18);
pub fn one_ok_19()
where
    N19: HasOne,
{
}
pub fn int_like_19()
where
    Fb19: IsZero,
{
}
pub type Ib20 = Pz<I<O<I<O<H>>>>>;
pub type Fb20 = Z;
pub type Wb20 = Pz<I<O<I<O<H>>>>>;
pub type N20 = UFixed<Ib20, Fb20, Cold>;
pub const W20: usize = <Wb20 as Nat>::VAL;
const _: () = assert!(W20 == 21);
pub fn one_ok_20()
where
    N20: HasOne,
{
}
pub fn int_like_20()
where
    Fb20: IsZero,
{
}
pub type Ib21 = Pz<I<I<I<O<H>>>>>;
pub type Fb21 = Z;
pub type Wb21 = Pz<I<I<I<O<H>>>>>;
pub type N21 = UFixed<Ib21, Fb21, Hot>;
pub const W21: usize = <Wb21 as Nat>::VAL;
const _: () = assert!(W21 == 23);
pub fn one_ok_21()
where
    N21: HasOne,
{
}
pub fn int_like_21()
where
    Fb21: IsZero,
{
}
pub type Ib22 = Pz<O<I<O<O<O<H>>>>>>;
pub type Fb22 = Z;
pub type Wb22 = Pz<O<I<O<O<O<H>>>>>>;
pub type N22 = UFixed<Ib22, Fb22, Warm>;
pub const W22: usize = <Wb22 as Nat>::VAL;
const _: () = assert!(W22 == 34);
pub fn one_ok_22()
where
    N22: HasOne,
{
}
pub fn int_like_22()
where
    Fb22: IsZero,
{
}
pub type Ib23 = Pz<I<I<O<O<O<H>>>>>>;
pub type Fb23 = Z;
pub type Wb23 = Pz<I<I<O<O<O<H>>>>>>;
pub type N23 = UFixed<Ib23, Fb23, Cold>;
pub const W23: usize = <Wb23 as Nat>::VAL;
const _: () = assert!(W23 == 35);
pub fn one_ok_23()
where
    N23: HasOne,
{
}
pub fn int_like_23()
where
    Fb23: IsZero,
{
}
pub type Ib24 = Pz<I<I<I<O<O<O<H>>>>>>>;
pub type Fb24 = Z;
pub type Wb24 = Pz<I<I<I<O<O<O<H>>>>>>>;
pub type N24 = UFixed<Ib24, Fb24, Hot>;
pub const W24: usize = <Wb24 as Nat>::VAL;
const _: () = assert!(W24 == 71);
pub fn one_ok_24()
where
    N24: HasOne,
{
}
pub fn int_like_24()
where
    Fb24: IsZero,
{
}
pub type Ib25 = Z;
pub type Fb25 = Pz<I<O<O<O<H>>>>>;
pub type Wb25 = Pz<I<O<O<O<H>>>>>;
pub type N25 = UFixed<Ib25, Fb25, Warm>;
pub const W25: usize = <Wb25 as Nat>::VAL;
const _: () = assert!(W25 == 17);
pub fn frac_like_25()
where
    Fb25: NonZero,
{
}
pub type Ib26 = Pz<I<I<I<H>>>>;
pub type Fb26 = Z;
pub type Wb26 = Pz<I<I<I<H>>>>;
pub type N26 = UFixed<Ib26, Fb26, Cold>;
pub const W26: usize = <Wb26 as Nat>::VAL;
const _: () = assert!(W26 == 15);
pub fn one_ok_26()
where
    N26: HasOne,
{
}
pub fn int_like_26()
where
    Fb26: IsZero,
{
}
pub type Ib27 = Pz<I<O<O<O<H>>>>>;
pub type Fb27 = Z;
pub type Wb27 = Pz<I<O<O<O<H>>>>>;
pub type N27 = UFixed<Ib27, Fb27, Hot>;
pub const W27: usize = <Wb27 as Nat>::VAL;
const _: () = assert!(W27 == 17);
pub fn one_ok_27()
where
    N27: HasOne,
{
}
pub fn int_like_27()
where
    Fb27: IsZero,
{
}
pub type Ib28 = Pz<I<I<O<O<H>>>>>;
pub type Fb28 = Z;
pub type Wb28 = Pz<I<I<O<O<H>>>>>;
pub type N28 = UFixed<Ib28, Fb28, Warm>;
pub const W28: usize = <Wb28 as Nat>::VAL;
const _: () = assert!(W28 == 19);
pub fn one_ok_28()
where
    N28: HasOne,
{
}
pub fn int_like_28()
where
    Fb28: IsZero,
{
}
pub type Ib29 = Pz<O<O<I<O<H>>>>>;
pub type Fb29 = Z;
pub type Wb29 = Pz<O<O<I<O<H>>>>>;
pub type N29 = UFixed<Ib29, Fb29, Cold>;
pub const W29: usize = <Wb29 as Nat>::VAL;
const _: () = assert!(W29 == 20);
pub fn one_ok_29()
where
    N29: HasOne,
{
}
pub fn int_like_29()
where
    Fb29: IsZero,
{
}
pub type Ib30 = Pz<I<O<O<I<H>>>>>;
pub type Fb30 = Z;
pub type Wb30 = Pz<I<O<O<I<H>>>>>;
pub type N30 = UFixed<Ib30, Fb30, Hot>;
pub const W30: usize = <Wb30 as Nat>::VAL;
const _: () = assert!(W30 == 25);
pub fn one_ok_30()
where
    N30: HasOne,
{
}
pub fn int_like_30()
where
    Fb30: IsZero,
{
}
pub type Ib31 = Pz<O<I<I<I<H>>>>>;
pub type Fb31 = Z;
pub type Wb31 = Pz<O<I<I<I<H>>>>>;
pub type N31 = UFixed<Ib31, Fb31, Warm>;
pub const W31: usize = <Wb31 as Nat>::VAL;
const _: () = assert!(W31 == 30);
pub fn one_ok_31()
where
    N31: HasOne,
{
}
pub fn int_like_31()
where
    Fb31: IsZero,
{
}
pub type Ib32 = Pz<I<O<O<I<O<H>>>>>>;
pub type Fb32 = Z;
pub type Wb32 = Pz<I<O<O<I<O<H>>>>>>;
pub type N32 = UFixed<Ib32, Fb32, Cold>;
pub const W32: usize = <Wb32 as Nat>::VAL;
const _: () = assert!(W32 == 41);
pub fn one_ok_32()
where
    N32: HasOne,
{
}
pub fn int_like_32()
where
    Fb32: IsZero,
{
}
pub type Ib33 = Pz<O<I<O<I<O<H>>>>>>;
pub type Fb33 = Z;
pub type Wb33 = Pz<O<I<O<I<O<H>>>>>>;
pub type N33 = UFixed<Ib33, Fb33, Hot>;
pub const W33: usize = <Wb33 as Nat>::VAL;
const _: () = assert!(W33 == 42);
pub fn one_ok_33()
where
    N33: HasOne,
{
}
pub fn int_like_33()
where
    Fb33: IsZero,
{
}
pub type Ib34 = Pz<O<I<I<I<O<O<H>>>>>>>;
pub type Fb34 = Z;
pub type Wb34 = Pz<O<I<I<I<O<O<H>>>>>>>;
pub type N34 = UFixed<Ib34, Fb34, Warm>;
pub const W34: usize = <Wb34 as Nat>::VAL;
const _: () = assert!(W34 == 78);
pub fn one_ok_34()
where
    N34: HasOne,
{
}
pub fn int_like_34()
where
    Fb34: IsZero,
{
}
pub type Ib35 = Z;
pub type Fb35 = Pz<O<I<O<O<H>>>>>;
pub type Wb35 = Pz<O<I<O<O<H>>>>>;
pub type N35 = UFixed<Ib35, Fb35, Cold>;
pub const W35: usize = <Wb35 as Nat>::VAL;
const _: () = assert!(W35 == 18);
pub fn frac_like_35()
where
    Fb35: NonZero,
{
}
pub type Ib36 = Pz<O<I<I<O<H>>>>>;
pub type Fb36 = Z;
pub type Wb36 = Pz<O<I<I<O<H>>>>>;
pub type N36 = UFixed<Ib36, Fb36, Hot>;
pub const W36: usize = <Wb36 as Nat>::VAL;
const _: () = assert!(W36 == 22);
pub fn one_ok_36()
where
    N36: HasOne,
{
}
pub fn int_like_36()
where
    Fb36: IsZero,
{
}
pub type Ib37 = Pz<O<O<O<I<H>>>>>;
pub type Fb37 = Z;
pub type Wb37 = Pz<O<O<O<I<H>>>>>;
pub type N37 = UFixed<Ib37, Fb37, Warm>;
pub const W37: usize = <Wb37 as Nat>::VAL;
const _: () = assert!(W37 == 24);
pub fn one_ok_37()
where
    N37: HasOne,
{
}
pub fn int_like_37()
where
    Fb37: IsZero,
{
}
pub type Ib38 = Pz<O<I<O<I<H>>>>>;
pub type Fb38 = Z;
pub type Wb38 = Pz<O<I<O<I<H>>>>>;
pub type N38 = UFixed<Ib38, Fb38, Cold>;
pub const W38: usize = <Wb38 as Nat>::VAL;
const _: () = assert!(W38 == 26);
pub fn one_ok_38()
where
    N38: HasOne,
{
}
pub fn int_like_38()
where
    Fb38: IsZero,
{
}
pub type Ib39 = Pz<O<O<O<O<O<H>>>>>>;
pub type Fb39 = Z;
pub type Wb39 = Pz<O<O<O<O<O<H>>>>>>;
pub type N39 = UFixed<Ib39, Fb39, Hot>;
pub const W39: usize = <Wb39 as Nat>::VAL;
const _: () = assert!(W39 == 32);
pub fn one_ok_39()
where
    N39: HasOne,
{
}
pub fn int_like_39()
where
    Fb39: IsZero,
{
}
pub type Ib40 = Pz<I<O<I<O<O<H>>>>>>;
pub type Fb40 = Z;
pub type Wb40 = Pz<I<O<I<O<O<H>>>>>>;
pub type N40 = UFixed<Ib40, Fb40, Warm>;
pub const W40: usize = <Wb40 as Nat>::VAL;
const _: () = assert!(W40 == 37);
pub fn one_ok_40()
where
    N40: HasOne,
{
}
pub fn int_like_40()
where
    Fb40: IsZero,
{
}
pub type Ib41 = Pz<O<O<O<O<I<H>>>>>>;
pub type Fb41 = Z;
pub type Wb41 = Pz<O<O<O<O<I<H>>>>>>;
pub type N41 = UFixed<Ib41, Fb41, Cold>;
pub const W41: usize = <Wb41 as Nat>::VAL;
const _: () = assert!(W41 == 48);
pub fn one_ok_41()
where
    N41: HasOne,
{
}
pub fn int_like_41()
where
    Fb41: IsZero,
{
}
pub type Ib42 = Pz<I<O<O<O<I<H>>>>>>;
pub type Fb42 = Z;
pub type Wb42 = Pz<I<O<O<O<I<H>>>>>>;
pub type N42 = UFixed<Ib42, Fb42, Hot>;
pub const W42: usize = <Wb42 as Nat>::VAL;
const _: () = assert!(W42 == 49);
pub fn one_ok_42()
where
    N42: HasOne,
{
}
pub fn int_like_42()
where
    Fb42: IsZero,
{
}
pub type Ib43 = Pz<I<O<I<O<I<O<H>>>>>>>;
pub type Fb43 = Z;
pub type Wb43 = Pz<I<O<I<O<I<O<H>>>>>>>;
pub type N43 = UFixed<Ib43, Fb43, Warm>;
pub const W43: usize = <Wb43 as Nat>::VAL;
const _: () = assert!(W43 == 85);
pub fn one_ok_43()
where
    N43: HasOne,
{
}
pub fn int_like_43()
where
    Fb43: IsZero,
{
}
pub type Ib44 = Z;
pub type Fb44 = Pz<I<I<O<O<H>>>>>;
pub type Wb44 = Pz<I<I<O<O<H>>>>>;
pub type N44 = UFixed<Ib44, Fb44, Cold>;
pub const W44: usize = <Wb44 as Nat>::VAL;
const _: () = assert!(W44 == 19);
pub fn frac_like_44()
where
    Fb44: NonZero,
{
}
pub type Ib45 = Pz<I<O<I<I<H>>>>>;
pub type Fb45 = Z;
pub type Wb45 = Pz<I<O<I<I<H>>>>>;
pub type N45 = UFixed<Ib45, Fb45, Hot>;
pub const W45: usize = <Wb45 as Nat>::VAL;
const _: () = assert!(W45 == 29);
pub fn one_ok_45()
where
    N45: HasOne,
{
}
pub fn int_like_45()
where
    Fb45: IsZero,
{
}
pub type Ib46 = Pz<I<I<I<I<H>>>>>;
pub type Fb46 = Z;
pub type Wb46 = Pz<I<I<I<I<H>>>>>;
pub type N46 = UFixed<Ib46, Fb46, Warm>;
pub const W46: usize = <Wb46 as Nat>::VAL;
const _: () = assert!(W46 == 31);
pub fn one_ok_46()
where
    N46: HasOne,
{
}
pub fn int_like_46()
where
    Fb46: IsZero,
{
}
pub type Ib47 = Pz<I<O<O<O<O<H>>>>>>;
pub type Fb47 = Z;
pub type Wb47 = Pz<I<O<O<O<O<H>>>>>>;
pub type N47 = UFixed<Ib47, Fb47, Cold>;
pub const W47: usize = <Wb47 as Nat>::VAL;
const _: () = assert!(W47 == 33);
pub fn one_ok_47()
where
    N47: HasOne,
{
}
pub fn int_like_47()
where
    Fb47: IsZero,
{
}
pub type Ib48 = Pz<I<I<I<O<O<H>>>>>>;
pub type Fb48 = Z;
pub type Wb48 = Pz<I<I<I<O<O<H>>>>>>;
pub type N48 = UFixed<Ib48, Fb48, Hot>;
pub const W48: usize = <Wb48 as Nat>::VAL;
const _: () = assert!(W48 == 39);
pub fn one_ok_48()
where
    N48: HasOne,
{
}
pub fn int_like_48()
where
    Fb48: IsZero,
{
}
pub type Ib49 = Pz<O<O<I<I<O<H>>>>>>;
pub type Fb49 = Z;
pub type Wb49 = Pz<O<O<I<I<O<H>>>>>>;
pub type N49 = UFixed<Ib49, Fb49, Warm>;
pub const W49: usize = <Wb49 as Nat>::VAL;
const _: () = assert!(W49 == 44);
pub fn one_ok_49()
where
    N49: HasOne,
{
}
pub fn int_like_49()
where
    Fb49: IsZero,
{
}
pub type Ib50 = Pz<I<I<I<O<I<H>>>>>>;
pub type Fb50 = Z;
pub type Wb50 = Pz<I<I<I<O<I<H>>>>>>;
pub type N50 = UFixed<Ib50, Fb50, Cold>;
pub const W50: usize = <Wb50 as Nat>::VAL;
const _: () = assert!(W50 == 55);
pub fn one_ok_50()
where
    N50: HasOne,
{
}
pub fn int_like_50()
where
    Fb50: IsZero,
{
}
pub type Ib51 = Pz<O<O<O<I<I<H>>>>>>;
pub type Fb51 = Z;
pub type Wb51 = Pz<O<O<O<I<I<H>>>>>>;
pub type N51 = UFixed<Ib51, Fb51, Hot>;
pub const W51: usize = <Wb51 as Nat>::VAL;
const _: () = assert!(W51 == 56);
pub fn one_ok_51()
where
    N51: HasOne,
{
}
pub fn int_like_51()
where
    Fb51: IsZero,
{
}
pub type Ib52 = Pz<O<O<I<I<I<O<H>>>>>>>;
pub type Fb52 = Z;
pub type Wb52 = Pz<O<O<I<I<I<O<H>>>>>>>;
pub type N52 = UFixed<Ib52, Fb52, Warm>;
pub const W52: usize = <Wb52 as Nat>::VAL;
const _: () = assert!(W52 == 92);
pub fn one_ok_52()
where
    N52: HasOne,
{
}
pub fn int_like_52()
where
    Fb52: IsZero,
{
}
pub type Ib53 = Z;
pub type Fb53 = Pz<O<O<I<O<H>>>>>;
pub type Wb53 = Pz<O<O<I<O<H>>>>>;
pub type N53 = UFixed<Ib53, Fb53, Cold>;
pub const W53: usize = <Wb53 as Nat>::VAL;
const _: () = assert!(W53 == 20);
pub fn frac_like_53()
where
    Fb53: NonZero,
{
}
pub type Ib54 = Pz<O<O<I<O<O<H>>>>>>;
pub type Fb54 = Z;
pub type Wb54 = Pz<O<O<I<O<O<H>>>>>>;
pub type N54 = UFixed<Ib54, Fb54, Hot>;
pub const W54: usize = <Wb54 as Nat>::VAL;
const _: () = assert!(W54 == 36);
pub fn one_ok_54()
where
    N54: HasOne,
{
}
pub fn int_like_54()
where
    Fb54: IsZero,
{
}
pub type Ib55 = Pz<O<I<I<O<O<H>>>>>>;
pub type Fb55 = Z;
pub type Wb55 = Pz<O<I<I<O<O<H>>>>>>;
pub type N55 = UFixed<Ib55, Fb55, Warm>;
pub const W55: usize = <Wb55 as Nat>::VAL;
const _: () = assert!(W55 == 38);
pub fn one_ok_55()
where
    N55: HasOne,
{
}
pub fn int_like_55()
where
    Fb55: IsZero,
{
}
pub type Ib56 = Pz<O<O<O<I<O<H>>>>>>;
pub type Fb56 = Z;
pub type Wb56 = Pz<O<O<O<I<O<H>>>>>>;
pub type N56 = UFixed<Ib56, Fb56, Cold>;
pub const W56: usize = <Wb56 as Nat>::VAL;
const _: () = assert!(W56 == 40);
pub fn one_ok_56()
where
    N56: HasOne,
{
}
pub fn int_like_56()
where
    Fb56: IsZero,
{
}
pub type Ib57 = Pz<O<I<I<I<O<H>>>>>>;
pub type Fb57 = Z;
pub type Wb57 = Pz<O<I<I<I<O<H>>>>>>;
pub type N57 = UFixed<Ib57, Fb57, Hot>;
pub const W57: usize = <Wb57 as Nat>::VAL;
const _: () = assert!(W57 == 46);
pub fn one_ok_57()
where
    N57: HasOne,
{
}
pub fn int_like_57()
where
    Fb57: IsZero,
{
}
pub type Ib58 = Pz<I<I<O<O<I<H>>>>>>;
pub type Fb58 = Z;
pub type Wb58 = Pz<I<I<O<O<I<H>>>>>>;
pub type N58 = UFixed<Ib58, Fb58, Warm>;
pub const W58: usize = <Wb58 as Nat>::VAL;
const _: () = assert!(W58 == 51);
pub fn one_ok_58()
where
    N58: HasOne,
{
}
pub fn int_like_58()
where
    Fb58: IsZero,
{
}
pub type Ib59 = Pz<O<I<I<I<I<H>>>>>>;
pub type Fb59 = Z;
pub type Wb59 = Pz<O<I<I<I<I<H>>>>>>;
pub type N59 = UFixed<Ib59, Fb59, Cold>;
pub const W59: usize = <Wb59 as Nat>::VAL;
const _: () = assert!(W59 == 62);
pub fn one_ok_59()
where
    N59: HasOne,
{
}
pub fn int_like_59()
where
    Fb59: IsZero,
{
}
pub type Ib60 = Pz<I<I<I<I<I<H>>>>>>;
pub type Fb60 = Z;
pub type Wb60 = Pz<I<I<I<I<I<H>>>>>>;
pub type N60 = UFixed<Ib60, Fb60, Hot>;
pub const W60: usize = <Wb60 as Nat>::VAL;
const _: () = assert!(W60 == 63);
pub fn one_ok_60()
where
    N60: HasOne,
{
}
pub fn int_like_60()
where
    Fb60: IsZero,
{
}
pub type Ib61 = Pz<I<I<O<O<O<I<H>>>>>>>;
pub type Fb61 = Z;
pub type Wb61 = Pz<I<I<O<O<O<I<H>>>>>>>;
pub type N61 = UFixed<Ib61, Fb61, Warm>;
pub const W61: usize = <Wb61 as Nat>::VAL;
const _: () = assert!(W61 == 99);
pub fn one_ok_61()
where
    N61: HasOne,
{
}
pub fn int_like_61()
where
    Fb61: IsZero,
{
}
pub type Ib62 = Z;
pub type Fb62 = Pz<I<O<I<O<H>>>>>;
pub type Wb62 = Pz<I<O<I<O<H>>>>>;
pub type N62 = UFixed<Ib62, Fb62, Cold>;
pub const W62: usize = <Wb62 as Nat>::VAL;
const _: () = assert!(W62 == 21);
pub fn frac_like_62()
where
    Fb62: NonZero,
{
}
pub type Ib63 = Pz<I<I<O<I<O<H>>>>>>;
pub type Fb63 = Z;
pub type Wb63 = Pz<I<I<O<I<O<H>>>>>>;
pub type N63 = UFixed<Ib63, Fb63, Hot>;
pub const W63: usize = <Wb63 as Nat>::VAL;
const _: () = assert!(W63 == 43);
pub fn one_ok_63()
where
    N63: HasOne,
{
}
pub fn int_like_63()
where
    Fb63: IsZero,
{
}
pub type Ib64 = Pz<I<O<I<I<O<H>>>>>>;
pub type Fb64 = Z;
pub type Wb64 = Pz<I<O<I<I<O<H>>>>>>;
pub type N64 = UFixed<Ib64, Fb64, Warm>;
pub const W64: usize = <Wb64 as Nat>::VAL;
const _: () = assert!(W64 == 45);
pub fn one_ok_64()
where
    N64: HasOne,
{
}
pub fn int_like_64()
where
    Fb64: IsZero,
{
}
pub type Ib65 = Pz<I<I<I<I<O<H>>>>>>;
pub type Fb65 = Z;
pub type Wb65 = Pz<I<I<I<I<O<H>>>>>>;
pub type N65 = UFixed<Ib65, Fb65, Cold>;
pub const W65: usize = <Wb65 as Nat>::VAL;
const _: () = assert!(W65 == 47);
pub fn one_ok_65()
where
    N65: HasOne,
{
}
pub fn int_like_65()
where
    Fb65: IsZero,
{
}
pub type Ib66 = Pz<I<O<I<O<I<H>>>>>>;
pub type Fb66 = Z;
pub type Wb66 = Pz<I<O<I<O<I<H>>>>>>;
pub type N66 = UFixed<Ib66, Fb66, Hot>;
pub const W66: usize = <Wb66 as Nat>::VAL;
const _: () = assert!(W66 == 53);
pub fn one_ok_66()
where
    N66: HasOne,
{
}
pub fn int_like_66()
where
    Fb66: IsZero,
{
}
pub type Ib67 = Pz<O<I<O<I<I<H>>>>>>;
pub type Fb67 = Z;
pub type Wb67 = Pz<O<I<O<I<I<H>>>>>>;
pub type N67 = UFixed<Ib67, Fb67, Warm>;
pub const W67: usize = <Wb67 as Nat>::VAL;
const _: () = assert!(W67 == 58);
pub fn one_ok_67()
where
    N67: HasOne,
{
}
pub fn int_like_67()
where
    Fb67: IsZero,
{
}
pub type Ib68 = Pz<I<O<I<O<O<O<H>>>>>>>;
pub type Fb68 = Z;
pub type Wb68 = Pz<I<O<I<O<O<O<H>>>>>>>;
pub type N68 = UFixed<Ib68, Fb68, Cold>;
pub const W68: usize = <Wb68 as Nat>::VAL;
const _: () = assert!(W68 == 69);
pub fn one_ok_68()
where
    N68: HasOne,
{
}
pub fn int_like_68()
where
    Fb68: IsZero,
{
}
pub type Ib69 = Pz<O<I<I<O<O<O<H>>>>>>>;
pub type Fb69 = Z;
pub type Wb69 = Pz<O<I<I<O<O<O<H>>>>>>>;
pub type N69 = UFixed<Ib69, Fb69, Hot>;
pub const W69: usize = <Wb69 as Nat>::VAL;
const _: () = assert!(W69 == 70);
pub fn one_ok_69()
where
    N69: HasOne,
{
}
pub fn int_like_69()
where
    Fb69: IsZero,
{
}
pub type Ib70 = Pz<O<I<O<I<O<I<H>>>>>>>;
pub type Fb70 = Z;
pub type Wb70 = Pz<O<I<O<I<O<I<H>>>>>>>;
pub type N70 = UFixed<Ib70, Fb70, Warm>;
pub const W70: usize = <Wb70 as Nat>::VAL;
const _: () = assert!(W70 == 106);
pub fn one_ok_70()
where
    N70: HasOne,
{
}
pub fn int_like_70()
where
    Fb70: IsZero,
{
}
pub type Ib71 = Z;
pub type Fb71 = Pz<O<I<I<O<H>>>>>;
pub type Wb71 = Pz<O<I<I<O<H>>>>>;
pub type N71 = UFixed<Ib71, Fb71, Cold>;
pub const W71: usize = <Wb71 as Nat>::VAL;
const _: () = assert!(W71 == 22);
pub fn frac_like_71()
where
    Fb71: NonZero,
{
}
pub type Ib72 = Pz<O<I<O<O<I<H>>>>>>;
pub type Fb72 = Z;
pub type Wb72 = Pz<O<I<O<O<I<H>>>>>>;
pub type N72 = UFixed<Ib72, Fb72, Hot>;
pub const W72: usize = <Wb72 as Nat>::VAL;
const _: () = assert!(W72 == 50);
pub fn one_ok_72()
where
    N72: HasOne,
{
}
pub fn int_like_72()
where
    Fb72: IsZero,
{
}
pub type Ib73 = Pz<O<O<I<O<I<H>>>>>>;
pub type Fb73 = Z;
pub type Wb73 = Pz<O<O<I<O<I<H>>>>>>;
pub type N73 = UFixed<Ib73, Fb73, Warm>;
pub const W73: usize = <Wb73 as Nat>::VAL;
const _: () = assert!(W73 == 52);
pub fn one_ok_73()
where
    N73: HasOne,
{
}
pub fn int_like_73()
where
    Fb73: IsZero,
{
}
pub type Ib74 = Pz<O<I<I<O<I<H>>>>>>;
pub type Fb74 = Z;
pub type Wb74 = Pz<O<I<I<O<I<H>>>>>>;
pub type N74 = UFixed<Ib74, Fb74, Cold>;
pub const W74: usize = <Wb74 as Nat>::VAL;
const _: () = assert!(W74 == 54);
pub fn one_ok_74()
where
    N74: HasOne,
{
}
pub fn int_like_74()
where
    Fb74: IsZero,
{
}
pub type Ib75 = Pz<O<O<I<I<I<H>>>>>>;
pub type Fb75 = Z;
pub type Wb75 = Pz<O<O<I<I<I<H>>>>>>;
pub type N75 = UFixed<Ib75, Fb75, Hot>;
pub const W75: usize = <Wb75 as Nat>::VAL;
const _: () = assert!(W75 == 60);
pub fn one_ok_75()
where
    N75: HasOne,
{
}
pub fn int_like_75()
where
    Fb75: IsZero,
{
}
pub type Ib76 = Pz<I<O<O<O<O<O<H>>>>>>>;
pub type Fb76 = Z;
pub type Wb76 = Pz<I<O<O<O<O<O<H>>>>>>>;
pub type N76 = UFixed<Ib76, Fb76, Warm>;
pub const W76: usize = <Wb76 as Nat>::VAL;
const _: () = assert!(W76 == 65);
pub fn one_ok_76()
where
    N76: HasOne,
{
}
pub fn int_like_76()
where
    Fb76: IsZero,
{
}
pub type Ib77 = Pz<O<O<I<I<O<O<H>>>>>>>;
pub type Fb77 = Z;
pub type Wb77 = Pz<O<O<I<I<O<O<H>>>>>>>;
pub type N77 = UFixed<Ib77, Fb77, Cold>;
pub const W77: usize = <Wb77 as Nat>::VAL;
const _: () = assert!(W77 == 76);
pub fn one_ok_77()
where
    N77: HasOne,
{
}
pub fn int_like_77()
where
    Fb77: IsZero,
{
}
pub type Ib78 = Pz<I<O<I<I<O<O<H>>>>>>>;
pub type Fb78 = Z;
pub type Wb78 = Pz<I<O<I<I<O<O<H>>>>>>>;
pub type N78 = UFixed<Ib78, Fb78, Hot>;
pub const W78: usize = <Wb78 as Nat>::VAL;
const _: () = assert!(W78 == 77);
pub fn one_ok_78()
where
    N78: HasOne,
{
}
pub fn int_like_78()
where
    Fb78: IsZero,
{
}
pub type Ib79 = Pz<I<O<O<O<I<I<H>>>>>>>;
pub type Fb79 = Z;
pub type Wb79 = Pz<I<O<O<O<I<I<H>>>>>>>;
pub type N79 = UFixed<Ib79, Fb79, Warm>;
pub const W79: usize = <Wb79 as Nat>::VAL;
const _: () = assert!(W79 == 113);
pub fn one_ok_79()
where
    N79: HasOne,
{
}
pub fn int_like_79()
where
    Fb79: IsZero,
{
}
pub type Ib80 = Z;
pub type Fb80 = Pz<I<I<I<O<H>>>>>;
pub type Wb80 = Pz<I<I<I<O<H>>>>>;
pub type N80 = UFixed<Ib80, Fb80, Cold>;
pub const W80: usize = <Wb80 as Nat>::VAL;
const _: () = assert!(W80 == 23);
pub fn frac_like_80()
where
    Fb80: NonZero,
{
}
pub type Ib81 = Pz<I<O<O<I<I<H>>>>>>;
pub type Fb81 = Z;
pub type Wb81 = Pz<I<O<O<I<I<H>>>>>>;
pub type N81 = UFixed<Ib81, Fb81, Hot>;
pub const W81: usize = <Wb81 as Nat>::VAL;
const _: () = assert!(W81 == 57);
pub fn one_ok_81()
where
    N81: HasOne,
{
}
pub fn int_like_81()
where
    Fb81: IsZero,
{
}
pub type Ib82 = Pz<I<I<O<I<I<H>>>>>>;
pub type Fb82 = Z;
pub type Wb82 = Pz<I<I<O<I<I<H>>>>>>;
pub type N82 = UFixed<Ib82, Fb82, Warm>;
pub const W82: usize = <Wb82 as Nat>::VAL;
const _: () = assert!(W82 == 59);
pub fn one_ok_82()
where
    N82: HasOne,
{
}
pub fn int_like_82()
where
    Fb82: IsZero,
{
}
pub type Ib83 = Pz<I<O<I<I<I<H>>>>>>;
pub type Fb83 = Z;
pub type Wb83 = Pz<I<O<I<I<I<H>>>>>>;
pub type N83 = UFixed<Ib83, Fb83, Cold>;
pub const W83: usize = <Wb83 as Nat>::VAL;
const _: () = assert!(W83 == 61);
pub fn one_ok_83()
where
    N83: HasOne,
{
}
pub fn int_like_83()
where
    Fb83: IsZero,
{
}
pub type Ib84 = Pz<I<I<O<O<O<O<H>>>>>>>;
pub type Fb84 = Z;
pub type Wb84 = Pz<I<I<O<O<O<O<H>>>>>>>;
pub type N84 = UFixed<Ib84, Fb84, Hot>;
pub const W84: usize = <Wb84 as Nat>::VAL;
const _: () = assert!(W84 == 67);
pub fn one_ok_84()
where
    N84: HasOne,
{
}
pub fn int_like_84()
where
    Fb84: IsZero,
{
}
pub type Ib85 = Pz<O<O<O<I<O<O<H>>>>>>>;
pub type Fb85 = Z;
pub type Wb85 = Pz<O<O<O<I<O<O<H>>>>>>>;
pub type N85 = UFixed<Ib85, Fb85, Warm>;
pub const W85: usize = <Wb85 as Nat>::VAL;
const _: () = assert!(W85 == 72);
pub fn one_ok_85()
where
    N85: HasOne,
{
}
pub fn int_like_85()
where
    Fb85: IsZero,
{
}
pub type Ib86 = Pz<I<I<O<O<I<O<H>>>>>>>;
pub type Fb86 = Z;
pub type Wb86 = Pz<I<I<O<O<I<O<H>>>>>>>;
pub type N86 = UFixed<Ib86, Fb86, Cold>;
pub const W86: usize = <Wb86 as Nat>::VAL;
const _: () = assert!(W86 == 83);
pub fn one_ok_86()
where
    N86: HasOne,
{
}
pub fn int_like_86()
where
    Fb86: IsZero,
{
}
pub type Ib87 = Pz<O<O<I<O<I<O<H>>>>>>>;
pub type Fb87 = Z;
pub type Wb87 = Pz<O<O<I<O<I<O<H>>>>>>>;
pub type N87 = UFixed<Ib87, Fb87, Hot>;
pub const W87: usize = <Wb87 as Nat>::VAL;
const _: () = assert!(W87 == 84);
pub fn one_ok_87()
where
    N87: HasOne,
{
}
pub fn int_like_87()
where
    Fb87: IsZero,
{
}
pub type Ib88 = Pz<O<O<O<I<I<I<H>>>>>>>;
pub type Fb88 = Z;
pub type Wb88 = Pz<O<O<O<I<I<I<H>>>>>>>;
pub type N88 = UFixed<Ib88, Fb88, Warm>;
pub const W88: usize = <Wb88 as Nat>::VAL;
const _: () = assert!(W88 == 120);
pub fn one_ok_88()
where
    N88: HasOne,
{
}
pub fn int_like_88()
where
    Fb88: IsZero,
{
}
pub type Ib89 = Z;
pub type Fb89 = Pz<O<O<O<I<H>>>>>;
pub type Wb89 = Pz<O<O<O<I<H>>>>>;
pub type N89 = UFixed<Ib89, Fb89, Cold>;
pub const W89: usize = <Wb89 as Nat>::VAL;
const _: () = assert!(W89 == 24);
pub fn frac_like_89()
where
    Fb89: NonZero,
{
}
pub type Ib90 = Pz<O<I<O<O<O<O<H>>>>>>>;
pub type Fb90 = Z;
pub type Wb90 = Pz<O<I<O<O<O<O<H>>>>>>>;
pub type N90 = UFixed<Ib90, Fb90, Hot>;
pub const W90: usize = <Wb90 as Nat>::VAL;
const _: () = assert!(W90 == 66);
pub fn one_ok_90()
where
    N90: HasOne,
{
}
pub fn int_like_90()
where
    Fb90: IsZero,
{
}
pub type Ib91 = Pz<O<O<I<O<O<O<H>>>>>>>;
pub type Fb91 = Z;
pub type Wb91 = Pz<O<O<I<O<O<O<H>>>>>>>;
pub type N91 = UFixed<Ib91, Fb91, Warm>;
pub const W91: usize = <Wb91 as Nat>::VAL;
const _: () = assert!(W91 == 68);
pub fn one_ok_91()
where
    N91: HasOne,
{
}
pub fn int_like_91()
where
    Fb91: IsZero,
{
}
pub type Ib92 = Pz<O<I<O<I<O<O<H>>>>>>>;
pub type Fb92 = Z;
pub type Wb92 = Pz<O<I<O<I<O<O<H>>>>>>>;
pub type N92 = UFixed<Ib92, Fb92, Cold>;
pub const W92: usize = <Wb92 as Nat>::VAL;
const _: () = assert!(W92 == 74);
pub fn one_ok_92()
where
    N92: HasOne,
{
}
pub fn int_like_92()
where
    Fb92: IsZero,
{
}
pub type Ib93 = Pz<I<I<I<I<O<O<H>>>>>>>;
pub type Fb93 = Z;
pub type Wb93 = Pz<I<I<I<I<O<O<H>>>>>>>;
pub type N93 = UFixed<Ib93, Fb93, Hot>;
pub const W93: usize = <Wb93 as Nat>::VAL;
const _: () = assert!(W93 == 79);
pub fn one_ok_93()
where
    N93: HasOne,
{
}
pub fn int_like_93()
where
    Fb93: IsZero,
{
}
pub type Ib94 = Pz<O<I<O<I<I<O<H>>>>>>>;
pub type Fb94 = Z;
pub type Wb94 = Pz<O<I<O<I<I<O<H>>>>>>>;
pub type N94 = UFixed<Ib94, Fb94, Warm>;
pub const W94: usize = <Wb94 as Nat>::VAL;
const _: () = assert!(W94 == 90);
pub fn one_ok_94()
where
    N94: HasOne,
{
}
pub fn int_like_94()
where
    Fb94: IsZero,
{
}
pub type Ib95 = Pz<I<I<O<I<I<O<H>>>>>>>;
pub type Fb95 = Z;
pub type Wb95 = Pz<I<I<O<I<I<O<H>>>>>>>;
pub type N95 = UFixed<Ib95, Fb95, Cold>;
pub const W95: usize = <Wb95 as Nat>::VAL;
const _: () = assert!(W95 == 91);
pub fn one_ok_95()
where
    N95: HasOne,
{
}
pub fn int_like_95()
where
    Fb95: IsZero,
{
}
pub type Ib96 = Pz<I<I<I<I<I<I<H>>>>>>>;
pub type Fb96 = Z;
pub type Wb96 = Pz<I<I<I<I<I<I<H>>>>>>>;
pub type N96 = UFixed<Ib96, Fb96, Hot>;
pub const W96: usize = <Wb96 as Nat>::VAL;
const _: () = assert!(W96 == 127);
pub fn one_ok_96()
where
    N96: HasOne,
{
}
pub fn int_like_96()
where
    Fb96: IsZero,
{
}
pub type Ib97 = Z;
pub type Fb97 = Pz<I<O<O<I<H>>>>>;
pub type Wb97 = Pz<I<O<O<I<H>>>>>;
pub type N97 = UFixed<Ib97, Fb97, Warm>;
pub const W97: usize = <Wb97 as Nat>::VAL;
const _: () = assert!(W97 == 25);
pub fn frac_like_97()
where
    Fb97: NonZero,
{
}
pub type Ib98 = Pz<I<O<O<I<O<O<H>>>>>>>;
pub type Fb98 = Z;
pub type Wb98 = Pz<I<O<O<I<O<O<H>>>>>>>;
pub type N98 = UFixed<Ib98, Fb98, Cold>;
pub const W98: usize = <Wb98 as Nat>::VAL;
const _: () = assert!(W98 == 73);
pub fn one_ok_98()
where
    N98: HasOne,
{
}
pub fn int_like_98()
where
    Fb98: IsZero,
{
}
pub type Ib99 = Pz<I<I<O<I<O<O<H>>>>>>>;
pub type Fb99 = Z;
pub type Wb99 = Pz<I<I<O<I<O<O<H>>>>>>>;
pub type N99 = UFixed<Ib99, Fb99, Hot>;
pub const W99: usize = <Wb99 as Nat>::VAL;
const _: () = assert!(W99 == 75);
pub fn one_ok_99()
where
    N99: HasOne,
{
}
pub fn int_like_99()
where
    Fb99: IsZero,
{
}
pub type Sum0 = <N0 as AddNum<N3>>::Out;
pub const SW0: usize = <Sum0 as Stored>::W;
pub type Sum1 = <N1 as AddNum<N4>>::Out;
pub const SW1: usize = <Sum1 as Stored>::W;
pub type Sum2 = <N2 as AddNum<N5>>::Out;
pub const SW2: usize = <Sum2 as Stored>::W;
pub type Sum3 = <N3 as AddNum<N6>>::Out;
pub const SW3: usize = <Sum3 as Stored>::W;
pub type Sum4 = <N4 as AddNum<N7>>::Out;
pub const SW4: usize = <Sum4 as Stored>::W;
pub type Sum5 = <N5 as AddNum<N8>>::Out;
pub const SW5: usize = <Sum5 as Stored>::W;
pub type Sum6 = <N6 as AddNum<N9>>::Out;
pub const SW6: usize = <Sum6 as Stored>::W;
pub type Sum7 = <N7 as AddNum<N10>>::Out;
pub const SW7: usize = <Sum7 as Stored>::W;
pub type Sum8 = <N8 as AddNum<N11>>::Out;
pub const SW8: usize = <Sum8 as Stored>::W;
pub type Sum9 = <N9 as AddNum<N12>>::Out;
pub const SW9: usize = <Sum9 as Stored>::W;
pub type Sum10 = <N10 as AddNum<N13>>::Out;
pub const SW10: usize = <Sum10 as Stored>::W;
pub type Sum11 = <N11 as AddNum<N14>>::Out;
pub const SW11: usize = <Sum11 as Stored>::W;
pub type Sum12 = <N12 as AddNum<N15>>::Out;
pub const SW12: usize = <Sum12 as Stored>::W;
pub type Sum13 = <N13 as AddNum<N16>>::Out;
pub const SW13: usize = <Sum13 as Stored>::W;
pub type Sum14 = <N14 as AddNum<N17>>::Out;
pub const SW14: usize = <Sum14 as Stored>::W;
pub type Sum15 = <N15 as AddNum<N18>>::Out;
pub const SW15: usize = <Sum15 as Stored>::W;
pub type Sum16 = <N16 as AddNum<N19>>::Out;
pub const SW16: usize = <Sum16 as Stored>::W;
pub type Sum17 = <N17 as AddNum<N20>>::Out;
pub const SW17: usize = <Sum17 as Stored>::W;
pub type Sum18 = <N18 as AddNum<N21>>::Out;
pub const SW18: usize = <Sum18 as Stored>::W;
pub type Sum19 = <N19 as AddNum<N22>>::Out;
pub const SW19: usize = <Sum19 as Stored>::W;
pub type Sum20 = <N20 as AddNum<N23>>::Out;
pub const SW20: usize = <Sum20 as Stored>::W;
pub type Sum21 = <N21 as AddNum<N24>>::Out;
pub const SW21: usize = <Sum21 as Stored>::W;
pub type Sum22 = <N22 as AddNum<N25>>::Out;
pub const SW22: usize = <Sum22 as Stored>::W;
pub type Sum23 = <N23 as AddNum<N26>>::Out;
pub const SW23: usize = <Sum23 as Stored>::W;
pub type Sum24 = <N24 as AddNum<N27>>::Out;
pub const SW24: usize = <Sum24 as Stored>::W;
pub type Sum25 = <N25 as AddNum<N28>>::Out;
pub const SW25: usize = <Sum25 as Stored>::W;
pub type Sum26 = <N26 as AddNum<N29>>::Out;
pub const SW26: usize = <Sum26 as Stored>::W;
pub type Sum27 = <N27 as AddNum<N30>>::Out;
pub const SW27: usize = <Sum27 as Stored>::W;
pub type Sum28 = <N28 as AddNum<N31>>::Out;
pub const SW28: usize = <Sum28 as Stored>::W;
pub type Sum29 = <N29 as AddNum<N32>>::Out;
pub const SW29: usize = <Sum29 as Stored>::W;
pub type Sum30 = <N30 as AddNum<N33>>::Out;
pub const SW30: usize = <Sum30 as Stored>::W;
pub type Sum31 = <N31 as AddNum<N34>>::Out;
pub const SW31: usize = <Sum31 as Stored>::W;
pub type Sum32 = <N32 as AddNum<N35>>::Out;
pub const SW32: usize = <Sum32 as Stored>::W;
pub type Sum33 = <N33 as AddNum<N36>>::Out;
pub const SW33: usize = <Sum33 as Stored>::W;
pub type Sum34 = <N34 as AddNum<N37>>::Out;
pub const SW34: usize = <Sum34 as Stored>::W;
pub type Sum35 = <N35 as AddNum<N38>>::Out;
pub const SW35: usize = <Sum35 as Stored>::W;
pub type Sum36 = <N36 as AddNum<N39>>::Out;
pub const SW36: usize = <Sum36 as Stored>::W;
pub type Sum37 = <N37 as AddNum<N40>>::Out;
pub const SW37: usize = <Sum37 as Stored>::W;
pub type Sum38 = <N38 as AddNum<N41>>::Out;
pub const SW38: usize = <Sum38 as Stored>::W;
pub type Sum39 = <N39 as AddNum<N42>>::Out;
pub const SW39: usize = <Sum39 as Stored>::W;
pub type Sum40 = <N40 as AddNum<N43>>::Out;
pub const SW40: usize = <Sum40 as Stored>::W;
pub type Sum41 = <N41 as AddNum<N44>>::Out;
pub const SW41: usize = <Sum41 as Stored>::W;
pub type Sum42 = <N42 as AddNum<N45>>::Out;
pub const SW42: usize = <Sum42 as Stored>::W;
pub type Sum43 = <N43 as AddNum<N46>>::Out;
pub const SW43: usize = <Sum43 as Stored>::W;
pub type Sum44 = <N44 as AddNum<N47>>::Out;
pub const SW44: usize = <Sum44 as Stored>::W;
pub type Sum45 = <N45 as AddNum<N48>>::Out;
pub const SW45: usize = <Sum45 as Stored>::W;
pub type Sum46 = <N46 as AddNum<N49>>::Out;
pub const SW46: usize = <Sum46 as Stored>::W;
pub type Sum47 = <N47 as AddNum<N50>>::Out;
pub const SW47: usize = <Sum47 as Stored>::W;
pub type Sum48 = <N48 as AddNum<N51>>::Out;
pub const SW48: usize = <Sum48 as Stored>::W;
pub type Sum49 = <N49 as AddNum<N52>>::Out;
pub const SW49: usize = <Sum49 as Stored>::W;
pub type Sum50 = <N50 as AddNum<N53>>::Out;
pub const SW50: usize = <Sum50 as Stored>::W;
pub type Sum51 = <N51 as AddNum<N54>>::Out;
pub const SW51: usize = <Sum51 as Stored>::W;
pub type Sum52 = <N52 as AddNum<N55>>::Out;
pub const SW52: usize = <Sum52 as Stored>::W;
pub type Sum53 = <N53 as AddNum<N56>>::Out;
pub const SW53: usize = <Sum53 as Stored>::W;
pub type Sum54 = <N54 as AddNum<N57>>::Out;
pub const SW54: usize = <Sum54 as Stored>::W;
pub type Sum55 = <N55 as AddNum<N58>>::Out;
pub const SW55: usize = <Sum55 as Stored>::W;
pub type Sum56 = <N56 as AddNum<N59>>::Out;
pub const SW56: usize = <Sum56 as Stored>::W;
pub type Sum57 = <N57 as AddNum<N60>>::Out;
pub const SW57: usize = <Sum57 as Stored>::W;
pub type Sum58 = <N58 as AddNum<N61>>::Out;
pub const SW58: usize = <Sum58 as Stored>::W;
pub type Sum59 = <N59 as AddNum<N62>>::Out;
pub const SW59: usize = <Sum59 as Stored>::W;
pub type Sum60 = <N60 as AddNum<N63>>::Out;
pub const SW60: usize = <Sum60 as Stored>::W;
pub type Sum61 = <N61 as AddNum<N64>>::Out;
pub const SW61: usize = <Sum61 as Stored>::W;
pub type Sum62 = <N62 as AddNum<N65>>::Out;
pub const SW62: usize = <Sum62 as Stored>::W;
pub type Sum63 = <N63 as AddNum<N66>>::Out;
pub const SW63: usize = <Sum63 as Stored>::W;
pub type Sum64 = <N64 as AddNum<N67>>::Out;
pub const SW64: usize = <Sum64 as Stored>::W;
pub type Sum65 = <N65 as AddNum<N68>>::Out;
pub const SW65: usize = <Sum65 as Stored>::W;
pub type Sum66 = <N66 as AddNum<N69>>::Out;
pub const SW66: usize = <Sum66 as Stored>::W;
pub type Sum67 = <N67 as AddNum<N70>>::Out;
pub const SW67: usize = <Sum67 as Stored>::W;
pub type Sum68 = <N68 as AddNum<N71>>::Out;
pub const SW68: usize = <Sum68 as Stored>::W;
pub type Sum69 = <N69 as AddNum<N72>>::Out;
pub const SW69: usize = <Sum69 as Stored>::W;
pub type Sum70 = <N70 as AddNum<N73>>::Out;
pub const SW70: usize = <Sum70 as Stored>::W;
pub type Sum71 = <N71 as AddNum<N74>>::Out;
pub const SW71: usize = <Sum71 as Stored>::W;
pub type Sum72 = <N72 as AddNum<N75>>::Out;
pub const SW72: usize = <Sum72 as Stored>::W;
pub type Sum73 = <N73 as AddNum<N76>>::Out;
pub const SW73: usize = <Sum73 as Stored>::W;
pub type Sum74 = <N74 as AddNum<N77>>::Out;
pub const SW74: usize = <Sum74 as Stored>::W;
pub type Sum75 = <N75 as AddNum<N78>>::Out;
pub const SW75: usize = <Sum75 as Stored>::W;
pub type Sum76 = <N76 as AddNum<N79>>::Out;
pub const SW76: usize = <Sum76 as Stored>::W;
pub type Sum77 = <N77 as AddNum<N80>>::Out;
pub const SW77: usize = <Sum77 as Stored>::W;
pub type Sum78 = <N78 as AddNum<N81>>::Out;
pub const SW78: usize = <Sum78 as Stored>::W;
pub type Sum79 = <N79 as AddNum<N82>>::Out;
pub const SW79: usize = <Sum79 as Stored>::W;
pub type Sum80 = <N80 as AddNum<N83>>::Out;
pub const SW80: usize = <Sum80 as Stored>::W;
pub type Sum81 = <N81 as AddNum<N84>>::Out;
pub const SW81: usize = <Sum81 as Stored>::W;
pub type Sum82 = <N82 as AddNum<N85>>::Out;
pub const SW82: usize = <Sum82 as Stored>::W;
pub type Sum83 = <N83 as AddNum<N86>>::Out;
pub const SW83: usize = <Sum83 as Stored>::W;
pub type Sum84 = <N84 as AddNum<N87>>::Out;
pub const SW84: usize = <Sum84 as Stored>::W;
pub type Sum85 = <N85 as AddNum<N88>>::Out;
pub const SW85: usize = <Sum85 as Stored>::W;
pub type Sum86 = <N86 as AddNum<N89>>::Out;
pub const SW86: usize = <Sum86 as Stored>::W;
pub type Sum87 = <N87 as AddNum<N90>>::Out;
pub const SW87: usize = <Sum87 as Stored>::W;
pub type Sum88 = <N88 as AddNum<N91>>::Out;
pub const SW88: usize = <Sum88 as Stored>::W;
pub type Sum89 = <N89 as AddNum<N92>>::Out;
pub const SW89: usize = <Sum89 as Stored>::W;
pub type Sum90 = <N90 as AddNum<N93>>::Out;
pub const SW90: usize = <Sum90 as Stored>::W;
pub type Sum91 = <N91 as AddNum<N94>>::Out;
pub const SW91: usize = <Sum91 as Stored>::W;
pub type Sum92 = <N92 as AddNum<N95>>::Out;
pub const SW92: usize = <Sum92 as Stored>::W;
pub type Sum93 = <N93 as AddNum<N96>>::Out;
pub const SW93: usize = <Sum93 as Stored>::W;
pub type Sum94 = <N94 as AddNum<N97>>::Out;
pub const SW94: usize = <Sum94 as Stored>::W;
pub type Sum95 = <N95 as AddNum<N98>>::Out;
pub const SW95: usize = <Sum95 as Stored>::W;
pub type Sum96 = <N96 as AddNum<N99>>::Out;
pub const SW96: usize = <Sum96 as Stored>::W;
pub type C0 = Slot<Pz<H>, 1>;
pub fn build0() -> <C0 as Capacity>::Array<u32> {
    C0::build(0)
}
pub type C1 = Slot<Pz<I<H>>, 3>;
pub fn build1() -> <C1 as Capacity>::Array<u32> {
    C1::build(0)
}
pub type C2 = Slot<Pz<O<O<H>>>, 4>;
pub fn build2() -> <C2 as Capacity>::Array<u32> {
    C2::build(0)
}
pub type C3 = Slot<Pz<I<I<H>>>, 7>;
pub fn build3() -> <C3 as Capacity>::Array<u32> {
    C3::build(0)
}
pub type C4 = Slot<Pz<O<O<O<H>>>>, 8>;
pub fn build4() -> <C4 as Capacity>::Array<u32> {
    C4::build(0)
}
pub type C5 = Slot<Pz<I<O<I<H>>>>, 13>;
pub fn build5() -> <C5 as Capacity>::Array<u32> {
    C5::build(0)
}
pub type C6 = Slot<Pz<O<O<O<O<H>>>>>, 16>;
pub fn build6() -> <C6 as Capacity>::Array<u32> {
    C6::build(0)
}
pub type C7 = Slot<Pz<O<O<I<I<H>>>>>, 28>;
pub fn build7() -> <C7 as Capacity>::Array<u32> {
    C7::build(0)
}
pub type C8 = Slot<Pz<O<O<O<O<O<H>>>>>>, 32>;
pub fn build8() -> <C8 as Capacity>::Array<u32> {
    C8::build(0)
}
pub type C9 = Slot<Pz<O<O<O<O<O<O<H>>>>>>>, 64>;
pub fn build9() -> <C9 as Capacity>::Array<u32> {
    C9::build(0)
}
pub type C10 = Slot<Pz<O<O<I<H>>>>, 12>;
pub fn build10() -> <C10 as Capacity>::Array<u32> {
    C10::build(0)
}
pub type C11 = Slot<Pz<O<I<I<H>>>>, 14>;
pub fn build11() -> <C11 as Capacity>::Array<u32> {
    C11::build(0)
}
pub type C12 = Slot<Pz<I<I<I<H>>>>, 15>;
pub fn build12() -> <C12 as Capacity>::Array<u32> {
    C12::build(0)
}
pub type C13 = Slot<Pz<O<I<O<O<H>>>>>, 18>;
pub fn build13() -> <C13 as Capacity>::Array<u32> {
    C13::build(0)
}
pub type C14 = Slot<Pz<I<I<O<O<H>>>>>, 19>;
pub fn build14() -> <C14 as Capacity>::Array<u32> {
    C14::build(0)
}
pub type C15 = Slot<Pz<O<O<O<I<H>>>>>, 24>;
pub fn build15() -> <C15 as Capacity>::Array<u32> {
    C15::build(0)
}
pub type C16 = Slot<Pz<I<I<O<I<H>>>>>, 27>;
pub fn build16() -> <C16 as Capacity>::Array<u32> {
    C16::build(0)
}
pub type C17 = Slot<Pz<I<I<I<O<O<H>>>>>>, 39>;
pub fn build17() -> <C17 as Capacity>::Array<u32> {
    C17::build(0)
}
pub type C18 = Slot<Pz<I<I<O<I<O<H>>>>>>, 43>;
pub fn build18() -> <C18 as Capacity>::Array<u32> {
    C18::build(0)
}
pub type C19 = Slot<Pz<I<I<O<I<O<O<H>>>>>>>, 75>;
pub fn build19() -> <C19 as Capacity>::Array<u32> {
    C19::build(0)
}
pub type C20 = Slot<Pz<I<I<I<O<H>>>>>, 23>;
pub fn build20() -> <C20 as Capacity>::Array<u32> {
    C20::build(0)
}
pub type C21 = Slot<Pz<I<O<O<I<H>>>>>, 25>;
pub fn build21() -> <C21 as Capacity>::Array<u32> {
    C21::build(0)
}
pub type C22 = Slot<Pz<O<I<O<I<H>>>>>, 26>;
pub fn build22() -> <C22 as Capacity>::Array<u32> {
    C22::build(0)
}
pub type C23 = Slot<Pz<I<O<I<I<H>>>>>, 29>;
pub fn build23() -> <C23 as Capacity>::Array<u32> {
    C23::build(0)
}
pub type C24 = Slot<Pz<O<I<I<I<H>>>>>, 30>;
pub fn build24() -> <C24 as Capacity>::Array<u32> {
    C24::build(0)
}
pub type C25 = Slot<Pz<I<I<O<O<O<H>>>>>>, 35>;
pub fn build25() -> <C25 as Capacity>::Array<u32> {
    C25::build(0)
}
pub type C26 = Slot<Pz<O<I<I<O<O<H>>>>>>, 38>;
pub fn build26() -> <C26 as Capacity>::Array<u32> {
    C26::build(0)
}
pub type C27 = Slot<Pz<O<I<O<O<I<H>>>>>>, 50>;
pub fn build27() -> <C27 as Capacity>::Array<u32> {
    C27::build(0)
}
pub type C28 = Slot<Pz<O<I<I<O<I<H>>>>>>, 54>;
pub fn build28() -> <C28 as Capacity>::Array<u32> {
    C28::build(0)
}
pub type C29 = Slot<Pz<O<I<I<O<I<O<H>>>>>>>, 86>;
pub fn build29() -> <C29 as Capacity>::Array<u32> {
    C29::build(0)
}
pub type C30 = Slot<Pz<O<I<O<O<O<H>>>>>>, 34>;
pub fn build30() -> <C30 as Capacity>::Array<u32> {
    C30::build(0)
}
pub type C31 = Slot<Pz<O<O<I<O<O<H>>>>>>, 36>;
pub fn build31() -> <C31 as Capacity>::Array<u32> {
    C31::build(0)
}
pub type C32 = Slot<Pz<I<O<I<O<O<H>>>>>>, 37>;
pub fn build32() -> <C32 as Capacity>::Array<u32> {
    C32::build(0)
}
pub type C33 = Slot<Pz<O<O<O<I<O<H>>>>>>, 40>;
pub fn build33() -> <C33 as Capacity>::Array<u32> {
    C33::build(0)
}
pub type C34 = Slot<Pz<I<O<O<I<O<H>>>>>>, 41>;
pub fn build34() -> <C34 as Capacity>::Array<u32> {
    C34::build(0)
}
pub type C35 = Slot<Pz<O<I<I<I<O<H>>>>>>, 46>;
pub fn build35() -> <C35 as Capacity>::Array<u32> {
    C35::build(0)
}
pub type C36 = Slot<Pz<I<O<O<O<I<H>>>>>>, 49>;
pub fn build36() -> <C36 as Capacity>::Array<u32> {
    C36::build(0)
}
pub type C37 = Slot<Pz<I<O<I<I<I<H>>>>>>, 61>;
pub fn build37() -> <C37 as Capacity>::Array<u32> {
    C37::build(0)
}
pub type C38 = Slot<Pz<I<O<O<O<O<O<H>>>>>>>, 65>;
pub fn build38() -> <C38 as Capacity>::Array<u32> {
    C38::build(0)
}
pub type C39 = Slot<Pz<I<O<O<O<O<I<H>>>>>>>, 97>;
pub fn build39() -> <C39 as Capacity>::Array<u32> {
    C39::build(0)
}
pub type C40 = Slot<Pz<I<O<I<I<O<H>>>>>>, 45>;
pub fn build40() -> <C40 as Capacity>::Array<u32> {
    C40::build(0)
}
pub type C41 = Slot<Pz<I<I<I<I<O<H>>>>>>, 47>;
pub fn build41() -> <C41 as Capacity>::Array<u32> {
    C41::build(0)
}
pub type C42 = Slot<Pz<O<O<O<O<I<H>>>>>>, 48>;
pub fn build42() -> <C42 as Capacity>::Array<u32> {
    C42::build(0)
}
pub type C43 = Slot<Pz<I<I<O<O<I<H>>>>>>, 51>;
pub fn build43() -> <C43 as Capacity>::Array<u32> {
    C43::build(0)
}
pub type C44 = Slot<Pz<O<O<I<O<I<H>>>>>>, 52>;
pub fn build44() -> <C44 as Capacity>::Array<u32> {
    C44::build(0)
}
pub type C45 = Slot<Pz<I<O<O<I<I<H>>>>>>, 57>;
pub fn build45() -> <C45 as Capacity>::Array<u32> {
    C45::build(0)
}
pub type C46 = Slot<Pz<O<O<I<I<I<H>>>>>>, 60>;
pub fn build46() -> <C46 as Capacity>::Array<u32> {
    C46::build(0)
}
pub type C47 = Slot<Pz<O<O<O<I<O<O<H>>>>>>>, 72>;
pub fn build47() -> <C47 as Capacity>::Array<u32> {
    C47::build(0)
}
pub type C48 = Slot<Pz<O<O<I<I<O<O<H>>>>>>>, 76>;
pub fn build48() -> <C48 as Capacity>::Array<u32> {
    C48::build(0)
}
pub type C49 = Slot<Pz<O<O<I<I<O<I<H>>>>>>>, 108>;
pub fn build49() -> <C49 as Capacity>::Array<u32> {
    C49::build(0)
}
pub type C50 = Slot<Pz<O<O<O<I<I<H>>>>>>, 56>;
pub fn build50() -> <C50 as Capacity>::Array<u32> {
    C50::build(0)
}
pub type C51 = Slot<Pz<O<I<O<I<I<H>>>>>>, 58>;
pub fn build51() -> <C51 as Capacity>::Array<u32> {
    C51::build(0)
}
pub type C52 = Slot<Pz<I<I<O<I<I<H>>>>>>, 59>;
pub fn build52() -> <C52 as Capacity>::Array<u32> {
    C52::build(0)
}
pub type C53 = Slot<Pz<O<I<I<I<I<H>>>>>>, 62>;
pub fn build53() -> <C53 as Capacity>::Array<u32> {
    C53::build(0)
}
pub type C54 = Slot<Pz<I<I<I<I<I<H>>>>>>, 63>;
pub fn build54() -> <C54 as Capacity>::Array<u32> {
    C54::build(0)
}
pub type C55 = Slot<Pz<O<O<I<O<O<O<H>>>>>>>, 68>;
pub fn build55() -> <C55 as Capacity>::Array<u32> {
    C55::build(0)
}
pub type C56 = Slot<Pz<I<I<I<O<O<O<H>>>>>>>, 71>;
pub fn build56() -> <C56 as Capacity>::Array<u32> {
    C56::build(0)
}
pub type C57 = Slot<Pz<I<I<O<O<I<O<H>>>>>>>, 83>;
pub fn build57() -> <C57 as Capacity>::Array<u32> {
    C57::build(0)
}
pub type C58 = Slot<Pz<I<I<I<O<I<O<H>>>>>>>, 87>;
pub fn build58() -> <C58 as Capacity>::Array<u32> {
    C58::build(0)
}
pub type C59 = Slot<Pz<I<I<I<O<I<I<H>>>>>>>, 119>;
pub fn build59() -> <C59 as Capacity>::Array<u32> {
    C59::build(0)
}
pub type C60 = Slot<Pz<I<I<O<O<O<O<H>>>>>>>, 67>;
pub fn build60() -> <C60 as Capacity>::Array<u32> {
    C60::build(0)
}
pub type C61 = Slot<Pz<I<O<I<O<O<O<H>>>>>>>, 69>;
pub fn build61() -> <C61 as Capacity>::Array<u32> {
    C61::build(0)
}
pub type C62 = Slot<Pz<O<I<I<O<O<O<H>>>>>>>, 70>;
pub fn build62() -> <C62 as Capacity>::Array<u32> {
    C62::build(0)
}
pub type C63 = Slot<Pz<I<O<O<I<O<O<H>>>>>>>, 73>;
pub fn build63() -> <C63 as Capacity>::Array<u32> {
    C63::build(0)
}
pub type C64 = Slot<Pz<O<I<O<I<O<O<H>>>>>>>, 74>;
pub fn build64() -> <C64 as Capacity>::Array<u32> {
    C64::build(0)
}
pub type C65 = Slot<Pz<I<I<I<I<O<O<H>>>>>>>, 79>;
pub fn build65() -> <C65 as Capacity>::Array<u32> {
    C65::build(0)
}
pub type C66 = Slot<Pz<O<I<O<O<I<O<H>>>>>>>, 82>;
pub fn build66() -> <C66 as Capacity>::Array<u32> {
    C66::build(0)
}
pub type C67 = Slot<Pz<O<I<I<I<I<O<H>>>>>>>, 94>;
pub fn build67() -> <C67 as Capacity>::Array<u32> {
    C67::build(0)
}
pub type C68 = Slot<Pz<O<I<O<O<O<I<H>>>>>>>, 98>;
pub fn build68() -> <C68 as Capacity>::Array<u32> {
    C68::build(0)
}
pub type C69 = Slot<Pz<O<I<O<O<O<O<O<H>>>>>>>>, 130>;
pub fn build69() -> <C69 as Capacity>::Array<u32> {
    C69::build(0)
}
pub type C70 = Slot<Pz<O<I<I<I<O<O<H>>>>>>>, 78>;
pub fn build70() -> <C70 as Capacity>::Array<u32> {
    C70::build(0)
}
pub type C71 = Slot<Pz<O<O<O<O<I<O<H>>>>>>>, 80>;
pub fn build71() -> <C71 as Capacity>::Array<u32> {
    C71::build(0)
}
pub type C72 = Slot<Pz<I<O<O<O<I<O<H>>>>>>>, 81>;
pub fn build72() -> <C72 as Capacity>::Array<u32> {
    C72::build(0)
}
pub type C73 = Slot<Pz<O<O<I<O<I<O<H>>>>>>>, 84>;
pub fn build73() -> <C73 as Capacity>::Array<u32> {
    C73::build(0)
}
pub type C74 = Slot<Pz<I<O<I<O<I<O<H>>>>>>>, 85>;
pub fn build74() -> <C74 as Capacity>::Array<u32> {
    C74::build(0)
}
pub type C75 = Slot<Pz<O<I<O<I<I<O<H>>>>>>>, 90>;
pub fn build75() -> <C75 as Capacity>::Array<u32> {
    C75::build(0)
}
pub type C76 = Slot<Pz<I<O<I<I<I<O<H>>>>>>>, 93>;
pub fn build76() -> <C76 as Capacity>::Array<u32> {
    C76::build(0)
}
pub type C77 = Slot<Pz<I<O<O<I<O<I<H>>>>>>>, 105>;
pub fn build77() -> <C77 as Capacity>::Array<u32> {
    C77::build(0)
}
pub type C78 = Slot<Pz<I<O<I<I<O<I<H>>>>>>>, 109>;
pub fn build78() -> <C78 as Capacity>::Array<u32> {
    C78::build(0)
}
pub type C79 = Slot<Pz<I<O<I<I<O<O<O<H>>>>>>>>, 141>;
pub fn build79() -> <C79 as Capacity>::Array<u32> {
    C79::build(0)
}
pub type C80 = Slot<Pz<I<O<O<I<I<O<H>>>>>>>, 89>;
pub fn build80() -> <C80 as Capacity>::Array<u32> {
    C80::build(0)
}
pub type C81 = Slot<Pz<I<I<O<I<I<O<H>>>>>>>, 91>;
pub fn build81() -> <C81 as Capacity>::Array<u32> {
    C81::build(0)
}
pub type C82 = Slot<Pz<O<O<I<I<I<O<H>>>>>>>, 92>;
pub fn build82() -> <C82 as Capacity>::Array<u32> {
    C82::build(0)
}
pub type C83 = Slot<Pz<I<I<I<I<I<O<H>>>>>>>, 95>;
pub fn build83() -> <C83 as Capacity>::Array<u32> {
    C83::build(0)
}
pub type C84 = Slot<Pz<O<O<O<O<O<I<H>>>>>>>, 96>;
pub fn build84() -> <C84 as Capacity>::Array<u32> {
    C84::build(0)
}
pub type C85 = Slot<Pz<I<O<I<O<O<I<H>>>>>>>, 101>;
pub fn build85() -> <C85 as Capacity>::Array<u32> {
    C85::build(0)
}
pub type C86 = Slot<Pz<O<O<O<I<O<I<H>>>>>>>, 104>;
pub fn build86() -> <C86 as Capacity>::Array<u32> {
    C86::build(0)
}
pub type C87 = Slot<Pz<O<O<I<O<I<I<H>>>>>>>, 116>;
pub fn build87() -> <C87 as Capacity>::Array<u32> {
    C87::build(0)
}
pub type C88 = Slot<Pz<O<O<O<I<I<I<H>>>>>>>, 120>;
pub fn build88() -> <C88 as Capacity>::Array<u32> {
    C88::build(0)
}
pub type C89 = Slot<Pz<O<O<O<I<I<O<O<H>>>>>>>>, 152>;
pub fn build89() -> <C89 as Capacity>::Array<u32> {
    C89::build(0)
}
pub type C90 = Slot<Pz<O<O<I<O<O<I<H>>>>>>>, 100>;
pub fn build90() -> <C90 as Capacity>::Array<u32> {
    C90::build(0)
}
pub type C91 = Slot<Pz<O<I<I<O<O<I<H>>>>>>>, 102>;
pub fn build91() -> <C91 as Capacity>::Array<u32> {
    C91::build(0)
}
pub type C92 = Slot<Pz<I<I<I<O<O<I<H>>>>>>>, 103>;
pub fn build92() -> <C92 as Capacity>::Array<u32> {
    C92::build(0)
}
pub type C93 = Slot<Pz<O<I<O<I<O<I<H>>>>>>>, 106>;
pub fn build93() -> <C93 as Capacity>::Array<u32> {
    C93::build(0)
}
pub type C94 = Slot<Pz<I<I<O<I<O<I<H>>>>>>>, 107>;
pub fn build94() -> <C94 as Capacity>::Array<u32> {
    C94::build(0)
}
pub type C95 = Slot<Pz<O<O<O<O<I<I<H>>>>>>>, 112>;
pub fn build95() -> <C95 as Capacity>::Array<u32> {
    C95::build(0)
}
pub type C96 = Slot<Pz<I<I<O<O<I<I<H>>>>>>>, 115>;
pub fn build96() -> <C96 as Capacity>::Array<u32> {
    C96::build(0)
}
pub type C97 = Slot<Pz<I<I<I<I<I<I<H>>>>>>>, 127>;
pub fn build97() -> <C97 as Capacity>::Array<u32> {
    C97::build(0)
}
pub type C98 = Slot<Pz<I<I<O<O<O<O<O<H>>>>>>>>, 131>;
pub fn build98() -> <C98 as Capacity>::Array<u32> {
    C98::build(0)
}
pub type C99 = Slot<Pz<I<I<O<O<O<I<O<H>>>>>>>>, 163>;
pub fn build99() -> <C99 as Capacity>::Array<u32> {
    C99::build(0)
}

// Obligation 4: build and walk, generic over the capacity.
pub fn fold_generic<C: Capacity>(seed: u32) -> u32 {
    let mut a = C::build(seed);
    let s: &mut [u32] = a.as_mut();
    let mut i = 0;
    while i < s.len() {
        s[i] = s[i].wrapping_add(i as u32);
        i += 1;
    }
    let r: &[u32] = a.as_ref();
    let mut acc = 0u32;
    let mut j = 0;
    while j < r.len() {
        acc = acc.wrapping_add(r[j]);
        j += 1;
    }
    acc
}
// Obligation 5: generic over a numeral AND a capacity at once. This is the
// site staging cannot reach, because neither width is known here.
pub fn scaled_fold<Ib, Fb, S, C>(seed: u32) -> u32
where
    Ib: Nat + AddN<Fb>,
    Fb: Nat,
    S: Strategy,
    C: Capacity,
    UFixed<Ib, Fb, S>: Stored + HasOne,
{
    fold_generic::<C>(seed).wrapping_mul(<UFixed<Ib, Fb, S> as Stored>::W as u32)
}

pub fn call0() -> u32 {
    scaled_fold::<Ib0, Fb0, Hot, C0>(0)
}
pub fn call1() -> u32 {
    scaled_fold::<Ib1, Fb1, Warm, C1>(1)
}
pub fn call2() -> u32 {
    scaled_fold::<Ib2, Fb2, Cold, C2>(2)
}
pub fn call3() -> u32 {
    scaled_fold::<Ib3, Fb3, Hot, C3>(3)
}
pub fn call4() -> u32 {
    scaled_fold::<Ib4, Fb4, Warm, C4>(4)
}
pub fn call5() -> u32 {
    scaled_fold::<Ib5, Fb5, Cold, C5>(5)
}
pub fn call6() -> u32 {
    scaled_fold::<Ib6, Fb6, Hot, C6>(6)
}
pub fn call7() -> u32 {
    scaled_fold::<Ib7, Fb7, Warm, C7>(7)
}
pub fn call8() -> u32 {
    scaled_fold::<Ib8, Fb8, Cold, C8>(8)
}
pub fn call9() -> u32 {
    scaled_fold::<Ib9, Fb9, Hot, C9>(9)
}
pub fn call10() -> u32 {
    scaled_fold::<Ib10, Fb10, Warm, C10>(10)
}
pub fn call11() -> u32 {
    scaled_fold::<Ib11, Fb11, Cold, C11>(11)
}
pub fn call12() -> u32 {
    scaled_fold::<Ib12, Fb12, Hot, C12>(12)
}
pub fn call14() -> u32 {
    scaled_fold::<Ib14, Fb14, Cold, C14>(14)
}
pub fn call15() -> u32 {
    scaled_fold::<Ib15, Fb15, Hot, C15>(15)
}
pub fn call16() -> u32 {
    scaled_fold::<Ib16, Fb16, Warm, C16>(16)
}
pub fn call17() -> u32 {
    scaled_fold::<Ib17, Fb17, Cold, C17>(17)
}
pub fn call18() -> u32 {
    scaled_fold::<Ib18, Fb18, Hot, C18>(18)
}
pub fn call19() -> u32 {
    scaled_fold::<Ib19, Fb19, Warm, C19>(19)
}
pub fn call20() -> u32 {
    scaled_fold::<Ib20, Fb20, Cold, C20>(20)
}
pub fn call21() -> u32 {
    scaled_fold::<Ib21, Fb21, Hot, C21>(21)
}
pub fn call22() -> u32 {
    scaled_fold::<Ib22, Fb22, Warm, C22>(22)
}
pub fn call23() -> u32 {
    scaled_fold::<Ib23, Fb23, Cold, C23>(23)
}
pub fn call24() -> u32 {
    scaled_fold::<Ib24, Fb24, Hot, C24>(24)
}
pub fn call26() -> u32 {
    scaled_fold::<Ib26, Fb26, Cold, C26>(26)
}
pub fn call27() -> u32 {
    scaled_fold::<Ib27, Fb27, Hot, C27>(27)
}
pub fn call28() -> u32 {
    scaled_fold::<Ib28, Fb28, Warm, C28>(28)
}
pub fn call29() -> u32 {
    scaled_fold::<Ib29, Fb29, Cold, C29>(29)
}
pub fn call30() -> u32 {
    scaled_fold::<Ib30, Fb30, Hot, C30>(30)
}
pub fn call31() -> u32 {
    scaled_fold::<Ib31, Fb31, Warm, C31>(31)
}
pub fn call32() -> u32 {
    scaled_fold::<Ib32, Fb32, Cold, C32>(32)
}
pub fn call33() -> u32 {
    scaled_fold::<Ib33, Fb33, Hot, C33>(33)
}
pub fn call34() -> u32 {
    scaled_fold::<Ib34, Fb34, Warm, C34>(34)
}
pub fn call36() -> u32 {
    scaled_fold::<Ib36, Fb36, Hot, C36>(36)
}
pub fn call37() -> u32 {
    scaled_fold::<Ib37, Fb37, Warm, C37>(37)
}
pub fn call38() -> u32 {
    scaled_fold::<Ib38, Fb38, Cold, C38>(38)
}
pub fn call39() -> u32 {
    scaled_fold::<Ib39, Fb39, Hot, C39>(39)
}
pub fn call40() -> u32 {
    scaled_fold::<Ib40, Fb40, Warm, C40>(40)
}
pub fn call41() -> u32 {
    scaled_fold::<Ib41, Fb41, Cold, C41>(41)
}
pub fn call42() -> u32 {
    scaled_fold::<Ib42, Fb42, Hot, C42>(42)
}
pub fn call43() -> u32 {
    scaled_fold::<Ib43, Fb43, Warm, C43>(43)
}
pub fn call45() -> u32 {
    scaled_fold::<Ib45, Fb45, Hot, C45>(45)
}
pub fn call46() -> u32 {
    scaled_fold::<Ib46, Fb46, Warm, C46>(46)
}
pub fn call47() -> u32 {
    scaled_fold::<Ib47, Fb47, Cold, C47>(47)
}
pub fn call48() -> u32 {
    scaled_fold::<Ib48, Fb48, Hot, C48>(48)
}
pub fn call49() -> u32 {
    scaled_fold::<Ib49, Fb49, Warm, C49>(49)
}
pub fn call50() -> u32 {
    scaled_fold::<Ib50, Fb50, Cold, C50>(50)
}
pub fn call51() -> u32 {
    scaled_fold::<Ib51, Fb51, Hot, C51>(51)
}
pub fn call52() -> u32 {
    scaled_fold::<Ib52, Fb52, Warm, C52>(52)
}
pub fn call54() -> u32 {
    scaled_fold::<Ib54, Fb54, Hot, C54>(54)
}
pub fn call55() -> u32 {
    scaled_fold::<Ib55, Fb55, Warm, C55>(55)
}
pub fn call56() -> u32 {
    scaled_fold::<Ib56, Fb56, Cold, C56>(56)
}
pub fn call57() -> u32 {
    scaled_fold::<Ib57, Fb57, Hot, C57>(57)
}
pub fn call58() -> u32 {
    scaled_fold::<Ib58, Fb58, Warm, C58>(58)
}
pub fn call59() -> u32 {
    scaled_fold::<Ib59, Fb59, Cold, C59>(59)
}
pub fn call60() -> u32 {
    scaled_fold::<Ib60, Fb60, Hot, C60>(60)
}
pub fn call61() -> u32 {
    scaled_fold::<Ib61, Fb61, Warm, C61>(61)
}
pub fn call63() -> u32 {
    scaled_fold::<Ib63, Fb63, Hot, C63>(63)
}
pub fn call64() -> u32 {
    scaled_fold::<Ib64, Fb64, Warm, C64>(64)
}
pub fn call65() -> u32 {
    scaled_fold::<Ib65, Fb65, Cold, C65>(65)
}
pub fn call66() -> u32 {
    scaled_fold::<Ib66, Fb66, Hot, C66>(66)
}
pub fn call67() -> u32 {
    scaled_fold::<Ib67, Fb67, Warm, C67>(67)
}
pub fn call68() -> u32 {
    scaled_fold::<Ib68, Fb68, Cold, C68>(68)
}
pub fn call69() -> u32 {
    scaled_fold::<Ib69, Fb69, Hot, C69>(69)
}
pub fn call70() -> u32 {
    scaled_fold::<Ib70, Fb70, Warm, C70>(70)
}
pub fn call72() -> u32 {
    scaled_fold::<Ib72, Fb72, Hot, C72>(72)
}
pub fn call73() -> u32 {
    scaled_fold::<Ib73, Fb73, Warm, C73>(73)
}
pub fn call74() -> u32 {
    scaled_fold::<Ib74, Fb74, Cold, C74>(74)
}
pub fn call75() -> u32 {
    scaled_fold::<Ib75, Fb75, Hot, C75>(75)
}
pub fn call76() -> u32 {
    scaled_fold::<Ib76, Fb76, Warm, C76>(76)
}
pub fn call77() -> u32 {
    scaled_fold::<Ib77, Fb77, Cold, C77>(77)
}
pub fn call78() -> u32 {
    scaled_fold::<Ib78, Fb78, Hot, C78>(78)
}
pub fn call79() -> u32 {
    scaled_fold::<Ib79, Fb79, Warm, C79>(79)
}
pub fn call81() -> u32 {
    scaled_fold::<Ib81, Fb81, Hot, C81>(81)
}
pub fn call82() -> u32 {
    scaled_fold::<Ib82, Fb82, Warm, C82>(82)
}
pub fn call83() -> u32 {
    scaled_fold::<Ib83, Fb83, Cold, C83>(83)
}
pub fn call84() -> u32 {
    scaled_fold::<Ib84, Fb84, Hot, C84>(84)
}
pub fn call85() -> u32 {
    scaled_fold::<Ib85, Fb85, Warm, C85>(85)
}
pub fn call86() -> u32 {
    scaled_fold::<Ib86, Fb86, Cold, C86>(86)
}
pub fn call87() -> u32 {
    scaled_fold::<Ib87, Fb87, Hot, C87>(87)
}
pub fn call88() -> u32 {
    scaled_fold::<Ib88, Fb88, Warm, C88>(88)
}
pub fn call90() -> u32 {
    scaled_fold::<Ib90, Fb90, Hot, C90>(90)
}
pub fn call91() -> u32 {
    scaled_fold::<Ib91, Fb91, Warm, C91>(91)
}
pub fn call92() -> u32 {
    scaled_fold::<Ib92, Fb92, Cold, C92>(92)
}
pub fn call93() -> u32 {
    scaled_fold::<Ib93, Fb93, Hot, C93>(93)
}
pub fn call94() -> u32 {
    scaled_fold::<Ib94, Fb94, Warm, C94>(94)
}
pub fn call95() -> u32 {
    scaled_fold::<Ib95, Fb95, Cold, C95>(95)
}
pub fn call96() -> u32 {
    scaled_fold::<Ib96, Fb96, Hot, C96>(96)
}
pub fn call98() -> u32 {
    scaled_fold::<Ib98, Fb98, Cold, C98>(98)
}
pub fn call99() -> u32 {
    scaled_fold::<Ib99, Fb99, Hot, C99>(99)
}
