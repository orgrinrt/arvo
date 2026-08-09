//! Route 4: skip membership and reach the audited blanket entry itself at a
//! non-member instantiation. Expected: refused at the struct's own bound
//! before `Crosses` is ever consulted.
#![no_std]
extern crate tower;

#[derive(Copy, Clone)]
pub struct NotANiche(u16);

fn require_crosses<T: tower::Crosses>() {}

pub fn attack() {
    require_crosses::<tower::ViaNiche<NotANiche>>();
}
