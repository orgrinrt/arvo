//! file 62's own fifth-route attack: implement `AtLeastTwo` (and its private
//! supertrait) for `H` from outside, which would make `Rad<H>` (radix one) legal.
//! Expected: refused on both the trait reach and the seal.
extern crate vu54;
use vu54::bias::nat::H;
use vu54::numeral::AtLeastTwo;

struct Smuggle;
impl vu54::numeral::radix_sealed::AtLeastTwoSealed for H {}
impl AtLeastTwo for H {}
