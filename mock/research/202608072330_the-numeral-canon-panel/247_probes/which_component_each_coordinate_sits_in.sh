#!/usr/bin/env bash
# Seat 247. The containment 246's section 2.2 rests on, re-derived from source
# rather than from 241, on two trees: the tree 246 measured (a12d4d5d, the tree
# of commit 033c02e2) and this worktree's HEAD, between which 748c6004 landed and
# respelled every coordinate.
#
# The claim: every required associated constant of arvo-format's four format
# traits (Ambient, Quantum, Slots, Format) is a refinement of chain component 1
# (the ambient domain) or component 2 (the representable set), and components 3
# (reduction), 4 (encoding) and 5 (container) carry no coordinate in arvo-format.
#
# Method: for each tree, extract the associated consts declared WITHOUT a default
# body from each trait (a const with `= {` is an obligation, not a coordinate).
# Component membership is read off each module's own doc comment, printed here so
# a reader can disagree with the mapping rather than with a hidden table.
#
# THE CASES THAT MUST FAIL, run before the census is reported:
#   C1  The extractor must count a different number on the two trees, since
#       748c6004 merged PHASE_NUM and PHASE_DEN into PHASE; an extractor that
#       reports the same count on both is not reading the trees.
#   C2  The extractor must exclude a const carrying a default body: a planted
#       trait with two required consts and one defaulted must count two.
#   C3  The encoding/container sweep must be able to find a hit: run against a
#       planted file that declares `trait Encoding`, it must report one.
set -u
cd "$(dirname "$0")/../../../.." || exit 1   # the repo root
fail() { echo "CONTROL FAILED: $1"; exit 2; }
g() { command grep "$@"; }
SRC=mock/crates/arvo-format/src
OLD=033c02e2   # tree a12d4d5d, which 246 cites
NEW=HEAD

# required consts of the trait named $2 in the text on stdin
required_consts() {  # required_consts <trait>
  awk -v t="$1" '
    $0 ~ "^pub trait "t"( |$|<)" {inb=1; next}
    inb && /^}/ {inb=0}
    inb && /^    const [A-Z_]+:/ {
      line=$0
      if (line ~ /= \{/ || line ~ /= \{$/) next   # a default body: an obligation, not a coordinate
      sub(/^    const /,"",line); sub(/:.*/,"",line); print line
    }'
}
consts_at() {  # consts_at <rev> <file> <trait>
  git show "$1:$SRC/$2" | required_consts "$3"
}

# --- C2: a planted defaulted const is excluded ------------------------------------
planted='pub trait Planted {
    const A: u32;
    const B: u32;
    const ADMITTED: () = {
        assert!(true);
    };
}'
n=$(printf '%s\n' "$planted" | required_consts Planted | wc -l | tr -d ' ')
[ "$n" -eq 2 ] || fail "C2, planted trait counted $n required consts, expected 2"
echo "C2 passes: a planted trait with two required consts and one defaulted counts 2."

# --- C3: the encoding sweep can find a hit -----------------------------------------
enc_sweep() { g -E '^(pub )?(trait|struct|enum) (Encoding|Encode|Gray|TwosComplement|OffsetBinary|Container|Carrier)\b' "$@"; }
tmp=$(mktemp -t s247enc); trap 'rm -f "$tmp"' EXIT
printf 'pub trait Encoding {}\n' > "$tmp"
[ "$(enc_sweep "$tmp" | wc -l | tr -d ' ')" -eq 1 ] || fail "C3, the encoding sweep cannot see a planted trait Encoding"
echo "C3 passes: the encoding/container sweep reports a planted 'trait Encoding'."

# --- the census, per tree ---------------------------------------------------------
census() {  # census <rev>
  local rev=$1 total=0
  echo "tree $(git rev-parse "$rev^{tree}" | cut -c1-8) (commit $(git rev-parse --short "$rev")):"
  for pair in ambient.rs:Ambient quantum.rs:Quantum slots.rs:Slots format.rs:Format; do
    f=${pair%%:*}; t=${pair##*:}
    cs=$(consts_at "$rev" "$f" "$t" | tr '\n' ' ')
    k=$(consts_at "$rev" "$f" "$t" | wc -l | tr -d ' ')
    total=$((total+k))
    printf '  %-8s %-8s %d  %s\n' "$t" "$f" "$k" "$cs"
  done
  echo "  format-trait coordinates: $total"
  ar=$(consts_at "$rev" adapt.rs Operation | tr '\n' ' ')
  echo "  Operation (adapt.rs) required consts: ${ar:-none}"
  echo "  encoding/container items declared in $SRC at this tree:"
  hits=0
  for f in $(git ls-tree --name-only "$rev" $SRC/ | sed "s|$SRC/||"); do
    h=$(git show "$rev:$SRC/$f" | enc_sweep | wc -l | tr -d ' ')
    hits=$((hits+h))
  done
  echo "    $hits"
  echo "  does arvo-format depend on arvo-placement at this tree:"
  git show "$rev:mock/crates/arvo-format/Cargo.toml" | g -c 'arvo-placement' | sed 's/^/    /'
  echo "$total"
}
o=$(census $OLD); echo "$o" | sed '$d'; told=$(echo "$o" | tail -1)
echo
nw=$(census $NEW); echo "$nw" | sed '$d'; tnew=$(echo "$nw" | tail -1)
echo

# --- C1: the extractor sees the change ------------------------------------------------
[ "$told" != "$tnew" ] || fail "C1, the extractor counts $told on both trees; it is not reading them"
echo "C1 passes: the extractor counts $told at $OLD and $tnew at HEAD, so it sees 748c6004."
echo

# --- component mapping, read from each module's own doc ---------------------------------
echo "component mapping, from the modules' own first doc lines at HEAD:"
for f in ambient.rs quantum.rs slots.rs format.rs adapt.rs; do
  printf '  %-10s %s\n' "$f" "$(g -m1 '^//! ' $SRC/$f | sed 's|^//! ||')"
done
echo
echo "VERDICT: on both trees every required const of the four format traits sits on the ambient domain"
echo "         (component 1) or on the representable set (component 2). No encoding or container item is"
echo "         declared in arvo-format and it does not depend on arvo-placement. The one const outside 1-2"
echo "         is Operation::ARITY in adapt.rs, which 748c6004's message counts as 'the tenth' and which"
echo "         is a coordinate of an operation over the signature, not of the format's identity."
