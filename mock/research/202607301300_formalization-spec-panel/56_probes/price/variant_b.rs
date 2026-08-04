#![feature(adt_const_params)]
#![allow(dead_code)]
use core::marker::ConstParamTy;
use core::marker::PhantomData;
const fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}
#[derive(PartialEq, Eq, ConstParamTy)]
pub struct Spec {
    pub precision: u16,
    pub bias_num: u64,
    pub bias_den: u64,
}
impl Spec {
    const fn is_reduced(&self) -> bool {
        gcd(self.bias_num, self.bias_den) == 1
    }
}
pub struct NFace<const S: Spec>(PhantomData<()>);
impl<const S: Spec> NFace<S> {
    pub const REDUCED: () = assert!(S.is_reduced());
    pub const fn checked() -> Self {
        let () = Self::REDUCED;
        NFace(PhantomData)
    }
}

pub const SPEC_0: Spec = Spec {
    precision: 44,
    bias_num: 93,
    bias_den: 98,
};
pub fn mk_0() -> NFace<SPEC_0> {
    NFace::<SPEC_0>::checked()
}
pub const SPEC_1: Spec = Spec {
    precision: 12,
    bias_num: 35,
    bias_den: 141,
};
pub fn mk_1() -> NFace<SPEC_1> {
    NFace::<SPEC_1>::checked()
}
pub const SPEC_2: Spec = Spec {
    precision: 32,
    bias_num: 377,
    bias_den: 328,
};
pub fn mk_2() -> NFace<SPEC_2> {
    NFace::<SPEC_2>::checked()
}
pub const SPEC_3: Spec = Spec {
    precision: 24,
    bias_num: 3,
    bias_den: 337,
};
pub fn mk_3() -> NFace<SPEC_3> {
    NFace::<SPEC_3>::checked()
}
pub const SPEC_4: Spec = Spec {
    precision: 44,
    bias_num: 18,
    bias_den: 299,
};
pub fn mk_4() -> NFace<SPEC_4> {
    NFace::<SPEC_4>::checked()
}
pub const SPEC_5: Spec = Spec {
    precision: 24,
    bias_num: 1,
    bias_den: 448,
};
pub fn mk_5() -> NFace<SPEC_5> {
    NFace::<SPEC_5>::checked()
}
pub const SPEC_6: Spec = Spec {
    precision: 37,
    bias_num: 346,
    bias_den: 1,
};
pub fn mk_6() -> NFace<SPEC_6> {
    NFace::<SPEC_6>::checked()
}
pub const SPEC_7: Spec = Spec {
    precision: 34,
    bias_num: 284,
    bias_den: 51,
};
pub fn mk_7() -> NFace<SPEC_7> {
    NFace::<SPEC_7>::checked()
}
pub const SPEC_8: Spec = Spec {
    precision: 40,
    bias_num: 128,
    bias_den: 99,
};
pub fn mk_8() -> NFace<SPEC_8> {
    NFace::<SPEC_8>::checked()
}
pub const SPEC_9: Spec = Spec {
    precision: 44,
    bias_num: 185,
    bias_den: 244,
};
pub fn mk_9() -> NFace<SPEC_9> {
    NFace::<SPEC_9>::checked()
}
pub const SPEC_10: Spec = Spec {
    precision: 51,
    bias_num: 191,
    bias_den: 100,
};
pub fn mk_10() -> NFace<SPEC_10> {
    NFace::<SPEC_10>::checked()
}
pub const SPEC_11: Spec = Spec {
    precision: 33,
    bias_num: 119,
    bias_den: 141,
};
pub fn mk_11() -> NFace<SPEC_11> {
    NFace::<SPEC_11>::checked()
}
pub const SPEC_12: Spec = Spec {
    precision: 32,
    bias_num: 352,
    bias_den: 23,
};
pub fn mk_12() -> NFace<SPEC_12> {
    NFace::<SPEC_12>::checked()
}
pub const SPEC_13: Spec = Spec {
    precision: 44,
    bias_num: 461,
    bias_den: 109,
};
pub fn mk_13() -> NFace<SPEC_13> {
    NFace::<SPEC_13>::checked()
}
pub const SPEC_14: Spec = Spec {
    precision: 12,
    bias_num: 176,
    bias_den: 405,
};
pub fn mk_14() -> NFace<SPEC_14> {
    NFace::<SPEC_14>::checked()
}
pub const SPEC_15: Spec = Spec {
    precision: 45,
    bias_num: 120,
    bias_den: 37,
};
pub fn mk_15() -> NFace<SPEC_15> {
    NFace::<SPEC_15>::checked()
}
pub const SPEC_16: Spec = Spec {
    precision: 1,
    bias_num: 98,
    bias_den: 155,
};
pub fn mk_16() -> NFace<SPEC_16> {
    NFace::<SPEC_16>::checked()
}
pub const SPEC_17: Spec = Spec {
    precision: 10,
    bias_num: 299,
    bias_den: 129,
};
pub fn mk_17() -> NFace<SPEC_17> {
    NFace::<SPEC_17>::checked()
}
pub const SPEC_18: Spec = Spec {
    precision: 30,
    bias_num: 5,
    bias_den: 467,
};
pub fn mk_18() -> NFace<SPEC_18> {
    NFace::<SPEC_18>::checked()
}
pub const SPEC_19: Spec = Spec {
    precision: 9,
    bias_num: 8,
    bias_den: 155,
};
pub fn mk_19() -> NFace<SPEC_19> {
    NFace::<SPEC_19>::checked()
}
pub const SPEC_20: Spec = Spec {
    precision: 54,
    bias_num: 75,
    bias_den: 221,
};
pub fn mk_20() -> NFace<SPEC_20> {
    NFace::<SPEC_20>::checked()
}
pub const SPEC_21: Spec = Spec {
    precision: 33,
    bias_num: 7,
    bias_den: 33,
};
pub fn mk_21() -> NFace<SPEC_21> {
    NFace::<SPEC_21>::checked()
}
pub const SPEC_22: Spec = Spec {
    precision: 33,
    bias_num: 47,
    bias_den: 26,
};
pub fn mk_22() -> NFace<SPEC_22> {
    NFace::<SPEC_22>::checked()
}
pub const SPEC_23: Spec = Spec {
    precision: 60,
    bias_num: 99,
    bias_den: 146,
};
pub fn mk_23() -> NFace<SPEC_23> {
    NFace::<SPEC_23>::checked()
}
pub const SPEC_24: Spec = Spec {
    precision: 45,
    bias_num: 215,
    bias_den: 74,
};
pub fn mk_24() -> NFace<SPEC_24> {
    NFace::<SPEC_24>::checked()
}
pub const SPEC_25: Spec = Spec {
    precision: 39,
    bias_num: 403,
    bias_den: 204,
};
pub fn mk_25() -> NFace<SPEC_25> {
    NFace::<SPEC_25>::checked()
}
pub const SPEC_26: Spec = Spec {
    precision: 16,
    bias_num: 367,
    bias_den: 30,
};
pub fn mk_26() -> NFace<SPEC_26> {
    NFace::<SPEC_26>::checked()
}
pub const SPEC_27: Spec = Spec {
    precision: 33,
    bias_num: 248,
    bias_den: 327,
};
pub fn mk_27() -> NFace<SPEC_27> {
    NFace::<SPEC_27>::checked()
}
pub const SPEC_28: Spec = Spec {
    precision: 48,
    bias_num: 276,
    bias_den: 155,
};
pub fn mk_28() -> NFace<SPEC_28> {
    NFace::<SPEC_28>::checked()
}
pub const SPEC_29: Spec = Spec {
    precision: 40,
    bias_num: 56,
    bias_den: 15,
};
pub fn mk_29() -> NFace<SPEC_29> {
    NFace::<SPEC_29>::checked()
}
pub const SPEC_30: Spec = Spec {
    precision: 42,
    bias_num: 356,
    bias_den: 343,
};
pub fn mk_30() -> NFace<SPEC_30> {
    NFace::<SPEC_30>::checked()
}
pub const SPEC_31: Spec = Spec {
    precision: 27,
    bias_num: 69,
    bias_den: 478,
};
pub fn mk_31() -> NFace<SPEC_31> {
    NFace::<SPEC_31>::checked()
}
pub const SPEC_32: Spec = Spec {
    precision: 11,
    bias_num: 31,
    bias_den: 29,
};
pub fn mk_32() -> NFace<SPEC_32> {
    NFace::<SPEC_32>::checked()
}
pub const SPEC_33: Spec = Spec {
    precision: 35,
    bias_num: 405,
    bias_den: 293,
};
pub fn mk_33() -> NFace<SPEC_33> {
    NFace::<SPEC_33>::checked()
}
pub const SPEC_34: Spec = Spec {
    precision: 10,
    bias_num: 67,
    bias_den: 82,
};
pub fn mk_34() -> NFace<SPEC_34> {
    NFace::<SPEC_34>::checked()
}
pub const SPEC_35: Spec = Spec {
    precision: 46,
    bias_num: 338,
    bias_den: 11,
};
pub fn mk_35() -> NFace<SPEC_35> {
    NFace::<SPEC_35>::checked()
}
pub const SPEC_36: Spec = Spec {
    precision: 3,
    bias_num: 20,
    bias_den: 349,
};
pub fn mk_36() -> NFace<SPEC_36> {
    NFace::<SPEC_36>::checked()
}
pub const SPEC_37: Spec = Spec {
    precision: 45,
    bias_num: 249,
    bias_den: 134,
};
pub fn mk_37() -> NFace<SPEC_37> {
    NFace::<SPEC_37>::checked()
}
pub const SPEC_38: Spec = Spec {
    precision: 7,
    bias_num: 87,
    bias_den: 23,
};
pub fn mk_38() -> NFace<SPEC_38> {
    NFace::<SPEC_38>::checked()
}
pub const SPEC_39: Spec = Spec {
    precision: 3,
    bias_num: 132,
    bias_den: 413,
};
pub fn mk_39() -> NFace<SPEC_39> {
    NFace::<SPEC_39>::checked()
}
pub const SPEC_40: Spec = Spec {
    precision: 46,
    bias_num: 70,
    bias_den: 11,
};
pub fn mk_40() -> NFace<SPEC_40> {
    NFace::<SPEC_40>::checked()
}
pub const SPEC_41: Spec = Spec {
    precision: 50,
    bias_num: 149,
    bias_den: 238,
};
pub fn mk_41() -> NFace<SPEC_41> {
    NFace::<SPEC_41>::checked()
}
pub const SPEC_42: Spec = Spec {
    precision: 56,
    bias_num: 488,
    bias_den: 291,
};
pub fn mk_42() -> NFace<SPEC_42> {
    NFace::<SPEC_42>::checked()
}
pub const SPEC_43: Spec = Spec {
    precision: 52,
    bias_num: 71,
    bias_den: 91,
};
pub fn mk_43() -> NFace<SPEC_43> {
    NFace::<SPEC_43>::checked()
}
pub const SPEC_44: Spec = Spec {
    precision: 8,
    bias_num: 123,
    bias_den: 52,
};
pub fn mk_44() -> NFace<SPEC_44> {
    NFace::<SPEC_44>::checked()
}
pub const SPEC_45: Spec = Spec {
    precision: 47,
    bias_num: 88,
    bias_den: 97,
};
pub fn mk_45() -> NFace<SPEC_45> {
    NFace::<SPEC_45>::checked()
}
pub const SPEC_46: Spec = Spec {
    precision: 15,
    bias_num: 107,
    bias_den: 303,
};
pub fn mk_46() -> NFace<SPEC_46> {
    NFace::<SPEC_46>::checked()
}
pub const SPEC_47: Spec = Spec {
    precision: 8,
    bias_num: 55,
    bias_den: 23,
};
pub fn mk_47() -> NFace<SPEC_47> {
    NFace::<SPEC_47>::checked()
}
pub const SPEC_48: Spec = Spec {
    precision: 9,
    bias_num: 69,
    bias_den: 17,
};
pub fn mk_48() -> NFace<SPEC_48> {
    NFace::<SPEC_48>::checked()
}
pub const SPEC_49: Spec = Spec {
    precision: 58,
    bias_num: 54,
    bias_den: 41,
};
pub fn mk_49() -> NFace<SPEC_49> {
    NFace::<SPEC_49>::checked()
}
pub const SPEC_50: Spec = Spec {
    precision: 8,
    bias_num: 472,
    bias_den: 119,
};
pub fn mk_50() -> NFace<SPEC_50> {
    NFace::<SPEC_50>::checked()
}
pub const SPEC_51: Spec = Spec {
    precision: 27,
    bias_num: 384,
    bias_den: 481,
};
pub fn mk_51() -> NFace<SPEC_51> {
    NFace::<SPEC_51>::checked()
}
pub const SPEC_52: Spec = Spec {
    precision: 26,
    bias_num: 367,
    bias_den: 56,
};
pub fn mk_52() -> NFace<SPEC_52> {
    NFace::<SPEC_52>::checked()
}
pub const SPEC_53: Spec = Spec {
    precision: 13,
    bias_num: 399,
    bias_den: 122,
};
pub fn mk_53() -> NFace<SPEC_53> {
    NFace::<SPEC_53>::checked()
}
pub const SPEC_54: Spec = Spec {
    precision: 60,
    bias_num: 479,
    bias_den: 148,
};
pub fn mk_54() -> NFace<SPEC_54> {
    NFace::<SPEC_54>::checked()
}
pub const SPEC_55: Spec = Spec {
    precision: 7,
    bias_num: 8,
    bias_den: 9,
};
pub fn mk_55() -> NFace<SPEC_55> {
    NFace::<SPEC_55>::checked()
}
pub const SPEC_56: Spec = Spec {
    precision: 39,
    bias_num: 55,
    bias_den: 7,
};
pub fn mk_56() -> NFace<SPEC_56> {
    NFace::<SPEC_56>::checked()
}
pub const SPEC_57: Spec = Spec {
    precision: 13,
    bias_num: 61,
    bias_den: 24,
};
pub fn mk_57() -> NFace<SPEC_57> {
    NFace::<SPEC_57>::checked()
}
pub const SPEC_58: Spec = Spec {
    precision: 42,
    bias_num: 479,
    bias_den: 220,
};
pub fn mk_58() -> NFace<SPEC_58> {
    NFace::<SPEC_58>::checked()
}
pub const SPEC_59: Spec = Spec {
    precision: 35,
    bias_num: 417,
    bias_den: 146,
};
pub fn mk_59() -> NFace<SPEC_59> {
    NFace::<SPEC_59>::checked()
}
