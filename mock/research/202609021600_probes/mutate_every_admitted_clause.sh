#!/usr/bin/env bash
# Q31 attack probe. Does the suite isolate each of the nine ADMITTED clauses?
#
# For each clause: neutralise it in BOTH the `assert!` inside `ADMITTED` and the
# matching conjunct of the verdict `const fn`, then run the whole workspace suite.
# Red means at least one test can see that clause. Green means the clause could be
# deleted and nothing in the suite would notice.
#
# Controls, both required and both reported:
#   C0  unmutated tree must be GREEN, or the instrument is measuring something else.
#   C1  a mutation known to be seen (slots MIN<=MAX) must be RED, or the method
#       cannot report red at all and every green below is worthless.
#
# Restores by copying (not moving), so the restored file carries a current mtime
# and cargo rebuilds it. See `a-restored-file-does-not-rebuild`.

set -uo pipefail

ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
SRC="$ROOT/mock/crates/arvo-format/src"
BAK="$(mktemp -d)"
OUT="$(dirname "$0")/output_mutate_every_admitted_clause.txt"

FILES="ambient.rs quantum.rs slots.rs format.rs"
for f in $FILES; do cp "$SRC/$f" "$BAK/$f"; done

restore() { for f in $FILES; do cp "$BAK/$f" "$SRC/$f"; touch "$SRC/$f"; done; }
trap restore EXIT

# sub <file> <literal-find> <literal-replace>
sub() {
  perl -0777 -i -pe "BEGIN{\$f=\$ARGV[0]} s/\Q$2\E/$3/s or die 'PATTERN NOT FOUND in $1: $2'" "$SRC/$1"
}

run_suite() {
  ( cd "$ROOT/mock" && cargo test --workspace 2>&1 )
}

# name|site|apply-function
mutate() {
  case "$1" in
    ambient_radix_positional)
      sub ambient.rs 'Self::RADIX.is_positional().get(),' 'true,'
      sub ambient.rs '    A::RADIX.is_positional()' '    Bool::TRUE' ;;
    quantum_ranges_over_a_magnitude)
      sub quantum.rs 'ranges_over_a_magnitude(Self::MAGNITUDES),' 'true,'
      sub quantum.rs 'ranges_over_a_magnitude(Q::MAGNITUDES)' 'true' ;;
    quantum_reach_is_representable)
      sub quantum.rs 'reach_is_representable(Self::BASE, Self::SLOPE, Self::MAGNITUDES),' 'true,'
      sub quantum.rs 'reach_is_representable(Q::BASE, Q::SLOPE, Q::MAGNITUDES),' 'true,' ;;
    slots_range_not_inverted)
      sub slots.rs 'Self::MIN.index() <= Self::MAX.index(),' 'true,'
      sub slots.rs 'S::MIN.index() <= S::MAX.index()' 'true' ;;
    slots_width_at_least_one)
      sub slots.rs 'Self::WIDTH.count() >= 1,' 'true,'
      sub slots.rs '&& S::WIDTH.count() >= 1' '&& true' ;;
    slots_width_at_most_62)
      sub slots.rs 'Self::WIDTH.count() <= 62,' 'true,'
      sub slots.rs '&& S::WIDTH.count() <= 62' '&& true' ;;
    slots_span_fits_a_count)
      sub slots.rs '(Self::MAX.index() as i128) - (Self::MIN.index() as i128) < i64::MAX as i128,' 'true,'
      sub slots.rs '&& (S::MAX.index() as i128) - (S::MIN.index() as i128) < i64::MAX as i128' '&& true' ;;
    slots_width_addresses_span)
      sub slots.rs '(Self::MAX.index() as i128) - (Self::MIN.index() as i128)
                < (1i128 << Self::WIDTH.count()),' 'true,'
      sub slots.rs '&& (S::MAX.index() as i128) - (S::MIN.index() as i128) < (1i128 << S::WIDTH.count()),' '&& true,' ;;
    format_phase_denotes)
      sub format.rs 'Self::PHASE.denotes().get(),' 'true,'
      sub format.rs '    F::PHASE.denotes()' '    Bool::TRUE' ;;
    *) echo "unknown mutation $1"; exit 2 ;;
  esac
}

MUTATIONS="ambient_radix_positional quantum_ranges_over_a_magnitude quantum_reach_is_representable slots_range_not_inverted slots_width_at_least_one slots_width_at_most_62 slots_span_fits_a_count slots_width_addresses_span format_phase_denotes"

{
  echo "probe: does the suite isolate each ADMITTED clause?"
  echo "tree:  $(git -C "$ROOT" rev-parse --short HEAD)"
  echo "tool:  $(cd "$ROOT/mock" && rustc --version)"
  echo

  echo "--- C0: unmutated control (must be GREEN) ---"
  restore
  base="$(run_suite)"
  if echo "$base" | grep -qE '^test result: FAILED|error(\[|:)'; then
    echo "C0 RED. instrument invalid, stopping."
    echo "$base" | grep -E '^(test result|error)' | head
    exit 1
  fi
  echo "C0 GREEN. baseline passing count: $(echo "$base" | grep -E '^test result' | awk -F'[ ;]' '{p+=$4} END {print p}')"
  echo

  printf '%-34s %-7s %s\n' "clause" "suite" "what went red"
  printf '%-34s %-7s %s\n' "----------------------------------" "-------" "--------------"

  for m in $MUTATIONS; do
    restore
    if ! mutate "$m" 2>/tmp/q31_sub_err; then
      printf '%-34s %-7s %s\n' "$m" "ERROR" "$(cat /tmp/q31_sub_err | head -1)"
      continue
    fi
    res="$(run_suite)"
    if echo "$res" | grep -qE '^test result: FAILED|^error(\[|:)'; then
      failed="$(echo "$res" | grep -E '^(test .* FAILED|failures:$)' -A 40 | grep -E '^    [a-z]' | sort -u | tr '\n' ' ')"
      [ -z "$failed" ] && failed="$(echo "$res" | grep -E '^error' | head -1)"
      printf '%-34s %-7s %s\n' "$m" "RED" "${failed:0:110}"
    else
      printf '%-34s %-7s %s\n' "$m" "GREEN" "nothing. the clause is invisible to the suite"
    fi
  done

  echo
  echo "--- C1: the method can report RED (slots_range_not_inverted above) ---"
  echo "If that row is GREEN the whole sweep is void."
  restore
  echo
  echo "--- final restore verified ---"
  for f in $FILES; do
    if diff -q "$BAK/$f" "$SRC/$f" >/dev/null; then echo "  $f restored"; else echo "  $f DIFFERS, restore failed"; fi
  done
} | tee "$OUT"
