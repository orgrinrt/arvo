#!/bin/sh
# Rebuilds every probe here and rewrites `outputs/`. Each crate is standalone
# (its own `[workspace]`), so a refusal in one stops no other.
set -u
cd "$(dirname "$0")"
export CARGO_INCREMENTAL=0
rustc -vV > outputs/toolchain.txt
diff ../202611192015_numeral-contracts/arm-const/src/lib.rs arm-const-copy/src/lib.rs > outputs/arm-const-copy-diff.txt; echo "diff exit $?" >> outputs/arm-const-copy-diff.txt
for c in concrete-const-call-no-gate monotone-outside-policy monotone-policy-marker-wrap; do
  cargo build --manifest-path "$c/Cargo.toml" > "outputs/$c.txt" 2>&1; echo "exit $?" >> "outputs/$c.txt"
done
for c in adt-param-consumer gated-const-eval ungated-call-of-gated-generic ungated-runtime-call-control opt-on-gated-control monotone-saturate-control monotone-policy-marker abort-oracle; do
  cargo test --manifest-path "$c/Cargo.toml" > "outputs/$c.txt" 2>&1; echo "exit $?" >> "outputs/$c.txt"
done
{ echo "== cargo check"; cargo check --manifest-path constant-bound-check/Cargo.toml 2>&1; echo "exit $?";
  echo "== cargo build"; cargo build --manifest-path constant-bound-check/Cargo.toml 2>&1; echo "exit $?"; } > outputs/constant-bound-check.txt
cargo build --release --manifest-path law-regions/Cargo.toml > outputs/law-regions-build.txt 2>&1
cargo run --release --manifest-path law-regions/Cargo.toml > outputs/law-regions.txt 2>> outputs/law-regions-build.txt
echo "exit $?" >> outputs/law-regions-build.txt
