// Probe E, the two negatives the form keying is claimed to buy.
#![no_std]
#![feature(const_trait_impl)]
#![allow(dead_code)]
extern crate base;
extern crate pd;
use base::*;
use pd::*;

// NEGATIVE ONE: over-keying. Try to give one numeral its own row, which is
// exactly what probe B admitted at exit 0.
const impl Lowering<Implicit<ENeg<I<H>>, BZero, BZero>> for Warm {
    type StoredWidth = Minimum;
    type Layout = Bitpacked;
    type Door = Inert;
    type Container = u32;
}
