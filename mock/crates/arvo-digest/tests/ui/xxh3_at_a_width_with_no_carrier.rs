// XXH3 at a width `Carrier` is not implemented for is not a type.

use arvo_digest::Xxh3;

fn takes(_: Xxh3<96>) {}

fn main() {}
