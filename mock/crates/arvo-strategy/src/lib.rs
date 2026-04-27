//! arvo-strategy. Strategy markers + container-projection traits.
//!
//! `Hot` / `Warm` / `Cold` / `Precise` ZSTs. `Strategy` marker
//! trait. `UContainerFor<N>` / `IContainerFor<N>` const traits
//! projecting strategy + bit-width to bare primitive containers.
//! `Resolve<S1, S2>` strategy resolution.
//!
//! See `DESIGN.md` for the full surface.

#![no_std]
#![feature(adt_const_params)]
#![feature(const_trait_impl)]
#![feature(const_param_ty_trait)]
#![allow(incomplete_features)]
