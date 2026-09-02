// stands in for a consumer crate: tries to populate the bridge for its own width.
#![no_std]
extern crate d14_lib;
use d14_lib::*;
impl ToNat for Idx<14> {
    type N = N1<N4<End>>;
}
