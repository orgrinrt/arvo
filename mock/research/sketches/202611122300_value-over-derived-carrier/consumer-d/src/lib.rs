//! Arm D from a crate with no `#![feature]` line: the five consumer cases.
//!
//! Case (3) bounds the signature by its carrier, as arm A does: the alias
//! normalises only once the carrier is known.

#![no_std]

use arm_d::Num;

common::consumer_cases!(Num, [common::Sole<Repr = i64>]);
