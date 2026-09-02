#!/usr/bin/env bash
# Seat 253. Four compilations. ARM 3 must fail and ARM 4 must not; without both
# the refusal in ARM 3 would say nothing about which declaration it refused.
set -u
echo "======== rustc"
rustc --version
echo
echo "======== ARM 1 and ARM 2: the exposed verdict and the derived one"
cargo run --quiet 2>&1
echo "-------- exit: $?"
echo
echo "======== ARM 3 (the case that must fail): pin Declared<true> against the derivation"
cargo build --quiet --features the_lie_is_refused 2>&1
echo "-------- exit: $?"
echo
echo "======== ARM 4 (the control): pin Declared<false> against the same derivation"
cargo build --quiet --features the_honest_declaration_is_refused_too 2>&1
echo "-------- exit: $?"
