#!/usr/bin/env bash
# Seat 253. Two compilations: the verdict arm prints, the forcing arm must fail.
# `build` rather than `check` on the forcing arm, because `ADMITTED` is forced at
# codegen and `cargo check` does not reach it.
set -u
echo "======== rustc"
rustc --version
echo
echo "======== ARM 1: the verdict form, which does not force the const"
cargo run --quiet 2>&1
echo "-------- exit: $?"
echo
echo "======== ARM 2 (the case that must fail): forcing ADMITTED on Wide63"
cargo build --quiet --features force 2>&1
echo "-------- exit: $?"
echo
echo "======== CONTROL for ARM 2: the same forcing on the 62-bit control builds"
sed 's/declared_slot_width::<Wide63>/declared_slot_width::<Narrow62>/' src/main.rs > src/main.rs.control
mv src/main.rs src/main.rs.orig && mv src/main.rs.control src/main.rs
cargo build --quiet --features force 2>&1
echo "-------- exit: $?"
mv src/main.rs.orig src/main.rs
touch src/main.rs
