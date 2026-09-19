// The rename of `user_of_the_crate_with_a_bare_prelude`, onto the same
// bare-prelude stand-in, naming `assert!` once. The prelude import is refused,
// so no `assert!` is in scope, and this arm shows that by being refused on the
// name as well as on the import. The `assert!(true)` checks no value; the name
// lookup is the whole of what this arm is about.
//
// Outcome: REFUSED, with both `cannot resolve a prelude import` and
// `cannot find macro `assert` in this scope`, which `run.sh` checks for.
#![no_std]

const _: () = assert!(true);
