//! Arm A from a crate with no `#![feature]` line: the five consumer cases.
//!
//! Case (3) bounds the signature by its carrier, `Sole<Repr = i64>`: the
//! inherent methods live on `Num<S, i64>`, so a body generic over every
//! signature has no impl to call.

#![no_std]

use arm_a::Num;

common::consumer_cases!(Num, [common::Sole<Repr = i64>]);
