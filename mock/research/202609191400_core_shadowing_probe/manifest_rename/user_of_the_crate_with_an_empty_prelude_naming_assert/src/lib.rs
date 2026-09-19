// The rename of `user_of_the_crate_with_an_empty_prelude`, onto the same
// empty-prelude stand-in, naming `assert!` once. The renamed crate's
// `prelude::rust_2024` is empty, so no `assert!` is in scope, and this arm
// shows that by being refused where it names one. The `assert!(true)` checks
// no value; the name lookup is the whole of what this arm is about.
//
// Outcome: REFUSED, `cannot find macro `assert` in this scope`, which
// `run.sh` checks for.
#![no_std]

const _: () = assert!(true);
