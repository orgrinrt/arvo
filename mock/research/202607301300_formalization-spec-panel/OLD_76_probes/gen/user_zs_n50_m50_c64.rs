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
