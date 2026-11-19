// A digest at a width `Carrier` is not implemented for is not a type.

use arvo_digest::Digest;

fn takes(_: Digest<96>) {}

fn main() {}
