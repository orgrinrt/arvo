#!/usr/bin/env bash
# How many types the stack has declared that are one host primitive in a
# wrapper, which is arvo's job done at the consumer.
#
# `pub struct X(u32);` and `pub struct X(pub u32);`, in a crate's own `src/`,
# at a ref rather than off a working tree. Counted per tree and listed, because
# the list is the answer: each is a position where somebody wanted a numeral,
# found none, and wrote one.
#
# The grep is deliberately narrow. A newtype spanning two lines, or wrapping a
# primitive behind a generic, is not counted, so this is a floor rather than a
# total, and it is the direction that understates the class.
set -euo pipefail
W="${1:?usage: <workspace root>}"
PRIM='u8|u16|u32|u64|u128|usize|i8|i16|i32|i64|i128|isize|f32|f64|bool'
total=0
for r in notko:origin/dev arvo:origin/dev hilavitkutin:origin/dev vehje:origin/main kolli:origin/dev; do
  repo="${r%%:*}"; ref="${r##*:}"
  echo "== $repo @ $ref"
  # `-h` would drop the path, and the path is what says whether the hit is in
  # a crate or in a bench variant. `mock/crates/*/src/` and a plain `src/` are
  # the two shapes a shipped crate takes here; everything else is a probe.
  hits=$(git -C "$W/$repo" grep -nE "^ *pub struct [A-Za-z_][A-Za-z0-9_]*\((pub )?($PRIM)\) *;" "$ref" \
      -- '*/src/*.rs' 'src/*.rs' 2>/dev/null \
      | grep -E ':(mock/crates/[a-z0-9-]+|[a-z0-9-]+)/src/|:src/' \
      | grep -vE 'benches/|research/|sketches/|/tests/|/examples/' \
      | sed "s|^$ref:||" | sort || true)
  if [ -n "$hits" ]; then
    echo "$hits" | sed 's/^/   /'
    n=$(echo "$hits" | wc -l | tr -d ' ')
  else
    n=0
  fi
  echo "   -> $n"
  total=$((total + n))
done
echo
echo "total: $total"
