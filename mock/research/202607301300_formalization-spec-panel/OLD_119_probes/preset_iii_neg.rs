#![allow(dead_code)]
use core::marker::PhantomData;

// ---- the kind: the coarsest layer the preset rows depend on -----------
pub trait Kind {}
pub struct Fixed;
impl Kind for Fixed {}
pub struct Float;
impl Kind for Float {}

pub trait ExponentForm {
    type Kind: Kind;
}
pub struct Implicit;
impl ExponentForm for Implicit {
    type Kind = Fixed;
}
pub struct Ranged;
impl ExponentForm for Ranged {
    type Kind = Float;
}

pub trait Numeral {
    type Exponent: ExponentForm;
}
pub type KindOf<N> = <<N as Numeral>::Exponent as ExponentForm>::Kind;

pub struct Fix13_3;
impl Numeral for Fix13_3 {
    type Exponent = Implicit;
}
pub struct Binary32;
impl Numeral for Binary32 {
    type Exponent = Ranged;
}

// ---- the vocabularies the two ratified tables use ---------------------
pub trait StoredWidth {}
pub struct Minimum;
impl StoredWidth for Minimum {}
pub struct DoubleLogical;
impl StoredWidth for DoubleLogical {}
pub trait StorageLayout {}
pub struct Dense;
impl StorageLayout for Dense {}
pub struct Bitpacked;
impl StorageLayout for Bitpacked {}
pub trait LoweringDoor {}
pub struct Inert;
impl LoweringDoor for Inert {}
pub struct Quantised;
impl LoweringDoor for Quantised {}
pub struct HostFloat;
impl LoweringDoor for HostFloat {}

// ---- SPELLING (iii): the contract is keyed on the kind ----------------
pub trait Lowering<K: Kind> {
    type StoredWidth: StoredWidth;
    type Layout: StorageLayout;
    type Door: LoweringDoor;
}

pub struct Hot;
pub struct Cold;
pub struct Warm;
pub struct Precise;

// section 1.21's fixed-point table, transcribed column by column
impl Lowering<Fixed> for Hot {
    type StoredWidth = Minimum;
    type Layout = Dense;
    type Door = Inert;
}
impl Lowering<Fixed> for Cold {
    type StoredWidth = Minimum;
    type Layout = Bitpacked;
    type Door = Inert;
}
impl Lowering<Fixed> for Warm {
    type StoredWidth = DoubleLogical;
    type Layout = Dense;
    type Door = Inert;
}
impl Lowering<Fixed> for Precise {
    type StoredWidth = DoubleLogical;
    type Layout = Dense;
    type Door = Inert;
}

// section 1.21's float table, transcribed column by column
impl Lowering<Float> for Hot {
    type StoredWidth = Minimum;
    type Layout = Dense;
    type Door = HostFloat;
}
impl Lowering<Float> for Cold {
    type StoredWidth = Minimum;
    type Layout = Bitpacked;
    type Door = Quantised;
}
impl Lowering<Float> for Warm {
    type StoredWidth = Minimum;
    type Layout = Dense;
    type Door = HostFloat;
}
impl Lowering<Float> for Precise {
    type StoredWidth = DoubleLogical;
    type Layout = Dense;
    type Door = Quantised;
}

// ---- the only text that changes: Number's own bound -------------------
pub struct Number<N: Numeral, S: Lowering<KindOf<N>>>(PhantomData<(N, S)>);

pub type UFixed<S> = Number<Fix13_3, S>;
pub type Binary32Warm = Number<Binary32, Warm>;

// the four cells the two tables disagree on, read back through one name
const fn same<A, B>() -> bool {
    false
}
pub fn check() {
    // Warm on a fixed numeral must reach DoubleLogical; on a float, Minimum.
    let _: <Warm as Lowering<KindOf<Fix13_3>>>::StoredWidth = DoubleLogical;
    let _: <Warm as Lowering<KindOf<Binary32>>>::StoredWidth = Minimum;
    let _: <Hot as Lowering<KindOf<Fix13_3>>>::Door = Inert;
    let _: <Hot as Lowering<KindOf<Binary32>>>::Door = HostFloat;
}

// negative control: force the float row onto a fixed-point numeral
pub fn wrong() {
    let _: <Warm as Lowering<KindOf<Fix13_3>>>::StoredWidth = Minimum;
}
