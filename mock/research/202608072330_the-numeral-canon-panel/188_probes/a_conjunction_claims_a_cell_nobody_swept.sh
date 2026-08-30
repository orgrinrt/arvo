#!/usr/bin/env bash
# `predicate.rs` reads a predicate as a conjunction, one entry per axis. So a
# `holds` list naming a set on two axes claims the whole product of those sets.
#
# 182 section 8.1 found the case where the VERDICT inverts across a cell and
# split the law in two. This is the case where the verdict does not invert and
# the EVIDENCE does not cover the product. Nothing splits, nothing fires, and
# the row claims a cell no cited instrument reached.
#
# CASE THAT MUST FAIL: control 1 takes a cell the instruments demonstrably DO
# cover and runs the identical argument on it. If that also reads as
# unestablished, the argument proves everything and therefore nothing.
set -uo pipefail
cd "$(dirname "$0")/.."

show() { printf '\n--- %s ---\n' "$1"; awk -v id="$1" '$0=="id = \""id"\""{f=1} f{print} f&&/^provenance/{exit}' \
  ../../registry/law.toml | grep -E '^(holds|fails|  "(total_width|signedness|fraction_width|overflow_policy|operation)|note)' | cut -c1-210; }

echo "############ INSTANCE 1"
show additive_associativity_under_wrapping
cat <<'T'

  the product the `holds` conjunction claims:
    {W=4, W=8} x {unsigned, signed} x {F any} x wrap x add x arity 3   = 4 width/sign cells
  what the cited instruments cover:
    63's cube (`63:443-447`)      signed|add|wrap = "abelian group / group, every F", at four-bit widths
    76 (`90:122`)                 "wrapping addition associates universally over `u8`" -- UNSIGNED, W=8
    93 F10 (`93:1005-1006`)       signedness = signed, overflow in {wrap, saturate}, W in 3..7
  the cell nobody reached:        signed, W = 8, wrap
T

echo "############ INSTANCE 2"
show multiplicative_associativity_under_wrapping
cat <<'T'

  the product the `holds` conjunction claims:
    {W in 3..=8} x {unsigned, signed} x F=0 x wrap x mul x arity{2,3}
  what the cited instruments cover:
    93 F1  (`93:544`)   "signedness = unsigned", W in 3..8
    93 F1a (`93:551`)   "signedness = unsigned", W any, by proof
    93 F10 (`93:1005`)  signedness = signed, W in 3..7
    63's cube           signed|mul|wrap = "ring" at four-bit widths
  the cell nobody reached:        signed, W = 8, wrap
  and the row's own note names the gap in prose: it says the 93 F1 sweep is
  "for the unsigned case". The note is prose. The predicate is what a gate reads.
T

echo
echo "############ CONTROL 1: the identical argument on a cell the instruments DO cover"
cat <<'T'
  cell:  unsigned, W = 8, wrap, add
  90:122 states 76's exhaustive probe: wrapping addition associates universally
  over `u8`. That is unsigned at W = 8 exactly. The argument above, applied here,
  finds the cell covered, so it discriminates rather than rejecting every cell.
T
grep -n 'associates universally over `u8`' 90_giesen_consolidation_derived_algebraic_laws.md | cut -c1-150

echo
echo "############ CONTROL 2: a narrowing, which is the opposite direction and is honest"
cat <<'T'
  93 F1a proves the unsigned F=0 multiplicative half at `W any`, by a congruence
  argument rather than a sweep. The row writes `W in 3..=8`, which claims LESS
  than the source. That is honest under the notation and is recorded here only
  so the two directions are not confused: instance 2 is a widening, this is not.
T
sed -n '551,553p' 93_orchard_the_strategy_axis_derived_cold.md
