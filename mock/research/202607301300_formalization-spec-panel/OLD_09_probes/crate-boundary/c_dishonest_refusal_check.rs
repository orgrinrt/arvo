use algebra::AddAssoc;
use lowering::DoubleWidth;
use numeral::Fix13_3Signed;
use numeric_dishonest::Number;
use policy::Warm;
pub fn fold<T: AddAssoc>() {}
pub fn refuses_at_bitpacked() {
    fold::<Number<Fix13_3Signed, Warm, DoubleWidth>>();
}
