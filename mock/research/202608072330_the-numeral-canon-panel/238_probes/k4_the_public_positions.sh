#!/usr/bin/env bash
# Which of the exempt crate's bare-primitive lines are *public API positions*.
#
# k3 measures what the lint reaches, which is every line including bodies,
# casts and tests. The obligation is narrower: it is about positions. This
# splits them, over declaration syntax only.
#
# It is a line scan and says so. What makes it usable rather than a guess is
# the control below: the same scan over the two crates that are checked at
# every gate today must return zero, and over the crate's own test modules it
# must return the test-module lines rather than nothing, so a filter that
# silently matched nothing would be visible.
#
# stderr is folded in on every command, because a `grep` that dies returns the
# same zero as a `grep` that found nothing.
set -u
# Three levels: 238_probes -> the panel dir -> research -> mock.
# It was four for one run, which put it at the repo root where `crates/`
# does not exist. Both controls still returned zero, because a `grep` over a
# missing path emits its complaint on stderr and the count of matching lines
# in that complaint is zero. The third control is what caught it.
cd "$(dirname "$0")/../../.."   # -> mock/

PRIM='(u8|u16|u32|u64|u128|i8|i16|i32|i64|i128|f32|f64|usize|isize|bool)'
# A public position: a `pub fn` / `pub const fn` signature, a trait or impl
# associated constant, or a `pub const` item. Bodies, casts and `let` bindings
# are deliberately outside it.
POSITION='^[[:space:]]*(pub (const )?fn |const [A-Z_]+:|pub const [A-Z_]+:)'

scan() {
    grep -rhE "$POSITION" "$1" 2>&1 | grep -cE "\b$PRIM\b" 2>&1
}

echo "=== controls: the crates checked at every gate today ==="
for c in arvo-placement arvo-strategy; do
    n=$(scan "crates/$c/src")
    echo "  crates/$c/src  -> $n public position(s) with a bare primitive"
    if [ "$n" != "0" ]; then
        echo "  CONTROL FAILED: a checked crate carries one, so either the scan is"
        echo "  wrong or the repository has a live gate violation. Stop here."
        exit 1
    fi
done

echo
echo "=== control: the scan can find something at all ==="
n=$(scan "crates/arvo-format/src")
if [ "$n" = "0" ]; then
    echo "  CONTROL FAILED: the scan found nothing anywhere, so its zero above"
    echo "  is a fact about the pattern rather than about the crates."
    exit 1
fi
echo "  crates/arvo-format/src -> $n, so the scan does match."

echo
echo "=== the introducing crate, per file, non-test source only ==="
total=0
for f in crates/arvo-format/src/*.rs crates/arvo-format/src/apply/*.rs; do
    case "$f" in *tests.rs) continue;; esac
    n=$(grep -hE "$POSITION" "$f" 2>&1 | grep -cE "\b$PRIM\b" 2>&1)
    [ "$n" = "0" ] && continue
    printf '  %3s  %s\n' "$n" "$f"
    total=$((total + n))
done
echo "  ---"
printf '  %3s  public positions carrying a bare primitive, tests excluded\n' "$total"

echo
echo "=== of those, the ones an outside implementor must write ==="
grep -rnE '^[[:space:]]+const [A-Z_]+: ' crates/arvo-format/src/*.rs 2>&1 \
  | grep -E "\b$PRIM\b" 2>&1 \
  | grep -vE '=' \
  | sed 's/^/  /'
echo "  ---"
n=$(grep -rhE '^[[:space:]]+const [A-Z_]+: ' crates/arvo-format/src/*.rs 2>&1 \
  | grep -E "\b$PRIM\b" 2>&1 | grep -cvE '=' 2>&1)
printf '  %3s  trait associated constants declared with a bare primitive\n' "$n"
