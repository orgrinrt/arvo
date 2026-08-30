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
pub type Ib100 = Pz<I<O<O<O<I<O<H>>>>>>>;
pub type Fb100 = Z;
pub type Wb100 = Pz<I<O<O<O<I<O<H>>>>>>>;
pub type N100 = UFixed<Ib100, Fb100, Warm>;
pub const W100: usize = <Wb100 as Nat>::VAL;
const _: () = assert!(W100 == 81);
pub fn one_ok_100()
where
    N100: HasOne,
{
}
pub fn int_like_100()
where
    Fb100: IsZero,
{
}
pub type Ib101 = Pz<O<I<I<O<I<O<H>>>>>>>;
pub type Fb101 = Z;
pub type Wb101 = Pz<O<I<I<O<I<O<H>>>>>>>;
pub type N101 = UFixed<Ib101, Fb101, Cold>;
pub const W101: usize = <Wb101 as Nat>::VAL;
const _: () = assert!(W101 == 86);
pub fn one_ok_101()
where
    N101: HasOne,
{
}
pub fn int_like_101()
where
    Fb101: IsZero,
{
}
pub type Ib102 = Pz<I<O<O<O<O<I<H>>>>>>>;
pub type Fb102 = Z;
pub type Wb102 = Pz<I<O<O<O<O<I<H>>>>>>>;
pub type N102 = UFixed<Ib102, Fb102, Hot>;
pub const W102: usize = <Wb102 as Nat>::VAL;
const _: () = assert!(W102 == 97);
pub fn one_ok_102()
where
    N102: HasOne,
{
}
pub fn int_like_102()
where
    Fb102: IsZero,
{
}
pub type Ib103 = Pz<O<I<O<O<O<I<H>>>>>>>;
pub type Fb103 = Z;
pub type Wb103 = Pz<O<I<O<O<O<I<H>>>>>>>;
pub type N103 = UFixed<Ib103, Fb103, Warm>;
pub const W103: usize = <Wb103 as Nat>::VAL;
const _: () = assert!(W103 == 98);
pub fn one_ok_103()
where
    N103: HasOne,
{
}
pub fn int_like_103()
where
    Fb103: IsZero,
{
}
pub type Ib104 = Pz<O<I<I<O<O<O<O<H>>>>>>>>;
pub type Fb104 = Z;
pub type Wb104 = Pz<O<I<I<O<O<O<O<H>>>>>>>>;
pub type N104 = UFixed<Ib104, Fb104, Cold>;
pub const W104: usize = <Wb104 as Nat>::VAL;
const _: () = assert!(W104 == 134);
pub fn one_ok_104()
where
    N104: HasOne,
{
}
pub fn int_like_104()
where
    Fb104: IsZero,
{
}
pub type Ib105 = Z;
pub type Fb105 = Pz<O<I<O<I<H>>>>>;
pub type Wb105 = Pz<O<I<O<I<H>>>>>;
pub type N105 = UFixed<Ib105, Fb105, Hot>;
pub const W105: usize = <Wb105 as Nat>::VAL;
const _: () = assert!(W105 == 26);
pub fn frac_like_105()
where
    Fb105: NonZero,
{
}
pub type Ib106 = Pz<O<O<O<O<I<O<H>>>>>>>;
pub type Fb106 = Z;
pub type Wb106 = Pz<O<O<O<O<I<O<H>>>>>>>;
pub type N106 = UFixed<Ib106, Fb106, Warm>;
pub const W106: usize = <Wb106 as Nat>::VAL;
const _: () = assert!(W106 == 80);
pub fn one_ok_106()
where
    N106: HasOne,
{
}
pub fn int_like_106()
where
    Fb106: IsZero,
{
}
pub type Ib107 = Pz<O<I<O<O<I<O<H>>>>>>>;
pub type Fb107 = Z;
pub type Wb107 = Pz<O<I<O<O<I<O<H>>>>>>>;
pub type N107 = UFixed<Ib107, Fb107, Cold>;
pub const W107: usize = <Wb107 as Nat>::VAL;
const _: () = assert!(W107 == 82);
pub fn one_ok_107()
where
    N107: HasOne,
{
}
pub fn int_like_107()
where
    Fb107: IsZero,
{
}
pub type Ib108 = Pz<O<O<O<I<I<O<H>>>>>>>;
pub type Fb108 = Z;
pub type Wb108 = Pz<O<O<O<I<I<O<H>>>>>>>;
pub type N108 = UFixed<Ib108, Fb108, Hot>;
pub const W108: usize = <Wb108 as Nat>::VAL;
const _: () = assert!(W108 == 88);
pub fn one_ok_108()
where
    N108: HasOne,
{
}
pub fn int_like_108()
where
    Fb108: IsZero,
{
}
pub type Ib109 = Pz<I<O<I<I<I<O<H>>>>>>>;
pub type Fb109 = Z;
pub type Wb109 = Pz<I<O<I<I<I<O<H>>>>>>>;
pub type N109 = UFixed<Ib109, Fb109, Warm>;
pub const W109: usize = <Wb109 as Nat>::VAL;
const _: () = assert!(W109 == 93);
pub fn one_ok_109()
where
    N109: HasOne,
{
}
pub fn int_like_109()
where
    Fb109: IsZero,
{
}
pub type Ib110 = Pz<O<O<O<I<O<I<H>>>>>>>;
pub type Fb110 = Z;
pub type Wb110 = Pz<O<O<O<I<O<I<H>>>>>>>;
pub type N110 = UFixed<Ib110, Fb110, Cold>;
pub const W110: usize = <Wb110 as Nat>::VAL;
const _: () = assert!(W110 == 104);
pub fn one_ok_110()
where
    N110: HasOne,
{
}
pub fn int_like_110()
where
    Fb110: IsZero,
{
}
pub type Ib111 = Pz<I<O<O<I<O<I<H>>>>>>>;
pub type Fb111 = Z;
pub type Wb111 = Pz<I<O<O<I<O<I<H>>>>>>>;
pub type N111 = UFixed<Ib111, Fb111, Hot>;
pub const W111: usize = <Wb111 as Nat>::VAL;
const _: () = assert!(W111 == 105);
pub fn one_ok_111()
where
    N111: HasOne,
{
}
pub fn int_like_111()
where
    Fb111: IsZero,
{
}
pub type Ib112 = Pz<I<O<I<I<O<O<O<H>>>>>>>>;
pub type Fb112 = Z;
pub type Wb112 = Pz<I<O<I<I<O<O<O<H>>>>>>>>;
pub type N112 = UFixed<Ib112, Fb112, Warm>;
pub const W112: usize = <Wb112 as Nat>::VAL;
const _: () = assert!(W112 == 141);
pub fn one_ok_112()
where
    N112: HasOne,
{
}
pub fn int_like_112()
where
    Fb112: IsZero,
{
}
pub type Ib113 = Z;
pub type Fb113 = Pz<I<I<O<I<H>>>>>;
pub type Wb113 = Pz<I<I<O<I<H>>>>>;
pub type N113 = UFixed<Ib113, Fb113, Cold>;
pub const W113: usize = <Wb113 as Nat>::VAL;
const _: () = assert!(W113 == 27);
pub fn frac_like_113()
where
    Fb113: NonZero,
{
}
pub type Ib114 = Pz<I<I<I<O<I<O<H>>>>>>>;
pub type Fb114 = Z;
pub type Wb114 = Pz<I<I<I<O<I<O<H>>>>>>>;
pub type N114 = UFixed<Ib114, Fb114, Hot>;
pub const W114: usize = <Wb114 as Nat>::VAL;
const _: () = assert!(W114 == 87);
pub fn one_ok_114()
where
    N114: HasOne,
{
}
pub fn int_like_114()
where
    Fb114: IsZero,
{
}
pub type Ib115 = Pz<I<O<O<I<I<O<H>>>>>>>;
pub type Fb115 = Z;
pub type Wb115 = Pz<I<O<O<I<I<O<H>>>>>>>;
pub type N115 = UFixed<Ib115, Fb115, Warm>;
pub const W115: usize = <Wb115 as Nat>::VAL;
const _: () = assert!(W115 == 89);
pub fn one_ok_115()
where
    N115: HasOne,
{
}
pub fn int_like_115()
where
    Fb115: IsZero,
{
}
pub type Ib116 = Pz<I<I<I<I<I<O<H>>>>>>>;
pub type Fb116 = Z;
pub type Wb116 = Pz<I<I<I<I<I<O<H>>>>>>>;
pub type N116 = UFixed<Ib116, Fb116, Cold>;
pub const W116: usize = <Wb116 as Nat>::VAL;
const _: () = assert!(W116 == 95);
pub fn one_ok_116()
where
    N116: HasOne,
{
}
pub fn int_like_116()
where
    Fb116: IsZero,
{
}
pub type Ib117 = Pz<O<O<I<O<O<I<H>>>>>>>;
pub type Fb117 = Z;
pub type Wb117 = Pz<O<O<I<O<O<I<H>>>>>>>;
pub type N117 = UFixed<Ib117, Fb117, Hot>;
pub const W117: usize = <Wb117 as Nat>::VAL;
const _: () = assert!(W117 == 100);
pub fn one_ok_117()
where
    N117: HasOne,
{
}
pub fn int_like_117()
where
    Fb117: IsZero,
{
}
pub type Ib118 = Pz<I<I<I<I<O<I<H>>>>>>>;
pub type Fb118 = Z;
pub type Wb118 = Pz<I<I<I<I<O<I<H>>>>>>>;
pub type N118 = UFixed<Ib118, Fb118, Warm>;
pub const W118: usize = <Wb118 as Nat>::VAL;
const _: () = assert!(W118 == 111);
pub fn one_ok_118()
where
    N118: HasOne,
{
}
pub fn int_like_118()
where
    Fb118: IsZero,
{
}
pub type Ib119 = Pz<O<O<O<O<I<I<H>>>>>>>;
pub type Fb119 = Z;
pub type Wb119 = Pz<O<O<O<O<I<I<H>>>>>>>;
pub type N119 = UFixed<Ib119, Fb119, Cold>;
pub const W119: usize = <Wb119 as Nat>::VAL;
const _: () = assert!(W119 == 112);
pub fn one_ok_119()
where
    N119: HasOne,
{
}
pub fn int_like_119()
where
    Fb119: IsZero,
{
}
pub type Ib120 = Pz<O<O<I<O<I<O<O<H>>>>>>>>;
pub type Fb120 = Z;
pub type Wb120 = Pz<O<O<I<O<I<O<O<H>>>>>>>>;
pub type N120 = UFixed<Ib120, Fb120, Hot>;
pub const W120: usize = <Wb120 as Nat>::VAL;
const _: () = assert!(W120 == 148);
pub fn one_ok_120()
where
    N120: HasOne,
{
}
pub fn int_like_120()
where
    Fb120: IsZero,
{
}
pub type Ib121 = Z;
pub type Fb121 = Pz<O<O<I<I<H>>>>>;
pub type Wb121 = Pz<O<O<I<I<H>>>>>;
pub type N121 = UFixed<Ib121, Fb121, Warm>;
pub const W121: usize = <Wb121 as Nat>::VAL;
const _: () = assert!(W121 == 28);
pub fn frac_like_121()
where
    Fb121: NonZero,
{
}
pub type Ib122 = Pz<O<I<I<I<I<O<H>>>>>>>;
pub type Fb122 = Z;
pub type Wb122 = Pz<O<I<I<I<I<O<H>>>>>>>;
pub type N122 = UFixed<Ib122, Fb122, Cold>;
pub const W122: usize = <Wb122 as Nat>::VAL;
const _: () = assert!(W122 == 94);
pub fn one_ok_122()
where
    N122: HasOne,
{
}
pub fn int_like_122()
where
    Fb122: IsZero,
{
}
pub type Ib123 = Pz<O<O<O<O<O<I<H>>>>>>>;
pub type Fb123 = Z;
pub type Wb123 = Pz<O<O<O<O<O<I<H>>>>>>>;
pub type N123 = UFixed<Ib123, Fb123, Hot>;
pub const W123: usize = <Wb123 as Nat>::VAL;
const _: () = assert!(W123 == 96);
pub fn one_ok_123()
where
    N123: HasOne,
{
}
pub fn int_like_123()
where
    Fb123: IsZero,
{
}
pub type Ib124 = Pz<O<I<I<O<O<I<H>>>>>>>;
pub type Fb124 = Z;
pub type Wb124 = Pz<O<I<I<O<O<I<H>>>>>>>;
pub type N124 = UFixed<Ib124, Fb124, Warm>;
pub const W124: usize = <Wb124 as Nat>::VAL;
const _: () = assert!(W124 == 102);
pub fn one_ok_124()
where
    N124: HasOne,
{
}
pub fn int_like_124()
where
    Fb124: IsZero,
{
}
pub type Ib125 = Pz<I<I<O<I<O<I<H>>>>>>>;
pub type Fb125 = Z;
pub type Wb125 = Pz<I<I<O<I<O<I<H>>>>>>>;
pub type N125 = UFixed<Ib125, Fb125, Cold>;
pub const W125: usize = <Wb125 as Nat>::VAL;
const _: () = assert!(W125 == 107);
pub fn one_ok_125()
where
    N125: HasOne,
{
}
pub fn int_like_125()
where
    Fb125: IsZero,
{
}
pub type Ib126 = Pz<O<I<I<O<I<I<H>>>>>>>;
pub type Fb126 = Z;
pub type Wb126 = Pz<O<I<I<O<I<I<H>>>>>>>;
pub type N126 = UFixed<Ib126, Fb126, Hot>;
pub const W126: usize = <Wb126 as Nat>::VAL;
const _: () = assert!(W126 == 118);
pub fn one_ok_126()
where
    N126: HasOne,
{
}
pub fn int_like_126()
where
    Fb126: IsZero,
{
}
pub type Ib127 = Pz<I<I<I<O<I<I<H>>>>>>>;
pub type Fb127 = Z;
pub type Wb127 = Pz<I<I<I<O<I<I<H>>>>>>>;
pub type N127 = UFixed<Ib127, Fb127, Warm>;
pub const W127: usize = <Wb127 as Nat>::VAL;
const _: () = assert!(W127 == 119);
pub fn one_ok_127()
where
    N127: HasOne,
{
}
pub fn int_like_127()
where
    Fb127: IsZero,
{
}
pub type Ib128 = Pz<I<I<O<I<I<O<O<H>>>>>>>>;
pub type Fb128 = Z;
pub type Wb128 = Pz<I<I<O<I<I<O<O<H>>>>>>>>;
pub type N128 = UFixed<Ib128, Fb128, Cold>;
pub const W128: usize = <Wb128 as Nat>::VAL;
const _: () = assert!(W128 == 155);
pub fn one_ok_128()
where
    N128: HasOne,
{
}
pub fn int_like_128()
where
    Fb128: IsZero,
{
}
pub type Ib129 = Z;
pub type Fb129 = Pz<I<O<I<I<H>>>>>;
pub type Wb129 = Pz<I<O<I<I<H>>>>>;
pub type N129 = UFixed<Ib129, Fb129, Hot>;
pub const W129: usize = <Wb129 as Nat>::VAL;
const _: () = assert!(W129 == 29);
pub fn frac_like_129()
where
    Fb129: NonZero,
{
}
pub type Ib130 = Pz<I<O<I<O<O<I<H>>>>>>>;
pub type Fb130 = Z;
pub type Wb130 = Pz<I<O<I<O<O<I<H>>>>>>>;
pub type N130 = UFixed<Ib130, Fb130, Warm>;
pub const W130: usize = <Wb130 as Nat>::VAL;
const _: () = assert!(W130 == 101);
pub fn one_ok_130()
where
    N130: HasOne,
{
}
pub fn int_like_130()
where
    Fb130: IsZero,
{
}
pub type Ib131 = Pz<I<I<I<O<O<I<H>>>>>>>;
pub type Fb131 = Z;
pub type Wb131 = Pz<I<I<I<O<O<I<H>>>>>>>;
pub type N131 = UFixed<Ib131, Fb131, Cold>;
pub const W131: usize = <Wb131 as Nat>::VAL;
const _: () = assert!(W131 == 103);
pub fn one_ok_131()
where
    N131: HasOne,
{
}
pub fn int_like_131()
where
    Fb131: IsZero,
{
}
pub type Ib132 = Pz<I<O<I<I<O<I<H>>>>>>>;
pub type Fb132 = Z;
pub type Wb132 = Pz<I<O<I<I<O<I<H>>>>>>>;
pub type N132 = UFixed<Ib132, Fb132, Hot>;
pub const W132: usize = <Wb132 as Nat>::VAL;
const _: () = assert!(W132 == 109);
pub fn one_ok_132()
where
    N132: HasOne,
{
}
pub fn int_like_132()
where
    Fb132: IsZero,
{
}
pub type Ib133 = Pz<O<I<O<O<I<I<H>>>>>>>;
pub type Fb133 = Z;
pub type Wb133 = Pz<O<I<O<O<I<I<H>>>>>>>;
pub type N133 = UFixed<Ib133, Fb133, Warm>;
pub const W133: usize = <Wb133 as Nat>::VAL;
const _: () = assert!(W133 == 114);
pub fn one_ok_133()
where
    N133: HasOne,
{
}
pub fn int_like_133()
where
    Fb133: IsZero,
{
}
pub type Ib134 = Pz<I<O<I<I<I<I<H>>>>>>>;
pub type Fb134 = Z;
pub type Wb134 = Pz<I<O<I<I<I<I<H>>>>>>>;
pub type N134 = UFixed<Ib134, Fb134, Cold>;
pub const W134: usize = <Wb134 as Nat>::VAL;
const _: () = assert!(W134 == 125);
pub fn one_ok_134()
where
    N134: HasOne,
{
}
pub fn int_like_134()
where
    Fb134: IsZero,
{
}
pub type Ib135 = Pz<O<I<I<I<I<I<H>>>>>>>;
pub type Fb135 = Z;
pub type Wb135 = Pz<O<I<I<I<I<I<H>>>>>>>;
pub type N135 = UFixed<Ib135, Fb135, Hot>;
pub const W135: usize = <Wb135 as Nat>::VAL;
const _: () = assert!(W135 == 126);
pub fn one_ok_135()
where
    N135: HasOne,
{
}
pub fn int_like_135()
where
    Fb135: IsZero,
{
}
pub type Ib136 = Pz<O<I<O<O<O<I<O<H>>>>>>>>;
pub type Fb136 = Z;
pub type Wb136 = Pz<O<I<O<O<O<I<O<H>>>>>>>>;
pub type N136 = UFixed<Ib136, Fb136, Warm>;
pub const W136: usize = <Wb136 as Nat>::VAL;
const _: () = assert!(W136 == 162);
pub fn one_ok_136()
where
    N136: HasOne,
{
}
pub fn int_like_136()
where
    Fb136: IsZero,
{
}
pub type Ib137 = Z;
pub type Fb137 = Pz<O<I<I<I<H>>>>>;
pub type Wb137 = Pz<O<I<I<I<H>>>>>;
pub type N137 = UFixed<Ib137, Fb137, Cold>;
pub const W137: usize = <Wb137 as Nat>::VAL;
const _: () = assert!(W137 == 30);
pub fn frac_like_137()
where
    Fb137: NonZero,
{
}
pub type Ib138 = Pz<O<O<I<I<O<I<H>>>>>>>;
pub type Fb138 = Z;
pub type Wb138 = Pz<O<O<I<I<O<I<H>>>>>>>;
pub type N138 = UFixed<Ib138, Fb138, Hot>;
pub const W138: usize = <Wb138 as Nat>::VAL;
const _: () = assert!(W138 == 108);
pub fn one_ok_138()
where
    N138: HasOne,
{
}
pub fn int_like_138()
where
    Fb138: IsZero,
{
}
pub type Ib139 = Pz<O<I<I<I<O<I<H>>>>>>>;
pub type Fb139 = Z;
pub type Wb139 = Pz<O<I<I<I<O<I<H>>>>>>>;
pub type N139 = UFixed<Ib139, Fb139, Warm>;
pub const W139: usize = <Wb139 as Nat>::VAL;
const _: () = assert!(W139 == 110);
pub fn one_ok_139()
where
    N139: HasOne,
{
}
pub fn int_like_139()
where
    Fb139: IsZero,
{
}
pub type Ib140 = Pz<O<O<I<O<I<I<H>>>>>>>;
pub type Fb140 = Z;
pub type Wb140 = Pz<O<O<I<O<I<I<H>>>>>>>;
pub type N140 = UFixed<Ib140, Fb140, Cold>;
pub const W140: usize = <Wb140 as Nat>::VAL;
const _: () = assert!(W140 == 116);
pub fn one_ok_140()
where
    N140: HasOne,
{
}
pub fn int_like_140()
where
    Fb140: IsZero,
{
}
pub type Ib141 = Pz<I<O<O<I<I<I<H>>>>>>>;
pub type Fb141 = Z;
pub type Wb141 = Pz<I<O<O<I<I<I<H>>>>>>>;
pub type N141 = UFixed<Ib141, Fb141, Hot>;
pub const W141: usize = <Wb141 as Nat>::VAL;
const _: () = assert!(W141 == 121);
pub fn one_ok_141()
where
    N141: HasOne,
{
}
pub fn int_like_141()
where
    Fb141: IsZero,
{
}
pub type Ib142 = Pz<O<O<I<O<O<O<O<H>>>>>>>>;
pub type Fb142 = Z;
pub type Wb142 = Pz<O<O<I<O<O<O<O<H>>>>>>>>;
pub type N142 = UFixed<Ib142, Fb142, Warm>;
pub const W142: usize = <Wb142 as Nat>::VAL;
const _: () = assert!(W142 == 132);
pub fn one_ok_142()
where
    N142: HasOne,
{
}
pub fn int_like_142()
where
    Fb142: IsZero,
{
}
pub type Ib143 = Pz<I<O<I<O<O<O<O<H>>>>>>>>;
pub type Fb143 = Z;
pub type Wb143 = Pz<I<O<I<O<O<O<O<H>>>>>>>>;
pub type N143 = UFixed<Ib143, Fb143, Cold>;
pub const W143: usize = <Wb143 as Nat>::VAL;
const _: () = assert!(W143 == 133);
pub fn one_ok_143()
where
    N143: HasOne,
{
}
pub fn int_like_143()
where
    Fb143: IsZero,
{
}
pub type Ib144 = Pz<I<O<O<I<O<I<O<H>>>>>>>>;
pub type Fb144 = Z;
pub type Wb144 = Pz<I<O<O<I<O<I<O<H>>>>>>>>;
pub type N144 = UFixed<Ib144, Fb144, Hot>;
pub const W144: usize = <Wb144 as Nat>::VAL;
const _: () = assert!(W144 == 169);
pub fn one_ok_144()
where
    N144: HasOne,
{
}
pub fn int_like_144()
where
    Fb144: IsZero,
{
}
pub type Ib145 = Z;
pub type Fb145 = Pz<I<I<I<I<H>>>>>;
pub type Wb145 = Pz<I<I<I<I<H>>>>>;
pub type N145 = UFixed<Ib145, Fb145, Warm>;
pub const W145: usize = <Wb145 as Nat>::VAL;
const _: () = assert!(W145 == 31);
pub fn frac_like_145()
where
    Fb145: NonZero,
{
}
pub type Ib146 = Pz<I<I<O<O<I<I<H>>>>>>>;
pub type Fb146 = Z;
pub type Wb146 = Pz<I<I<O<O<I<I<H>>>>>>>;
pub type N146 = UFixed<Ib146, Fb146, Cold>;
pub const W146: usize = <Wb146 as Nat>::VAL;
const _: () = assert!(W146 == 115);
pub fn one_ok_146()
where
    N146: HasOne,
{
}
pub fn int_like_146()
where
    Fb146: IsZero,
{
}
pub type Ib147 = Pz<I<O<I<O<I<I<H>>>>>>>;
pub type Fb147 = Z;
pub type Wb147 = Pz<I<O<I<O<I<I<H>>>>>>>;
pub type N147 = UFixed<Ib147, Fb147, Hot>;
pub const W147: usize = <Wb147 as Nat>::VAL;
const _: () = assert!(W147 == 117);
pub fn one_ok_147()
where
    N147: HasOne,
{
}
pub fn int_like_147()
where
    Fb147: IsZero,
{
}
pub type Ib148 = Pz<I<I<O<I<I<I<H>>>>>>>;
pub type Fb148 = Z;
pub type Wb148 = Pz<I<I<O<I<I<I<H>>>>>>>;
pub type N148 = UFixed<Ib148, Fb148, Warm>;
pub const W148: usize = <Wb148 as Nat>::VAL;
const _: () = assert!(W148 == 123);
pub fn one_ok_148()
where
    N148: HasOne,
{
}
pub fn int_like_148()
where
    Fb148: IsZero,
{
}
pub type Ib149 = Pz<O<O<O<O<O<O<O<H>>>>>>>>;
pub type Fb149 = Z;
pub type Wb149 = Pz<O<O<O<O<O<O<O<H>>>>>>>>;
pub type N149 = UFixed<Ib149, Fb149, Cold>;
pub const W149: usize = <Wb149 as Nat>::VAL;
const _: () = assert!(W149 == 128);
pub fn one_ok_149()
where
    N149: HasOne,
{
}
pub fn int_like_149()
where
    Fb149: IsZero,
{
}
pub type Ib150 = Pz<I<I<O<I<O<O<O<H>>>>>>>>;
pub type Fb150 = Z;
pub type Wb150 = Pz<I<I<O<I<O<O<O<H>>>>>>>>;
pub type N150 = UFixed<Ib150, Fb150, Hot>;
pub const W150: usize = <Wb150 as Nat>::VAL;
const _: () = assert!(W150 == 139);
pub fn one_ok_150()
where
    N150: HasOne,
{
}
pub fn int_like_150()
where
    Fb150: IsZero,
{
}
pub type Ib151 = Pz<O<O<I<I<O<O<O<H>>>>>>>>;
pub type Fb151 = Z;
pub type Wb151 = Pz<O<O<I<I<O<O<O<H>>>>>>>>;
pub type N151 = UFixed<Ib151, Fb151, Warm>;
pub const W151: usize = <Wb151 as Nat>::VAL;
const _: () = assert!(W151 == 140);
pub fn one_ok_151()
where
    N151: HasOne,
{
}
pub fn int_like_151()
where
    Fb151: IsZero,
{
}
pub type Ib152 = Pz<O<O<O<O<I<I<O<H>>>>>>>>;
pub type Fb152 = Z;
pub type Wb152 = Pz<O<O<O<O<I<I<O<H>>>>>>>>;
pub type N152 = UFixed<Ib152, Fb152, Cold>;
pub const W152: usize = <Wb152 as Nat>::VAL;
const _: () = assert!(W152 == 176);
pub fn one_ok_152()
where
    N152: HasOne,
{
}
pub fn int_like_152()
where
    Fb152: IsZero,
{
}
pub type Ib153 = Z;
pub type Fb153 = Pz<O<O<O<O<O<H>>>>>>;
pub type Wb153 = Pz<O<O<O<O<O<H>>>>>>;
pub type N153 = UFixed<Ib153, Fb153, Hot>;
pub const W153: usize = <Wb153 as Nat>::VAL;
const _: () = assert!(W153 == 32);
pub fn frac_like_153()
where
    Fb153: NonZero,
{
}
pub type Ib154 = Pz<O<I<O<I<I<I<H>>>>>>>;
pub type Fb154 = Z;
pub type Wb154 = Pz<O<I<O<I<I<I<H>>>>>>>;
pub type N154 = UFixed<Ib154, Fb154, Warm>;
pub const W154: usize = <Wb154 as Nat>::VAL;
const _: () = assert!(W154 == 122);
pub fn one_ok_154()
where
    N154: HasOne,
{
}
pub fn int_like_154()
where
    Fb154: IsZero,
{
}
pub type Ib155 = Pz<O<O<I<I<I<I<H>>>>>>>;
pub type Fb155 = Z;
pub type Wb155 = Pz<O<O<I<I<I<I<H>>>>>>>;
pub type N155 = UFixed<Ib155, Fb155, Cold>;
pub const W155: usize = <Wb155 as Nat>::VAL;
const _: () = assert!(W155 == 124);
pub fn one_ok_155()
where
    N155: HasOne,
{
}
pub fn int_like_155()
where
    Fb155: IsZero,
{
}
pub type Ib156 = Pz<O<I<O<O<O<O<O<H>>>>>>>>;
pub type Fb156 = Z;
pub type Wb156 = Pz<O<I<O<O<O<O<O<H>>>>>>>>;
pub type N156 = UFixed<Ib156, Fb156, Hot>;
pub const W156: usize = <Wb156 as Nat>::VAL;
const _: () = assert!(W156 == 130);
pub fn one_ok_156()
where
    N156: HasOne,
{
}
pub fn int_like_156()
where
    Fb156: IsZero,
{
}
pub type Ib157 = Pz<I<I<I<O<O<O<O<H>>>>>>>>;
pub type Fb157 = Z;
pub type Wb157 = Pz<I<I<I<O<O<O<O<H>>>>>>>>;
pub type N157 = UFixed<Ib157, Fb157, Warm>;
pub const W157: usize = <Wb157 as Nat>::VAL;
const _: () = assert!(W157 == 135);
pub fn one_ok_157()
where
    N157: HasOne,
{
}
pub fn int_like_157()
where
    Fb157: IsZero,
{
}
pub type Ib158 = Pz<O<I<O<O<I<O<O<H>>>>>>>>;
pub type Fb158 = Z;
pub type Wb158 = Pz<O<I<O<O<I<O<O<H>>>>>>>>;
pub type N158 = UFixed<Ib158, Fb158, Cold>;
pub const W158: usize = <Wb158 as Nat>::VAL;
const _: () = assert!(W158 == 146);
pub fn one_ok_158()
where
    N158: HasOne,
{
}
pub fn int_like_158()
where
    Fb158: IsZero,
{
}
pub type Ib159 = Pz<I<I<O<O<I<O<O<H>>>>>>>>;
pub type Fb159 = Z;
pub type Wb159 = Pz<I<I<O<O<I<O<O<H>>>>>>>>;
pub type N159 = UFixed<Ib159, Fb159, Hot>;
pub const W159: usize = <Wb159 as Nat>::VAL;
const _: () = assert!(W159 == 147);
pub fn one_ok_159()
where
    N159: HasOne,
{
}
pub fn int_like_159()
where
    Fb159: IsZero,
{
}
pub type Ib160 = Pz<I<I<I<O<I<I<O<H>>>>>>>>;
pub type Fb160 = Z;
pub type Wb160 = Pz<I<I<I<O<I<I<O<H>>>>>>>>;
pub type N160 = UFixed<Ib160, Fb160, Warm>;
pub const W160: usize = <Wb160 as Nat>::VAL;
const _: () = assert!(W160 == 183);
pub fn one_ok_160()
where
    N160: HasOne,
{
}
pub fn int_like_160()
where
    Fb160: IsZero,
{
}
pub type Ib161 = Z;
pub type Fb161 = Pz<I<O<O<O<O<H>>>>>>;
pub type Wb161 = Pz<I<O<O<O<O<H>>>>>>;
pub type N161 = UFixed<Ib161, Fb161, Cold>;
pub const W161: usize = <Wb161 as Nat>::VAL;
const _: () = assert!(W161 == 33);
pub fn frac_like_161()
where
    Fb161: NonZero,
{
}
pub type Ib162 = Pz<I<O<O<O<O<O<O<H>>>>>>>>;
pub type Fb162 = Z;
pub type Wb162 = Pz<I<O<O<O<O<O<O<H>>>>>>>>;
pub type N162 = UFixed<Ib162, Fb162, Hot>;
pub const W162: usize = <Wb162 as Nat>::VAL;
const _: () = assert!(W162 == 129);
pub fn one_ok_162()
where
    N162: HasOne,
{
}
pub fn int_like_162()
where
    Fb162: IsZero,
{
}
pub type Ib163 = Pz<I<I<O<O<O<O<O<H>>>>>>>>;
pub type Fb163 = Z;
pub type Wb163 = Pz<I<I<O<O<O<O<O<H>>>>>>>>;
pub type N163 = UFixed<Ib163, Fb163, Warm>;
pub const W163: usize = <Wb163 as Nat>::VAL;
const _: () = assert!(W163 == 131);
pub fn one_ok_163()
where
    N163: HasOne,
{
}
pub fn int_like_163()
where
    Fb163: IsZero,
{
}
pub type Ib164 = Pz<I<O<O<I<O<O<O<H>>>>>>>>;
pub type Fb164 = Z;
pub type Wb164 = Pz<I<O<O<I<O<O<O<H>>>>>>>>;
pub type N164 = UFixed<Ib164, Fb164, Cold>;
pub const W164: usize = <Wb164 as Nat>::VAL;
const _: () = assert!(W164 == 137);
pub fn one_ok_164()
where
    N164: HasOne,
{
}
pub fn int_like_164()
where
    Fb164: IsZero,
{
}
pub type Ib165 = Pz<O<I<I<I<O<O<O<H>>>>>>>>;
pub type Fb165 = Z;
pub type Wb165 = Pz<O<I<I<I<O<O<O<H>>>>>>>>;
pub type N165 = UFixed<Ib165, Fb165, Hot>;
pub const W165: usize = <Wb165 as Nat>::VAL;
const _: () = assert!(W165 == 142);
pub fn one_ok_165()
where
    N165: HasOne,
{
}
pub fn int_like_165()
where
    Fb165: IsZero,
{
}
pub type Ib166 = Pz<I<O<O<I<I<O<O<H>>>>>>>>;
pub type Fb166 = Z;
pub type Wb166 = Pz<I<O<O<I<I<O<O<H>>>>>>>>;
pub type N166 = UFixed<Ib166, Fb166, Warm>;
pub const W166: usize = <Wb166 as Nat>::VAL;
const _: () = assert!(W166 == 153);
pub fn one_ok_166()
where
    N166: HasOne,
{
}
pub fn int_like_166()
where
    Fb166: IsZero,
{
}
pub type Ib167 = Pz<O<I<O<I<I<O<O<H>>>>>>>>;
pub type Fb167 = Z;
pub type Wb167 = Pz<O<I<O<I<I<O<O<H>>>>>>>>;
pub type N167 = UFixed<Ib167, Fb167, Cold>;
pub const W167: usize = <Wb167 as Nat>::VAL;
const _: () = assert!(W167 == 154);
pub fn one_ok_167()
where
    N167: HasOne,
{
}
pub fn int_like_167()
where
    Fb167: IsZero,
{
}
pub type Ib168 = Pz<O<I<I<I<I<I<O<H>>>>>>>>;
pub type Fb168 = Z;
pub type Wb168 = Pz<O<I<I<I<I<I<O<H>>>>>>>>;
pub type N168 = UFixed<Ib168, Fb168, Hot>;
pub const W168: usize = <Wb168 as Nat>::VAL;
const _: () = assert!(W168 == 190);
pub fn one_ok_168()
where
    N168: HasOne,
{
}
pub fn int_like_168()
where
    Fb168: IsZero,
{
}
pub type Ib169 = Z;
pub type Fb169 = Pz<O<I<O<O<O<H>>>>>>;
pub type Wb169 = Pz<O<I<O<O<O<H>>>>>>;
pub type N169 = UFixed<Ib169, Fb169, Warm>;
pub const W169: usize = <Wb169 as Nat>::VAL;
const _: () = assert!(W169 == 34);
pub fn frac_like_169()
where
    Fb169: NonZero,
{
}
pub type Ib170 = Pz<O<O<O<I<O<O<O<H>>>>>>>>;
pub type Fb170 = Z;
pub type Wb170 = Pz<O<O<O<I<O<O<O<H>>>>>>>>;
pub type N170 = UFixed<Ib170, Fb170, Cold>;
pub const W170: usize = <Wb170 as Nat>::VAL;
const _: () = assert!(W170 == 136);
pub fn one_ok_170()
where
    N170: HasOne,
{
}
pub fn int_like_170()
where
    Fb170: IsZero,
{
}
pub type Ib171 = Pz<O<I<O<I<O<O<O<H>>>>>>>>;
pub type Fb171 = Z;
pub type Wb171 = Pz<O<I<O<I<O<O<O<H>>>>>>>>;
pub type N171 = UFixed<Ib171, Fb171, Hot>;
pub const W171: usize = <Wb171 as Nat>::VAL;
const _: () = assert!(W171 == 138);
pub fn one_ok_171()
where
    N171: HasOne,
{
}
pub fn int_like_171()
where
    Fb171: IsZero,
{
}
pub type Ib172 = Pz<O<O<O<O<I<O<O<H>>>>>>>>;
pub type Fb172 = Z;
pub type Wb172 = Pz<O<O<O<O<I<O<O<H>>>>>>>>;
pub type N172 = UFixed<Ib172, Fb172, Warm>;
pub const W172: usize = <Wb172 as Nat>::VAL;
const _: () = assert!(W172 == 144);
pub fn one_ok_172()
where
    N172: HasOne,
{
}
pub fn int_like_172()
where
    Fb172: IsZero,
{
}
pub type Ib173 = Pz<I<O<I<O<I<O<O<H>>>>>>>>;
pub type Fb173 = Z;
pub type Wb173 = Pz<I<O<I<O<I<O<O<H>>>>>>>>;
pub type N173 = UFixed<Ib173, Fb173, Cold>;
pub const W173: usize = <Wb173 as Nat>::VAL;
const _: () = assert!(W173 == 149);
pub fn one_ok_173()
where
    N173: HasOne,
{
}
pub fn int_like_173()
where
    Fb173: IsZero,
{
}
pub type Ib174 = Pz<O<O<O<O<O<I<O<H>>>>>>>>;
pub type Fb174 = Z;
pub type Wb174 = Pz<O<O<O<O<O<I<O<H>>>>>>>>;
pub type N174 = UFixed<Ib174, Fb174, Hot>;
pub const W174: usize = <Wb174 as Nat>::VAL;
const _: () = assert!(W174 == 160);
pub fn one_ok_174()
where
    N174: HasOne,
{
}
pub fn int_like_174()
where
    Fb174: IsZero,
{
}
pub type Ib175 = Pz<I<O<O<O<O<I<O<H>>>>>>>>;
pub type Fb175 = Z;
pub type Wb175 = Pz<I<O<O<O<O<I<O<H>>>>>>>>;
pub type N175 = UFixed<Ib175, Fb175, Warm>;
pub const W175: usize = <Wb175 as Nat>::VAL;
const _: () = assert!(W175 == 161);
pub fn one_ok_175()
where
    N175: HasOne,
{
}
pub fn int_like_175()
where
    Fb175: IsZero,
{
}
pub type Ib176 = Pz<I<O<I<O<O<O<I<H>>>>>>>>;
pub type Fb176 = Z;
pub type Wb176 = Pz<I<O<I<O<O<O<I<H>>>>>>>>;
pub type N176 = UFixed<Ib176, Fb176, Cold>;
pub const W176: usize = <Wb176 as Nat>::VAL;
const _: () = assert!(W176 == 197);
pub fn one_ok_176()
where
    N176: HasOne,
{
}
pub fn int_like_176()
where
    Fb176: IsZero,
{
}
pub type Ib177 = Z;
pub type Fb177 = Pz<I<I<O<O<O<H>>>>>>;
pub type Wb177 = Pz<I<I<O<O<O<H>>>>>>;
pub type N177 = UFixed<Ib177, Fb177, Hot>;
pub const W177: usize = <Wb177 as Nat>::VAL;
const _: () = assert!(W177 == 35);
pub fn frac_like_177()
where
    Fb177: NonZero,
{
}
pub type Ib178 = Pz<I<I<I<I<O<O<O<H>>>>>>>>;
pub type Fb178 = Z;
pub type Wb178 = Pz<I<I<I<I<O<O<O<H>>>>>>>>;
pub type N178 = UFixed<Ib178, Fb178, Warm>;
pub const W178: usize = <Wb178 as Nat>::VAL;
const _: () = assert!(W178 == 143);
pub fn one_ok_178()
where
    N178: HasOne,
{
}
pub fn int_like_178()
where
    Fb178: IsZero,
{
}
pub type Ib179 = Pz<I<O<O<O<I<O<O<H>>>>>>>>;
pub type Fb179 = Z;
pub type Wb179 = Pz<I<O<O<O<I<O<O<H>>>>>>>>;
pub type N179 = UFixed<Ib179, Fb179, Cold>;
pub const W179: usize = <Wb179 as Nat>::VAL;
const _: () = assert!(W179 == 145);
pub fn one_ok_179()
where
    N179: HasOne,
{
}
pub fn int_like_179()
where
    Fb179: IsZero,
{
}
pub type Ib180 = Pz<I<I<I<O<I<O<O<H>>>>>>>>;
pub type Fb180 = Z;
pub type Wb180 = Pz<I<I<I<O<I<O<O<H>>>>>>>>;
pub type N180 = UFixed<Ib180, Fb180, Hot>;
pub const W180: usize = <Wb180 as Nat>::VAL;
const _: () = assert!(W180 == 151);
pub fn one_ok_180()
where
    N180: HasOne,
{
}
pub fn int_like_180()
where
    Fb180: IsZero,
{
}
pub type Ib181 = Pz<O<O<I<I<I<O<O<H>>>>>>>>;
pub type Fb181 = Z;
pub type Wb181 = Pz<O<O<I<I<I<O<O<H>>>>>>>>;
pub type N181 = UFixed<Ib181, Fb181, Warm>;
pub const W181: usize = <Wb181 as Nat>::VAL;
const _: () = assert!(W181 == 156);
pub fn one_ok_181()
where
    N181: HasOne,
{
}
pub fn int_like_181()
where
    Fb181: IsZero,
{
}
pub type Ib182 = Pz<I<I<I<O<O<I<O<H>>>>>>>>;
pub type Fb182 = Z;
pub type Wb182 = Pz<I<I<I<O<O<I<O<H>>>>>>>>;
pub type N182 = UFixed<Ib182, Fb182, Cold>;
pub const W182: usize = <Wb182 as Nat>::VAL;
const _: () = assert!(W182 == 167);
pub fn one_ok_182()
where
    N182: HasOne,
{
}
pub fn int_like_182()
where
    Fb182: IsZero,
{
}
pub type Ib183 = Pz<O<O<O<I<O<I<O<H>>>>>>>>;
pub type Fb183 = Z;
pub type Wb183 = Pz<O<O<O<I<O<I<O<H>>>>>>>>;
pub type N183 = UFixed<Ib183, Fb183, Hot>;
pub const W183: usize = <Wb183 as Nat>::VAL;
const _: () = assert!(W183 == 168);
pub fn one_ok_183()
where
    N183: HasOne,
{
}
pub fn int_like_183()
where
    Fb183: IsZero,
{
}
pub type Ib184 = Pz<O<O<I<I<O<O<I<H>>>>>>>>;
pub type Fb184 = Z;
pub type Wb184 = Pz<O<O<I<I<O<O<I<H>>>>>>>>;
pub type N184 = UFixed<Ib184, Fb184, Warm>;
pub const W184: usize = <Wb184 as Nat>::VAL;
const _: () = assert!(W184 == 204);
pub fn one_ok_184()
where
    N184: HasOne,
{
}
pub fn int_like_184()
where
    Fb184: IsZero,
{
}
pub type Ib185 = Z;
pub type Fb185 = Pz<O<O<I<O<O<H>>>>>>;
pub type Wb185 = Pz<O<O<I<O<O<H>>>>>>;
pub type N185 = UFixed<Ib185, Fb185, Cold>;
pub const W185: usize = <Wb185 as Nat>::VAL;
const _: () = assert!(W185 == 36);
pub fn frac_like_185()
where
    Fb185: NonZero,
{
}
pub type Ib186 = Pz<O<I<I<O<I<O<O<H>>>>>>>>;
pub type Fb186 = Z;
pub type Wb186 = Pz<O<I<I<O<I<O<O<H>>>>>>>>;
pub type N186 = UFixed<Ib186, Fb186, Hot>;
pub const W186: usize = <Wb186 as Nat>::VAL;
const _: () = assert!(W186 == 150);
pub fn one_ok_186()
where
    N186: HasOne,
{
}
pub fn int_like_186()
where
    Fb186: IsZero,
{
}
pub type Ib187 = Pz<O<O<O<I<I<O<O<H>>>>>>>>;
pub type Fb187 = Z;
pub type Wb187 = Pz<O<O<O<I<I<O<O<H>>>>>>>>;
pub type N187 = UFixed<Ib187, Fb187, Warm>;
pub const W187: usize = <Wb187 as Nat>::VAL;
const _: () = assert!(W187 == 152);
pub fn one_ok_187()
where
    N187: HasOne,
{
}
pub fn int_like_187()
where
    Fb187: IsZero,
{
}
pub type Ib188 = Pz<O<I<I<I<I<O<O<H>>>>>>>>;
pub type Fb188 = Z;
pub type Wb188 = Pz<O<I<I<I<I<O<O<H>>>>>>>>;
pub type N188 = UFixed<Ib188, Fb188, Cold>;
pub const W188: usize = <Wb188 as Nat>::VAL;
const _: () = assert!(W188 == 158);
pub fn one_ok_188()
where
    N188: HasOne,
{
}
pub fn int_like_188()
where
    Fb188: IsZero,
{
}
pub type Ib189 = Pz<I<I<O<O<O<I<O<H>>>>>>>>;
pub type Fb189 = Z;
pub type Wb189 = Pz<I<I<O<O<O<I<O<H>>>>>>>>;
pub type N189 = UFixed<Ib189, Fb189, Hot>;
pub const W189: usize = <Wb189 as Nat>::VAL;
const _: () = assert!(W189 == 163);
pub fn one_ok_189()
where
    N189: HasOne,
{
}
pub fn int_like_189()
where
    Fb189: IsZero,
{
}
pub type Ib190 = Pz<O<I<I<I<O<I<O<H>>>>>>>>;
pub type Fb190 = Z;
pub type Wb190 = Pz<O<I<I<I<O<I<O<H>>>>>>>>;
pub type N190 = UFixed<Ib190, Fb190, Warm>;
pub const W190: usize = <Wb190 as Nat>::VAL;
const _: () = assert!(W190 == 174);
pub fn one_ok_190()
where
    N190: HasOne,
{
}
pub fn int_like_190()
where
    Fb190: IsZero,
{
}
pub type Ib191 = Pz<I<I<I<I<O<I<O<H>>>>>>>>;
pub type Fb191 = Z;
pub type Wb191 = Pz<I<I<I<I<O<I<O<H>>>>>>>>;
pub type N191 = UFixed<Ib191, Fb191, Cold>;
pub const W191: usize = <Wb191 as Nat>::VAL;
const _: () = assert!(W191 == 175);
pub fn one_ok_191()
where
    N191: HasOne,
{
}
pub fn int_like_191()
where
    Fb191: IsZero,
{
}
pub type Ib192 = Pz<I<I<O<O<I<O<I<H>>>>>>>>;
pub type Fb192 = Z;
pub type Wb192 = Pz<I<I<O<O<I<O<I<H>>>>>>>>;
pub type N192 = UFixed<Ib192, Fb192, Hot>;
pub const W192: usize = <Wb192 as Nat>::VAL;
const _: () = assert!(W192 == 211);
pub fn one_ok_192()
where
    N192: HasOne,
{
}
pub fn int_like_192()
where
    Fb192: IsZero,
{
}
pub type Ib193 = Z;
pub type Fb193 = Pz<I<O<I<O<O<H>>>>>>;
pub type Wb193 = Pz<I<O<I<O<O<H>>>>>>;
pub type N193 = UFixed<Ib193, Fb193, Warm>;
pub const W193: usize = <Wb193 as Nat>::VAL;
const _: () = assert!(W193 == 37);
pub fn frac_like_193()
where
    Fb193: NonZero,
{
}
pub type Ib194 = Pz<I<O<I<I<I<O<O<H>>>>>>>>;
pub type Fb194 = Z;
pub type Wb194 = Pz<I<O<I<I<I<O<O<H>>>>>>>>;
pub type N194 = UFixed<Ib194, Fb194, Cold>;
pub const W194: usize = <Wb194 as Nat>::VAL;
const _: () = assert!(W194 == 157);
pub fn one_ok_194()
where
    N194: HasOne,
{
}
pub fn int_like_194()
where
    Fb194: IsZero,
{
}
pub type Ib195 = Pz<I<I<I<I<I<O<O<H>>>>>>>>;
pub type Fb195 = Z;
pub type Wb195 = Pz<I<I<I<I<I<O<O<H>>>>>>>>;
pub type N195 = UFixed<Ib195, Fb195, Hot>;
pub const W195: usize = <Wb195 as Nat>::VAL;
const _: () = assert!(W195 == 159);
pub fn one_ok_195()
where
    N195: HasOne,
{
}
pub fn int_like_195()
where
    Fb195: IsZero,
{
}
pub type Ib196 = Pz<I<O<I<O<O<I<O<H>>>>>>>>;
pub type Fb196 = Z;
pub type Wb196 = Pz<I<O<I<O<O<I<O<H>>>>>>>>;
pub type N196 = UFixed<Ib196, Fb196, Warm>;
pub const W196: usize = <Wb196 as Nat>::VAL;
const _: () = assert!(W196 == 165);
pub fn one_ok_196()
where
    N196: HasOne,
{
}
pub fn int_like_196()
where
    Fb196: IsZero,
{
}
pub type Ib197 = Pz<O<I<O<I<O<I<O<H>>>>>>>>;
pub type Fb197 = Z;
pub type Wb197 = Pz<O<I<O<I<O<I<O<H>>>>>>>>;
pub type N197 = UFixed<Ib197, Fb197, Cold>;
pub const W197: usize = <Wb197 as Nat>::VAL;
const _: () = assert!(W197 == 170);
pub fn one_ok_197()
where
    N197: HasOne,
{
}
pub fn int_like_197()
where
    Fb197: IsZero,
{
}
pub type Ib198 = Pz<I<O<I<O<I<I<O<H>>>>>>>>;
pub type Fb198 = Z;
pub type Wb198 = Pz<I<O<I<O<I<I<O<H>>>>>>>>;
pub type N198 = UFixed<Ib198, Fb198, Hot>;
pub const W198: usize = <Wb198 as Nat>::VAL;
const _: () = assert!(W198 == 181);
pub fn one_ok_198()
where
    N198: HasOne,
{
}
pub fn int_like_198()
where
    Fb198: IsZero,
{
}
pub type Ib199 = Pz<O<I<I<O<I<I<O<H>>>>>>>>;
pub type Fb199 = Z;
pub type Wb199 = Pz<O<I<I<O<I<I<O<H>>>>>>>>;
pub type N199 = UFixed<Ib199, Fb199, Warm>;
pub const W199: usize = <Wb199 as Nat>::VAL;
const _: () = assert!(W199 == 182);
pub fn one_ok_199()
where
    N199: HasOne,
{
}
pub fn int_like_199()
where
    Fb199: IsZero,
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
pub type Sum97 = <N97 as AddNum<N100>>::Out;
pub const SW97: usize = <Sum97 as Stored>::W;
pub type Sum98 = <N98 as AddNum<N101>>::Out;
pub const SW98: usize = <Sum98 as Stored>::W;
pub type Sum99 = <N99 as AddNum<N102>>::Out;
pub const SW99: usize = <Sum99 as Stored>::W;
pub type Sum100 = <N100 as AddNum<N103>>::Out;
pub const SW100: usize = <Sum100 as Stored>::W;
pub type Sum101 = <N101 as AddNum<N104>>::Out;
pub const SW101: usize = <Sum101 as Stored>::W;
pub type Sum102 = <N102 as AddNum<N105>>::Out;
pub const SW102: usize = <Sum102 as Stored>::W;
pub type Sum103 = <N103 as AddNum<N106>>::Out;
pub const SW103: usize = <Sum103 as Stored>::W;
pub type Sum104 = <N104 as AddNum<N107>>::Out;
pub const SW104: usize = <Sum104 as Stored>::W;
pub type Sum105 = <N105 as AddNum<N108>>::Out;
pub const SW105: usize = <Sum105 as Stored>::W;
pub type Sum106 = <N106 as AddNum<N109>>::Out;
pub const SW106: usize = <Sum106 as Stored>::W;
pub type Sum107 = <N107 as AddNum<N110>>::Out;
pub const SW107: usize = <Sum107 as Stored>::W;
pub type Sum108 = <N108 as AddNum<N111>>::Out;
pub const SW108: usize = <Sum108 as Stored>::W;
pub type Sum109 = <N109 as AddNum<N112>>::Out;
pub const SW109: usize = <Sum109 as Stored>::W;
pub type Sum110 = <N110 as AddNum<N113>>::Out;
pub const SW110: usize = <Sum110 as Stored>::W;
pub type Sum111 = <N111 as AddNum<N114>>::Out;
pub const SW111: usize = <Sum111 as Stored>::W;
pub type Sum112 = <N112 as AddNum<N115>>::Out;
pub const SW112: usize = <Sum112 as Stored>::W;
pub type Sum113 = <N113 as AddNum<N116>>::Out;
pub const SW113: usize = <Sum113 as Stored>::W;
pub type Sum114 = <N114 as AddNum<N117>>::Out;
pub const SW114: usize = <Sum114 as Stored>::W;
pub type Sum115 = <N115 as AddNum<N118>>::Out;
pub const SW115: usize = <Sum115 as Stored>::W;
pub type Sum116 = <N116 as AddNum<N119>>::Out;
pub const SW116: usize = <Sum116 as Stored>::W;
pub type Sum117 = <N117 as AddNum<N120>>::Out;
pub const SW117: usize = <Sum117 as Stored>::W;
pub type Sum118 = <N118 as AddNum<N121>>::Out;
pub const SW118: usize = <Sum118 as Stored>::W;
pub type Sum119 = <N119 as AddNum<N122>>::Out;
pub const SW119: usize = <Sum119 as Stored>::W;
pub type Sum120 = <N120 as AddNum<N123>>::Out;
pub const SW120: usize = <Sum120 as Stored>::W;
pub type Sum121 = <N121 as AddNum<N124>>::Out;
pub const SW121: usize = <Sum121 as Stored>::W;
pub type Sum122 = <N122 as AddNum<N125>>::Out;
pub const SW122: usize = <Sum122 as Stored>::W;
pub type Sum123 = <N123 as AddNum<N126>>::Out;
pub const SW123: usize = <Sum123 as Stored>::W;
pub type Sum124 = <N124 as AddNum<N127>>::Out;
pub const SW124: usize = <Sum124 as Stored>::W;
pub type Sum125 = <N125 as AddNum<N128>>::Out;
pub const SW125: usize = <Sum125 as Stored>::W;
pub type Sum126 = <N126 as AddNum<N129>>::Out;
pub const SW126: usize = <Sum126 as Stored>::W;
pub type Sum127 = <N127 as AddNum<N130>>::Out;
pub const SW127: usize = <Sum127 as Stored>::W;
pub type Sum128 = <N128 as AddNum<N131>>::Out;
pub const SW128: usize = <Sum128 as Stored>::W;
pub type Sum129 = <N129 as AddNum<N132>>::Out;
pub const SW129: usize = <Sum129 as Stored>::W;
pub type Sum130 = <N130 as AddNum<N133>>::Out;
pub const SW130: usize = <Sum130 as Stored>::W;
pub type Sum131 = <N131 as AddNum<N134>>::Out;
pub const SW131: usize = <Sum131 as Stored>::W;
pub type Sum132 = <N132 as AddNum<N135>>::Out;
pub const SW132: usize = <Sum132 as Stored>::W;
pub type Sum133 = <N133 as AddNum<N136>>::Out;
pub const SW133: usize = <Sum133 as Stored>::W;
pub type Sum134 = <N134 as AddNum<N137>>::Out;
pub const SW134: usize = <Sum134 as Stored>::W;
pub type Sum135 = <N135 as AddNum<N138>>::Out;
pub const SW135: usize = <Sum135 as Stored>::W;
pub type Sum136 = <N136 as AddNum<N139>>::Out;
pub const SW136: usize = <Sum136 as Stored>::W;
pub type Sum137 = <N137 as AddNum<N140>>::Out;
pub const SW137: usize = <Sum137 as Stored>::W;
pub type Sum138 = <N138 as AddNum<N141>>::Out;
pub const SW138: usize = <Sum138 as Stored>::W;
pub type Sum139 = <N139 as AddNum<N142>>::Out;
pub const SW139: usize = <Sum139 as Stored>::W;
pub type Sum140 = <N140 as AddNum<N143>>::Out;
pub const SW140: usize = <Sum140 as Stored>::W;
pub type Sum141 = <N141 as AddNum<N144>>::Out;
pub const SW141: usize = <Sum141 as Stored>::W;
pub type Sum142 = <N142 as AddNum<N145>>::Out;
pub const SW142: usize = <Sum142 as Stored>::W;
pub type Sum143 = <N143 as AddNum<N146>>::Out;
pub const SW143: usize = <Sum143 as Stored>::W;
pub type Sum144 = <N144 as AddNum<N147>>::Out;
pub const SW144: usize = <Sum144 as Stored>::W;
pub type Sum145 = <N145 as AddNum<N148>>::Out;
pub const SW145: usize = <Sum145 as Stored>::W;
pub type Sum146 = <N146 as AddNum<N149>>::Out;
pub const SW146: usize = <Sum146 as Stored>::W;
pub type Sum147 = <N147 as AddNum<N150>>::Out;
pub const SW147: usize = <Sum147 as Stored>::W;
pub type Sum148 = <N148 as AddNum<N151>>::Out;
pub const SW148: usize = <Sum148 as Stored>::W;
pub type Sum149 = <N149 as AddNum<N152>>::Out;
pub const SW149: usize = <Sum149 as Stored>::W;
pub type Sum150 = <N150 as AddNum<N153>>::Out;
pub const SW150: usize = <Sum150 as Stored>::W;
pub type Sum151 = <N151 as AddNum<N154>>::Out;
pub const SW151: usize = <Sum151 as Stored>::W;
pub type Sum152 = <N152 as AddNum<N155>>::Out;
pub const SW152: usize = <Sum152 as Stored>::W;
pub type Sum153 = <N153 as AddNum<N156>>::Out;
pub const SW153: usize = <Sum153 as Stored>::W;
pub type Sum154 = <N154 as AddNum<N157>>::Out;
pub const SW154: usize = <Sum154 as Stored>::W;
pub type Sum155 = <N155 as AddNum<N158>>::Out;
pub const SW155: usize = <Sum155 as Stored>::W;
pub type Sum156 = <N156 as AddNum<N159>>::Out;
pub const SW156: usize = <Sum156 as Stored>::W;
pub type Sum157 = <N157 as AddNum<N160>>::Out;
pub const SW157: usize = <Sum157 as Stored>::W;
pub type Sum158 = <N158 as AddNum<N161>>::Out;
pub const SW158: usize = <Sum158 as Stored>::W;
pub type Sum159 = <N159 as AddNum<N162>>::Out;
pub const SW159: usize = <Sum159 as Stored>::W;
pub type Sum160 = <N160 as AddNum<N163>>::Out;
pub const SW160: usize = <Sum160 as Stored>::W;
pub type Sum161 = <N161 as AddNum<N164>>::Out;
pub const SW161: usize = <Sum161 as Stored>::W;
pub type Sum162 = <N162 as AddNum<N165>>::Out;
pub const SW162: usize = <Sum162 as Stored>::W;
pub type Sum163 = <N163 as AddNum<N166>>::Out;
pub const SW163: usize = <Sum163 as Stored>::W;
pub type Sum164 = <N164 as AddNum<N167>>::Out;
pub const SW164: usize = <Sum164 as Stored>::W;
pub type Sum165 = <N165 as AddNum<N168>>::Out;
pub const SW165: usize = <Sum165 as Stored>::W;
pub type Sum166 = <N166 as AddNum<N169>>::Out;
pub const SW166: usize = <Sum166 as Stored>::W;
pub type Sum167 = <N167 as AddNum<N170>>::Out;
pub const SW167: usize = <Sum167 as Stored>::W;
pub type Sum168 = <N168 as AddNum<N171>>::Out;
pub const SW168: usize = <Sum168 as Stored>::W;
pub type Sum169 = <N169 as AddNum<N172>>::Out;
pub const SW169: usize = <Sum169 as Stored>::W;
pub type Sum170 = <N170 as AddNum<N173>>::Out;
pub const SW170: usize = <Sum170 as Stored>::W;
pub type Sum171 = <N171 as AddNum<N174>>::Out;
pub const SW171: usize = <Sum171 as Stored>::W;
pub type Sum172 = <N172 as AddNum<N175>>::Out;
pub const SW172: usize = <Sum172 as Stored>::W;
pub type Sum173 = <N173 as AddNum<N176>>::Out;
pub const SW173: usize = <Sum173 as Stored>::W;
pub type Sum174 = <N174 as AddNum<N177>>::Out;
pub const SW174: usize = <Sum174 as Stored>::W;
pub type Sum175 = <N175 as AddNum<N178>>::Out;
pub const SW175: usize = <Sum175 as Stored>::W;
pub type Sum176 = <N176 as AddNum<N179>>::Out;
pub const SW176: usize = <Sum176 as Stored>::W;
pub type Sum177 = <N177 as AddNum<N180>>::Out;
pub const SW177: usize = <Sum177 as Stored>::W;
pub type Sum178 = <N178 as AddNum<N181>>::Out;
pub const SW178: usize = <Sum178 as Stored>::W;
pub type Sum179 = <N179 as AddNum<N182>>::Out;
pub const SW179: usize = <Sum179 as Stored>::W;
pub type Sum180 = <N180 as AddNum<N183>>::Out;
pub const SW180: usize = <Sum180 as Stored>::W;
pub type Sum181 = <N181 as AddNum<N184>>::Out;
pub const SW181: usize = <Sum181 as Stored>::W;
pub type Sum182 = <N182 as AddNum<N185>>::Out;
pub const SW182: usize = <Sum182 as Stored>::W;
pub type Sum183 = <N183 as AddNum<N186>>::Out;
pub const SW183: usize = <Sum183 as Stored>::W;
pub type Sum184 = <N184 as AddNum<N187>>::Out;
pub const SW184: usize = <Sum184 as Stored>::W;
pub type Sum185 = <N185 as AddNum<N188>>::Out;
pub const SW185: usize = <Sum185 as Stored>::W;
pub type Sum186 = <N186 as AddNum<N189>>::Out;
pub const SW186: usize = <Sum186 as Stored>::W;
pub type Sum187 = <N187 as AddNum<N190>>::Out;
pub const SW187: usize = <Sum187 as Stored>::W;
pub type Sum188 = <N188 as AddNum<N191>>::Out;
pub const SW188: usize = <Sum188 as Stored>::W;
pub type Sum189 = <N189 as AddNum<N192>>::Out;
pub const SW189: usize = <Sum189 as Stored>::W;
pub type Sum190 = <N190 as AddNum<N193>>::Out;
pub const SW190: usize = <Sum190 as Stored>::W;
pub type Sum191 = <N191 as AddNum<N194>>::Out;
pub const SW191: usize = <Sum191 as Stored>::W;
pub type Sum192 = <N192 as AddNum<N195>>::Out;
pub const SW192: usize = <Sum192 as Stored>::W;
pub type Sum193 = <N193 as AddNum<N196>>::Out;
pub const SW193: usize = <Sum193 as Stored>::W;
pub type Sum194 = <N194 as AddNum<N197>>::Out;
pub const SW194: usize = <Sum194 as Stored>::W;
pub type Sum195 = <N195 as AddNum<N198>>::Out;
pub const SW195: usize = <Sum195 as Stored>::W;
pub type Sum196 = <N196 as AddNum<N199>>::Out;
pub const SW196: usize = <Sum196 as Stored>::W;
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
pub type C100 = Slot<Pz<I<I<I<I<O<I<H>>>>>>>, 111>;
pub fn build100() -> <C100 as Capacity>::Array<u32> {
    C100::build(0)
}
pub type C101 = Slot<Pz<I<O<O<O<I<I<H>>>>>>>, 113>;
pub fn build101() -> <C101 as Capacity>::Array<u32> {
    C101::build(0)
}
pub type C102 = Slot<Pz<O<I<O<O<I<I<H>>>>>>>, 114>;
pub fn build102() -> <C102 as Capacity>::Array<u32> {
    C102::build(0)
}
pub type C103 = Slot<Pz<I<O<I<O<I<I<H>>>>>>>, 117>;
pub fn build103() -> <C103 as Capacity>::Array<u32> {
    C103::build(0)
}
pub type C104 = Slot<Pz<O<I<I<O<I<I<H>>>>>>>, 118>;
pub fn build104() -> <C104 as Capacity>::Array<u32> {
    C104::build(0)
}
pub type C105 = Slot<Pz<I<I<O<I<I<I<H>>>>>>>, 123>;
pub fn build105() -> <C105 as Capacity>::Array<u32> {
    C105::build(0)
}
pub type C106 = Slot<Pz<O<I<I<I<I<I<H>>>>>>>, 126>;
pub fn build106() -> <C106 as Capacity>::Array<u32> {
    C106::build(0)
}
pub type C107 = Slot<Pz<O<I<O<I<O<O<O<H>>>>>>>>, 138>;
pub fn build107() -> <C107 as Capacity>::Array<u32> {
    C107::build(0)
}
pub type C108 = Slot<Pz<O<I<I<I<O<O<O<H>>>>>>>>, 142>;
pub fn build108() -> <C108 as Capacity>::Array<u32> {
    C108::build(0)
}
pub type C109 = Slot<Pz<O<I<I<I<O<I<O<H>>>>>>>>, 174>;
pub fn build109() -> <C109 as Capacity>::Array<u32> {
    C109::build(0)
}
pub type C110 = Slot<Pz<O<I<O<I<I<I<H>>>>>>>, 122>;
pub fn build110() -> <C110 as Capacity>::Array<u32> {
    C110::build(0)
}
pub type C111 = Slot<Pz<O<O<I<I<I<I<H>>>>>>>, 124>;
pub fn build111() -> <C111 as Capacity>::Array<u32> {
    C111::build(0)
}
pub type C112 = Slot<Pz<I<O<I<I<I<I<H>>>>>>>, 125>;
pub fn build112() -> <C112 as Capacity>::Array<u32> {
    C112::build(0)
}
pub type C113 = Slot<Pz<O<O<O<O<O<O<O<H>>>>>>>>, 128>;
pub fn build113() -> <C113 as Capacity>::Array<u32> {
    C113::build(0)
}
pub type C114 = Slot<Pz<I<O<O<O<O<O<O<H>>>>>>>>, 129>;
pub fn build114() -> <C114 as Capacity>::Array<u32> {
    C114::build(0)
}
pub type C115 = Slot<Pz<O<I<I<O<O<O<O<H>>>>>>>>, 134>;
pub fn build115() -> <C115 as Capacity>::Array<u32> {
    C115::build(0)
}
pub type C116 = Slot<Pz<I<O<O<I<O<O<O<H>>>>>>>>, 137>;
pub fn build116() -> <C116 as Capacity>::Array<u32> {
    C116::build(0)
}
pub type C117 = Slot<Pz<I<O<I<O<I<O<O<H>>>>>>>>, 149>;
pub fn build117() -> <C117 as Capacity>::Array<u32> {
    C117::build(0)
}
pub type C118 = Slot<Pz<I<O<O<I<I<O<O<H>>>>>>>>, 153>;
pub fn build118() -> <C118 as Capacity>::Array<u32> {
    C118::build(0)
}
pub type C119 = Slot<Pz<I<O<O<I<I<I<O<H>>>>>>>>, 185>;
pub fn build119() -> <C119 as Capacity>::Array<u32> {
    C119::build(0)
}
pub type C120 = Slot<Pz<I<O<I<O<O<O<O<H>>>>>>>>, 133>;
pub fn build120() -> <C120 as Capacity>::Array<u32> {
    C120::build(0)
}
pub type C121 = Slot<Pz<I<I<I<O<O<O<O<H>>>>>>>>, 135>;
pub fn build121() -> <C121 as Capacity>::Array<u32> {
    C121::build(0)
}
pub type C122 = Slot<Pz<O<O<O<I<O<O<O<H>>>>>>>>, 136>;
pub fn build122() -> <C122 as Capacity>::Array<u32> {
    C122::build(0)
}
pub type C123 = Slot<Pz<I<I<O<I<O<O<O<H>>>>>>>>, 139>;
pub fn build123() -> <C123 as Capacity>::Array<u32> {
    C123::build(0)
}
pub type C124 = Slot<Pz<O<O<I<I<O<O<O<H>>>>>>>>, 140>;
pub fn build124() -> <C124 as Capacity>::Array<u32> {
    C124::build(0)
}
pub type C125 = Slot<Pz<I<O<O<O<I<O<O<H>>>>>>>>, 145>;
pub fn build125() -> <C125 as Capacity>::Array<u32> {
    C125::build(0)
}
pub type C126 = Slot<Pz<O<O<I<O<I<O<O<H>>>>>>>>, 148>;
pub fn build126() -> <C126 as Capacity>::Array<u32> {
    C126::build(0)
}
pub type C127 = Slot<Pz<O<O<O<O<O<I<O<H>>>>>>>>, 160>;
pub fn build127() -> <C127 as Capacity>::Array<u32> {
    C127::build(0)
}
pub type C128 = Slot<Pz<O<O<I<O<O<I<O<H>>>>>>>>, 164>;
pub fn build128() -> <C128 as Capacity>::Array<u32> {
    C128::build(0)
}
pub type C129 = Slot<Pz<O<O<I<O<O<O<I<H>>>>>>>>, 196>;
pub fn build129() -> <C129 as Capacity>::Array<u32> {
    C129::build(0)
}
pub type C130 = Slot<Pz<O<O<O<O<I<O<O<H>>>>>>>>, 144>;
pub fn build130() -> <C130 as Capacity>::Array<u32> {
    C130::build(0)
}
pub type C131 = Slot<Pz<O<I<O<O<I<O<O<H>>>>>>>>, 146>;
pub fn build131() -> <C131 as Capacity>::Array<u32> {
    C131::build(0)
}
pub type C132 = Slot<Pz<I<I<O<O<I<O<O<H>>>>>>>>, 147>;
pub fn build132() -> <C132 as Capacity>::Array<u32> {
    C132::build(0)
}
pub type C133 = Slot<Pz<O<I<I<O<I<O<O<H>>>>>>>>, 150>;
pub fn build133() -> <C133 as Capacity>::Array<u32> {
    C133::build(0)
}
pub type C134 = Slot<Pz<I<I<I<O<I<O<O<H>>>>>>>>, 151>;
pub fn build134() -> <C134 as Capacity>::Array<u32> {
    C134::build(0)
}
pub type C135 = Slot<Pz<O<O<I<I<I<O<O<H>>>>>>>>, 156>;
pub fn build135() -> <C135 as Capacity>::Array<u32> {
    C135::build(0)
}
pub type C136 = Slot<Pz<I<I<I<I<I<O<O<H>>>>>>>>, 159>;
pub fn build136() -> <C136 as Capacity>::Array<u32> {
    C136::build(0)
}
pub type C137 = Slot<Pz<I<I<O<I<O<I<O<H>>>>>>>>, 171>;
pub fn build137() -> <C137 as Capacity>::Array<u32> {
    C137::build(0)
}
pub type C138 = Slot<Pz<I<I<I<I<O<I<O<H>>>>>>>>, 175>;
pub fn build138() -> <C138 as Capacity>::Array<u32> {
    C138::build(0)
}
pub type C139 = Slot<Pz<I<I<I<I<O<O<I<H>>>>>>>>, 207>;
pub fn build139() -> <C139 as Capacity>::Array<u32> {
    C139::build(0)
}
pub type C140 = Slot<Pz<I<I<O<I<I<O<O<H>>>>>>>>, 155>;
pub fn build140() -> <C140 as Capacity>::Array<u32> {
    C140::build(0)
}
pub type C141 = Slot<Pz<I<O<I<I<I<O<O<H>>>>>>>>, 157>;
pub fn build141() -> <C141 as Capacity>::Array<u32> {
    C141::build(0)
}
pub type C142 = Slot<Pz<O<I<I<I<I<O<O<H>>>>>>>>, 158>;
pub fn build142() -> <C142 as Capacity>::Array<u32> {
    C142::build(0)
}
pub type C143 = Slot<Pz<I<O<O<O<O<I<O<H>>>>>>>>, 161>;
pub fn build143() -> <C143 as Capacity>::Array<u32> {
    C143::build(0)
}
pub type C144 = Slot<Pz<O<I<O<O<O<I<O<H>>>>>>>>, 162>;
pub fn build144() -> <C144 as Capacity>::Array<u32> {
    C144::build(0)
}
pub type C145 = Slot<Pz<I<I<I<O<O<I<O<H>>>>>>>>, 167>;
pub fn build145() -> <C145 as Capacity>::Array<u32> {
    C145::build(0)
}
pub type C146 = Slot<Pz<O<I<O<I<O<I<O<H>>>>>>>>, 170>;
pub fn build146() -> <C146 as Capacity>::Array<u32> {
    C146::build(0)
}
pub type C147 = Slot<Pz<O<I<I<O<I<I<O<H>>>>>>>>, 182>;
pub fn build147() -> <C147 as Capacity>::Array<u32> {
    C147::build(0)
}
pub type C148 = Slot<Pz<O<I<O<I<I<I<O<H>>>>>>>>, 186>;
pub fn build148() -> <C148 as Capacity>::Array<u32> {
    C148::build(0)
}
pub type C149 = Slot<Pz<O<I<O<I<I<O<I<H>>>>>>>>, 218>;
pub fn build149() -> <C149 as Capacity>::Array<u32> {
    C149::build(0)
}
pub type C150 = Slot<Pz<O<I<I<O<O<I<O<H>>>>>>>>, 166>;
pub fn build150() -> <C150 as Capacity>::Array<u32> {
    C150::build(0)
}
pub type C151 = Slot<Pz<O<O<O<I<O<I<O<H>>>>>>>>, 168>;
pub fn build151() -> <C151 as Capacity>::Array<u32> {
    C151::build(0)
}
pub type C152 = Slot<Pz<I<O<O<I<O<I<O<H>>>>>>>>, 169>;
pub fn build152() -> <C152 as Capacity>::Array<u32> {
    C152::build(0)
}
pub type C153 = Slot<Pz<O<O<I<I<O<I<O<H>>>>>>>>, 172>;
pub fn build153() -> <C153 as Capacity>::Array<u32> {
    C153::build(0)
}
pub type C154 = Slot<Pz<I<O<I<I<O<I<O<H>>>>>>>>, 173>;
pub fn build154() -> <C154 as Capacity>::Array<u32> {
    C154::build(0)
}
pub type C155 = Slot<Pz<O<I<O<O<I<I<O<H>>>>>>>>, 178>;
pub fn build155() -> <C155 as Capacity>::Array<u32> {
    C155::build(0)
}
pub type C156 = Slot<Pz<I<O<I<O<I<I<O<H>>>>>>>>, 181>;
pub fn build156() -> <C156 as Capacity>::Array<u32> {
    C156::build(0)
}
pub type C157 = Slot<Pz<I<O<O<O<O<O<I<H>>>>>>>>, 193>;
pub fn build157() -> <C157 as Capacity>::Array<u32> {
    C157::build(0)
}
pub type C158 = Slot<Pz<I<O<I<O<O<O<I<H>>>>>>>>, 197>;
pub fn build158() -> <C158 as Capacity>::Array<u32> {
    C158::build(0)
}
pub type C159 = Slot<Pz<I<O<I<O<O<I<I<H>>>>>>>>, 229>;
pub fn build159() -> <C159 as Capacity>::Array<u32> {
    C159::build(0)
}
pub type C160 = Slot<Pz<I<O<O<O<I<I<O<H>>>>>>>>, 177>;
pub fn build160() -> <C160 as Capacity>::Array<u32> {
    C160::build(0)
}
pub type C161 = Slot<Pz<I<I<O<O<I<I<O<H>>>>>>>>, 179>;
pub fn build161() -> <C161 as Capacity>::Array<u32> {
    C161::build(0)
}
pub type C162 = Slot<Pz<O<O<I<O<I<I<O<H>>>>>>>>, 180>;
pub fn build162() -> <C162 as Capacity>::Array<u32> {
    C162::build(0)
}
pub type C163 = Slot<Pz<I<I<I<O<I<I<O<H>>>>>>>>, 183>;
pub fn build163() -> <C163 as Capacity>::Array<u32> {
    C163::build(0)
}
pub type C164 = Slot<Pz<O<O<O<I<I<I<O<H>>>>>>>>, 184>;
pub fn build164() -> <C164 as Capacity>::Array<u32> {
    C164::build(0)
}
pub type C165 = Slot<Pz<I<O<I<I<I<I<O<H>>>>>>>>, 189>;
pub fn build165() -> <C165 as Capacity>::Array<u32> {
    C165::build(0)
}
pub type C166 = Slot<Pz<O<O<O<O<O<O<I<H>>>>>>>>, 192>;
pub fn build166() -> <C166 as Capacity>::Array<u32> {
    C166::build(0)
}
pub type C167 = Slot<Pz<O<O<I<I<O<O<I<H>>>>>>>>, 204>;
pub fn build167() -> <C167 as Capacity>::Array<u32> {
    C167::build(0)
}
pub type C168 = Slot<Pz<O<O<O<O<I<O<I<H>>>>>>>>, 208>;
pub fn build168() -> <C168 as Capacity>::Array<u32> {
    C168::build(0)
}
pub type C169 = Slot<Pz<O<O<O<O<I<I<I<H>>>>>>>>, 240>;
pub fn build169() -> <C169 as Capacity>::Array<u32> {
    C169::build(0)
}
pub type C170 = Slot<Pz<O<O<I<I<I<I<O<H>>>>>>>>, 188>;
pub fn build170() -> <C170 as Capacity>::Array<u32> {
    C170::build(0)
}
pub type C171 = Slot<Pz<O<I<I<I<I<I<O<H>>>>>>>>, 190>;
pub fn build171() -> <C171 as Capacity>::Array<u32> {
    C171::build(0)
}
pub type C172 = Slot<Pz<I<I<I<I<I<I<O<H>>>>>>>>, 191>;
pub fn build172() -> <C172 as Capacity>::Array<u32> {
    C172::build(0)
}
pub type C173 = Slot<Pz<O<I<O<O<O<O<I<H>>>>>>>>, 194>;
pub fn build173() -> <C173 as Capacity>::Array<u32> {
    C173::build(0)
}
pub type C174 = Slot<Pz<I<I<O<O<O<O<I<H>>>>>>>>, 195>;
pub fn build174() -> <C174 as Capacity>::Array<u32> {
    C174::build(0)
}
pub type C175 = Slot<Pz<O<O<O<I<O<O<I<H>>>>>>>>, 200>;
pub fn build175() -> <C175 as Capacity>::Array<u32> {
    C175::build(0)
}
pub type C176 = Slot<Pz<I<I<O<I<O<O<I<H>>>>>>>>, 203>;
pub fn build176() -> <C176 as Capacity>::Array<u32> {
    C176::build(0)
}
pub type C177 = Slot<Pz<I<I<I<O<I<O<I<H>>>>>>>>, 215>;
pub fn build177() -> <C177 as Capacity>::Array<u32> {
    C177::build(0)
}
pub type C178 = Slot<Pz<I<I<O<I<I<O<I<H>>>>>>>>, 219>;
pub fn build178() -> <C178 as Capacity>::Array<u32> {
    C178::build(0)
}
pub type C179 = Slot<Pz<I<I<O<I<I<I<I<H>>>>>>>>, 251>;
pub fn build179() -> <C179 as Capacity>::Array<u32> {
    C179::build(0)
}
pub type C180 = Slot<Pz<I<I<I<O<O<O<I<H>>>>>>>>, 199>;
pub fn build180() -> <C180 as Capacity>::Array<u32> {
    C180::build(0)
}
pub type C181 = Slot<Pz<I<O<O<I<O<O<I<H>>>>>>>>, 201>;
pub fn build181() -> <C181 as Capacity>::Array<u32> {
    C181::build(0)
}
pub type C182 = Slot<Pz<O<I<O<I<O<O<I<H>>>>>>>>, 202>;
pub fn build182() -> <C182 as Capacity>::Array<u32> {
    C182::build(0)
}
pub type C183 = Slot<Pz<I<O<I<I<O<O<I<H>>>>>>>>, 205>;
pub fn build183() -> <C183 as Capacity>::Array<u32> {
    C183::build(0)
}
pub type C184 = Slot<Pz<O<I<I<I<O<O<I<H>>>>>>>>, 206>;
pub fn build184() -> <C184 as Capacity>::Array<u32> {
    C184::build(0)
}
pub type C185 = Slot<Pz<I<I<O<O<I<O<I<H>>>>>>>>, 211>;
pub fn build185() -> <C185 as Capacity>::Array<u32> {
    C185::build(0)
}
pub type C186 = Slot<Pz<O<I<I<O<I<O<I<H>>>>>>>>, 214>;
pub fn build186() -> <C186 as Capacity>::Array<u32> {
    C186::build(0)
}
pub type C187 = Slot<Pz<O<I<O<O<O<I<I<H>>>>>>>>, 226>;
pub fn build187() -> <C187 as Capacity>::Array<u32> {
    C187::build(0)
}
pub type C188 = Slot<Pz<O<I<I<O<O<I<I<H>>>>>>>>, 230>;
pub fn build188() -> <C188 as Capacity>::Array<u32> {
    C188::build(0)
}
pub type C189 = Slot<Pz<O<I<I<O<O<O<O<O<H>>>>>>>>>, 262>;
pub fn build189() -> <C189 as Capacity>::Array<u32> {
    C189::build(0)
}
pub type C190 = Slot<Pz<O<I<O<O<I<O<I<H>>>>>>>>, 210>;
pub fn build190() -> <C190 as Capacity>::Array<u32> {
    C190::build(0)
}
pub type C191 = Slot<Pz<O<O<I<O<I<O<I<H>>>>>>>>, 212>;
pub fn build191() -> <C191 as Capacity>::Array<u32> {
    C191::build(0)
}
pub type C192 = Slot<Pz<I<O<I<O<I<O<I<H>>>>>>>>, 213>;
pub fn build192() -> <C192 as Capacity>::Array<u32> {
    C192::build(0)
}
pub type C193 = Slot<Pz<O<O<O<I<I<O<I<H>>>>>>>>, 216>;
pub fn build193() -> <C193 as Capacity>::Array<u32> {
    C193::build(0)
}
pub type C194 = Slot<Pz<I<O<O<I<I<O<I<H>>>>>>>>, 217>;
pub fn build194() -> <C194 as Capacity>::Array<u32> {
    C194::build(0)
}
pub type C195 = Slot<Pz<O<I<I<I<I<O<I<H>>>>>>>>, 222>;
pub fn build195() -> <C195 as Capacity>::Array<u32> {
    C195::build(0)
}
pub type C196 = Slot<Pz<I<O<O<O<O<I<I<H>>>>>>>>, 225>;
pub fn build196() -> <C196 as Capacity>::Array<u32> {
    C196::build(0)
}
pub type C197 = Slot<Pz<I<O<I<I<O<I<I<H>>>>>>>>, 237>;
pub fn build197() -> <C197 as Capacity>::Array<u32> {
    C197::build(0)
}
pub type C198 = Slot<Pz<I<O<O<O<I<I<I<H>>>>>>>>, 241>;
pub fn build198() -> <C198 as Capacity>::Array<u32> {
    C198::build(0)
}
pub type C199 = Slot<Pz<I<O<O<O<I<O<O<O<H>>>>>>>>>, 273>;
pub fn build199() -> <C199 as Capacity>::Array<u32> {
    C199::build(0)
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
pub fn call100() -> u32 {
    scaled_fold::<Ib100, Fb100, Warm, C100>(100)
}
pub fn call101() -> u32 {
    scaled_fold::<Ib101, Fb101, Cold, C101>(101)
}
pub fn call102() -> u32 {
    scaled_fold::<Ib102, Fb102, Hot, C102>(102)
}
pub fn call103() -> u32 {
    scaled_fold::<Ib103, Fb103, Warm, C103>(103)
}
pub fn call104() -> u32 {
    scaled_fold::<Ib104, Fb104, Cold, C104>(104)
}
pub fn call106() -> u32 {
    scaled_fold::<Ib106, Fb106, Warm, C106>(106)
}
pub fn call107() -> u32 {
    scaled_fold::<Ib107, Fb107, Cold, C107>(107)
}
pub fn call108() -> u32 {
    scaled_fold::<Ib108, Fb108, Hot, C108>(108)
}
pub fn call109() -> u32 {
    scaled_fold::<Ib109, Fb109, Warm, C109>(109)
}
pub fn call110() -> u32 {
    scaled_fold::<Ib110, Fb110, Cold, C110>(110)
}
pub fn call111() -> u32 {
    scaled_fold::<Ib111, Fb111, Hot, C111>(111)
}
pub fn call112() -> u32 {
    scaled_fold::<Ib112, Fb112, Warm, C112>(112)
}
pub fn call114() -> u32 {
    scaled_fold::<Ib114, Fb114, Hot, C114>(114)
}
pub fn call115() -> u32 {
    scaled_fold::<Ib115, Fb115, Warm, C115>(115)
}
pub fn call116() -> u32 {
    scaled_fold::<Ib116, Fb116, Cold, C116>(116)
}
pub fn call117() -> u32 {
    scaled_fold::<Ib117, Fb117, Hot, C117>(117)
}
pub fn call118() -> u32 {
    scaled_fold::<Ib118, Fb118, Warm, C118>(118)
}
pub fn call119() -> u32 {
    scaled_fold::<Ib119, Fb119, Cold, C119>(119)
}
pub fn call120() -> u32 {
    scaled_fold::<Ib120, Fb120, Hot, C120>(120)
}
pub fn call122() -> u32 {
    scaled_fold::<Ib122, Fb122, Cold, C122>(122)
}
pub fn call123() -> u32 {
    scaled_fold::<Ib123, Fb123, Hot, C123>(123)
}
pub fn call124() -> u32 {
    scaled_fold::<Ib124, Fb124, Warm, C124>(124)
}
pub fn call125() -> u32 {
    scaled_fold::<Ib125, Fb125, Cold, C125>(125)
}
pub fn call126() -> u32 {
    scaled_fold::<Ib126, Fb126, Hot, C126>(126)
}
pub fn call127() -> u32 {
    scaled_fold::<Ib127, Fb127, Warm, C127>(127)
}
pub fn call128() -> u32 {
    scaled_fold::<Ib128, Fb128, Cold, C128>(128)
}
pub fn call130() -> u32 {
    scaled_fold::<Ib130, Fb130, Warm, C130>(130)
}
pub fn call131() -> u32 {
    scaled_fold::<Ib131, Fb131, Cold, C131>(131)
}
pub fn call132() -> u32 {
    scaled_fold::<Ib132, Fb132, Hot, C132>(132)
}
pub fn call133() -> u32 {
    scaled_fold::<Ib133, Fb133, Warm, C133>(133)
}
pub fn call134() -> u32 {
    scaled_fold::<Ib134, Fb134, Cold, C134>(134)
}
pub fn call135() -> u32 {
    scaled_fold::<Ib135, Fb135, Hot, C135>(135)
}
pub fn call136() -> u32 {
    scaled_fold::<Ib136, Fb136, Warm, C136>(136)
}
pub fn call138() -> u32 {
    scaled_fold::<Ib138, Fb138, Hot, C138>(138)
}
pub fn call139() -> u32 {
    scaled_fold::<Ib139, Fb139, Warm, C139>(139)
}
pub fn call140() -> u32 {
    scaled_fold::<Ib140, Fb140, Cold, C140>(140)
}
pub fn call141() -> u32 {
    scaled_fold::<Ib141, Fb141, Hot, C141>(141)
}
pub fn call142() -> u32 {
    scaled_fold::<Ib142, Fb142, Warm, C142>(142)
}
pub fn call143() -> u32 {
    scaled_fold::<Ib143, Fb143, Cold, C143>(143)
}
pub fn call144() -> u32 {
    scaled_fold::<Ib144, Fb144, Hot, C144>(144)
}
pub fn call146() -> u32 {
    scaled_fold::<Ib146, Fb146, Cold, C146>(146)
}
pub fn call147() -> u32 {
    scaled_fold::<Ib147, Fb147, Hot, C147>(147)
}
pub fn call148() -> u32 {
    scaled_fold::<Ib148, Fb148, Warm, C148>(148)
}
pub fn call149() -> u32 {
    scaled_fold::<Ib149, Fb149, Cold, C149>(149)
}
pub fn call150() -> u32 {
    scaled_fold::<Ib150, Fb150, Hot, C150>(150)
}
pub fn call151() -> u32 {
    scaled_fold::<Ib151, Fb151, Warm, C151>(151)
}
pub fn call152() -> u32 {
    scaled_fold::<Ib152, Fb152, Cold, C152>(152)
}
pub fn call154() -> u32 {
    scaled_fold::<Ib154, Fb154, Warm, C154>(154)
}
pub fn call155() -> u32 {
    scaled_fold::<Ib155, Fb155, Cold, C155>(155)
}
pub fn call156() -> u32 {
    scaled_fold::<Ib156, Fb156, Hot, C156>(156)
}
pub fn call157() -> u32 {
    scaled_fold::<Ib157, Fb157, Warm, C157>(157)
}
pub fn call158() -> u32 {
    scaled_fold::<Ib158, Fb158, Cold, C158>(158)
}
pub fn call159() -> u32 {
    scaled_fold::<Ib159, Fb159, Hot, C159>(159)
}
pub fn call160() -> u32 {
    scaled_fold::<Ib160, Fb160, Warm, C160>(160)
}
pub fn call162() -> u32 {
    scaled_fold::<Ib162, Fb162, Hot, C162>(162)
}
pub fn call163() -> u32 {
    scaled_fold::<Ib163, Fb163, Warm, C163>(163)
}
pub fn call164() -> u32 {
    scaled_fold::<Ib164, Fb164, Cold, C164>(164)
}
pub fn call165() -> u32 {
    scaled_fold::<Ib165, Fb165, Hot, C165>(165)
}
pub fn call166() -> u32 {
    scaled_fold::<Ib166, Fb166, Warm, C166>(166)
}
pub fn call167() -> u32 {
    scaled_fold::<Ib167, Fb167, Cold, C167>(167)
}
pub fn call168() -> u32 {
    scaled_fold::<Ib168, Fb168, Hot, C168>(168)
}
pub fn call170() -> u32 {
    scaled_fold::<Ib170, Fb170, Cold, C170>(170)
}
pub fn call171() -> u32 {
    scaled_fold::<Ib171, Fb171, Hot, C171>(171)
}
pub fn call172() -> u32 {
    scaled_fold::<Ib172, Fb172, Warm, C172>(172)
}
pub fn call173() -> u32 {
    scaled_fold::<Ib173, Fb173, Cold, C173>(173)
}
pub fn call174() -> u32 {
    scaled_fold::<Ib174, Fb174, Hot, C174>(174)
}
pub fn call175() -> u32 {
    scaled_fold::<Ib175, Fb175, Warm, C175>(175)
}
pub fn call176() -> u32 {
    scaled_fold::<Ib176, Fb176, Cold, C176>(176)
}
pub fn call178() -> u32 {
    scaled_fold::<Ib178, Fb178, Warm, C178>(178)
}
pub fn call179() -> u32 {
    scaled_fold::<Ib179, Fb179, Cold, C179>(179)
}
pub fn call180() -> u32 {
    scaled_fold::<Ib180, Fb180, Hot, C180>(180)
}
pub fn call181() -> u32 {
    scaled_fold::<Ib181, Fb181, Warm, C181>(181)
}
pub fn call182() -> u32 {
    scaled_fold::<Ib182, Fb182, Cold, C182>(182)
}
pub fn call183() -> u32 {
    scaled_fold::<Ib183, Fb183, Hot, C183>(183)
}
pub fn call184() -> u32 {
    scaled_fold::<Ib184, Fb184, Warm, C184>(184)
}
pub fn call186() -> u32 {
    scaled_fold::<Ib186, Fb186, Hot, C186>(186)
}
pub fn call187() -> u32 {
    scaled_fold::<Ib187, Fb187, Warm, C187>(187)
}
pub fn call188() -> u32 {
    scaled_fold::<Ib188, Fb188, Cold, C188>(188)
}
pub fn call189() -> u32 {
    scaled_fold::<Ib189, Fb189, Hot, C189>(189)
}
pub fn call190() -> u32 {
    scaled_fold::<Ib190, Fb190, Warm, C190>(190)
}
pub fn call191() -> u32 {
    scaled_fold::<Ib191, Fb191, Cold, C191>(191)
}
pub fn call192() -> u32 {
    scaled_fold::<Ib192, Fb192, Hot, C192>(192)
}
pub fn call194() -> u32 {
    scaled_fold::<Ib194, Fb194, Cold, C194>(194)
}
pub fn call195() -> u32 {
    scaled_fold::<Ib195, Fb195, Hot, C195>(195)
}
pub fn call196() -> u32 {
    scaled_fold::<Ib196, Fb196, Warm, C196>(196)
}
pub fn call197() -> u32 {
    scaled_fold::<Ib197, Fb197, Cold, C197>(197)
}
pub fn call198() -> u32 {
    scaled_fold::<Ib198, Fb198, Hot, C198>(198)
}
pub fn call199() -> u32 {
    scaled_fold::<Ib199, Fb199, Warm, C199>(199)
}
