#!/usr/bin/env bash
# Seat q31a. Compile and run `refused_candidates.rs` against the shipped crate.
#
# Builds arvo-format through cargo (so the rlib is the real one, at the pinned
# toolchain), then compiles the probe against it with rustc directly. rustc
# rather than a nested cargo project, because `mock/` is a cargo workspace and a
# member added under `mock/research/` would either be swept into it or need an
# exclude entry, and a probe does not get to edit the workspace manifest.
set -uo pipefail
cd "$(dirname "$0")/../../.." || exit 1

cargo build --manifest-path mock/Cargo.toml -p arvo-format >/dev/null 2>&1 || {
  echo "arvo-format did not build; nothing downstream of this is evidence"; exit 1; }

DEPS=mock/target/debug/deps
FMT=$(ls -t "$DEPS"/libarvo_format-*.rlib 2>/dev/null | head -1)
NOTKO=$(ls -t "$DEPS"/libnotko-*.rlib 2>/dev/null | head -1)
[ -n "$FMT" ] || { echo "no arvo_format rlib found under $DEPS"; exit 1; }
echo "linking against:"
echo "  $FMT"
echo "  ${NOTKO:-<notko rlib not found>}"
echo

OUT=mock/target/q31a_refused_candidates
rustc --edition 2024 -L "$DEPS" \
  --extern arvo_format="$FMT" \
  ${NOTKO:+--extern notko="$NOTKO"} \
  -o "$OUT" "$(dirname "$0")/refused_candidates.rs" || {
    echo "PROBE DID NOT COMPILE. That is a result and is not to be papered over."; exit 1; }

"$OUT"
