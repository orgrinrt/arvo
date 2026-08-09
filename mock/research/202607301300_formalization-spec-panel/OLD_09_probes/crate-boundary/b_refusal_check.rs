//! confirms Hot's SubstituteZero rule is refused by the fact, even under the
//! crate split: Number<Fix13_3Signed, Hot, MinWidth> must not fold.
use algebra::AddAssoc;
use lowering::MinWidth;
use numeral::Fix13_3Signed;
use numeric_honest::Number;
use policy::Hot;

pub fn fold<T: AddAssoc>() {}

pub fn bad() {
    fold::<Number<Fix13_3Signed, Hot, MinWidth>>();
}
