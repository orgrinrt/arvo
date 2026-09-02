// Build 1: proves the description machinery compiles under #![no_std] with NO
// feature gate whatsoever. Nothing is executed here; this build IS the claim.
#![no_std]
include!("expr.rs");
