// FNV-1a at a width `Carrier` is not implemented for is not a type.

use arvo_digest::Fnv1a;

fn takes(_: Fnv1a<96>) {}

fn main() {}
