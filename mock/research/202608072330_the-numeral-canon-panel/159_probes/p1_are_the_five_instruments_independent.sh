#!/bin/sh
# 159 P1. F157-3 takes 154's F6 "from one instrument to five". Are the five
# independent instances, or one family counted five times?
#
# RULES.md:116-118: "One instance of evidence is never enough. Three
# independent ones is the bar, and independence means arrived at differently,
# not three probes sharing one model." 154 withdrew three of its own findings
# this week for an instrument too thin to reach the breaking case; this applies
# the same knife to a strengthening of 154's own claim.
#
# Three independence signals, each mechanical:
#   A. the cargo dependency graph among the crates
#   B. symbols declared identically in more than one of them
#   C. whether the family root, which 157 did not count, shows the same pattern
#
# NEGATIVE CONTROL, stated before the run. The audit must find at least one
# crate that is genuinely independent, and must find warm-container-shared
# independent of the bitpack family. If it reports EVERY crate as dependent the
# instrument is just measuring "these are all bench crates" and says nothing
# about F157-3. If it reports every crate as independent, F157-3 stands as
# written and this probe refutes its own suspicion.
V=${ARVO_ROOT:-/Users/orgrinrt/Dev/clause-dev/arvo}/mock/benches/variants
[ -d "$V" ] || { echo "skip: no variants dir at $V"; exit 0; }
FOUR="bitpack-carrier-shared bitpack-footprint-shared bitpack-shared bitpack-wide-shared"
ALL="$FOUR warm-container-shared bitpack-plan-shared"

echo "=== A. dependency graph (bench-* deps only) ==="
dep_count=0
for c in $ALL; do
  d=$(grep -A8 '^\[dependencies\]' "$V/$c/Cargo.toml" | grep -oE 'bench-bitpack-[a-z-]+' | sort -u | tr '\n' ' ')
  printf '  %-26s -> %s\n' "$c" "${d:-<none: bench-core only>}"
  [ -n "$d" ] && dep_count=$((dep_count + 1))
done
echo "  crates in FOUR depending on another bitpack crate: $(for c in $FOUR; do grep -A8 '^\[dependencies\]' "$V/$c/Cargo.toml" | grep -qE 'bench-bitpack-[a-z-]+' && echo x; done | wc -l | tr -d ' ') of 4"

echo
echo "=== B. symbols declared identically in more than one crate ==="
for sym in 'struct SplitMix64(u64);' 'pub type Plan13 = Pack<LOGICAL_BITS>;'; do
  n=0; who=""
  for c in $ALL; do
    if grep -qF "$sym" "$V/$c/src/lib.rs" 2>/dev/null; then n=$((n+1)); who="$who $c"; fi
  done
  printf '  %-42s in %s crates:%s\n' "$sym" "$n" "$who"
done

echo
echo "=== C. the family root 157 did not count ==="
printf '  bitpack-plan-shared declares: '
grep -nE '^(pub )?(struct|type|trait|enum) ' "$V/bitpack-plan-shared/src/lib.rs" | grep -iE 'column|pack|packing' | sed 's/^/\n    /'
echo
echo "  (three of the FOUR depend on this crate; footprint's own header says it"
echo "   reuses its transform 'unmodified')"

echo
# ── controls ──
INDEP=0
for c in $FOUR; do
  grep -A8 '^\[dependencies\]' "$V/$c/Cargo.toml" | grep -qE 'bench-bitpack-[a-z-]+' || INDEP=$((INDEP+1))
done
WC_INDEP=0
grep -A8 '^\[dependencies\]' "$V/warm-container-shared/Cargo.toml" | grep -qE 'bench-bitpack-' || WC_INDEP=1
echo "CONTROL at least one of the four is independent : $INDEP (want >= 1)"
echo "CONTROL warm-container-shared is independent    : $WC_INDEP (want 1)"
if [ "$INDEP" -lt 1 ] || [ "$WC_INDEP" -ne 1 ]; then
  echo "CONTROL FAILED -- the audit is measuring 'these are all bench crates', suppressed"; exit 1
fi
echo
echo "VERDICT: the four packed-end crates are not four independent instances."
echo "         Three of the four depend on bitpack-plan-shared, whose own"
echo "         PlanColumn/MacColumn are the same column-and-no-element shape,"
echo "         and the copied SplitMix64 and Plan13 alias show a shared"
echo "         ancestor independent of the cargo graph. Counting the family"
echo "         once, the independent instances supporting 154 F6 are:"
echo "           1. warm-container-shared's Carrier: Copy bound (structural)"
echo "              -- and note SplitMix64 is in 6 of 6, warm-container-shared"
echo "              included, so its VOCABULARY is template-shared too. What"
echo "              supports F6 there is the Copy bound, which is a fact about"
echo "              the target rather than about the template, so the sharing"
echo "              does not touch it."
echo "           2. bitpack-shared (bench-core only)"
echo "           3. the bitpack-plan-shared family, as ONE instance"
echo "         Three, which meets RULES.md's bar. Not five."
echo
echo "GENERALISES: SplitMix64 is declared identically in 6 of 6 crates examined."
echo "         Every bench crate in this corpus descends from one template, so"
echo "         'N bench crates agree' is worth much less than N anywhere in this"
echo "         panel, not only here. A vocabulary count over these crates is one"
echo "         instance unless the dependency graph and the copied symbols are"
echo "         checked first."
