#!/usr/bin/env bash
# Seat 247. Which source facts in 244, 245 and 246 are facts about a tree that
# had already moved, or has moved since.
#
# 246 predicates every source claim on "tree a12d4d5d". That is a tree hash, the
# tree of commit 033c02e2. Two things this instrument establishes about it:
#
#   (1) 748c6004, "spell every format coordinate in a type the stack owns", and
#       246's first commit 5fd3134a are BOTH children of 033c02e2, merged at
#       8064a454. 246 never saw 748c6004, so its coordinate spellings (PHASE_NUM
#       and PHASE_DEN, a u32 RADIX) are honest about a12d4d5d and stale at HEAD.
#
#   (2) The obligations at Ambient, Quantum and Format landed at da2f9d23
#       (2026-09-02 00:06), which is NOT in 244's tree 800e120a and IS in 245's
#       tree 98a4b7ee and 246's tree a12d4d5d. So 244's A4, "the admission
#       obligation exists at Slots and at no tier above it", was true at 244's
#       tree and false at the trees of the two seats that carried it forward.
#       245 wrote that none of the growth between the trees touched the admission
#       topic; this is the commit that did.
#
# THE CASE THAT MUST FAIL: something that did NOT move must read the same on the
# trees compared, or the instrument reports change everywhere. Slots::ADMITTED
# carries five assert! calls at 800e120a, at a12d4d5d and at HEAD.
#
# The first version of this file asserted 748c6004 was an ancestor of 246's first
# commit, and its own ordering control refused. The second version assumed only
# Slots carried ADMITTED at a12d4d5d, and its census said four. Both corrections
# came from the instrument rather than from rereading the files.
set -u
cd "$(dirname "$0")/../../../.." || exit 1   # the repo root
fail() { echo "CONTROL FAILED: $1"; exit 2; }
g() { command grep "$@"; }
SRC=mock/crates/arvo-format/src
T244=800e120a; T245=98a4b7ee; OLD=033c02e2; MOVE=748c6004; S246=5fd3134a; MERGE=8064a454; OBL=da2f9d23

slots_asserts()   { git show "$1:$SRC/slots.rs" | awk '/const ADMITTED: \(\) = \{/{p=1} p&&/^    \};/{p=0} p' | g -c 'assert!'; }
admitted_traits() { for f in ambient quantum slots format; do git show "$1:$SRC/$f.rs" | g -q '^    const ADMITTED: ()' && printf '%s ' "$f"; done; echo; }
phase_consts()    { git show "$1:$SRC/format.rs"  | g -oE '^    const PHASE[A-Z_]*: [A-Za-z0-9]+;' | sed 's/^    //' | tr '\n' ' '; }
radix_type()      { git show "$1:$SRC/ambient.rs" | g -oE '^    const RADIX: [A-Za-z0-9]+;' | sed 's/^    //'; }

# --- the control first -----------------------------------------------------------------
for t in $T244 $OLD HEAD; do
  [ "$(slots_asserts $t)" = "5" ] || fail "control, Slots::ADMITTED assert count at $t is $(slots_asserts $t), expected 5"
done
echo "control passes: Slots::ADMITTED carries 5 assert! calls at $T244, at a12d4d5d and at HEAD, so 'unchanged' is readable."
echo

echo "(1) 246's tree, and the source commit it never saw"
echo "    tree of $OLD : $(git rev-parse $OLD^{tree} | cut -c1-8)  (246 cites a12d4d5d)"
[ "$(git rev-parse $OLD^{tree} | cut -c1-8)" = "a12d4d5d" ] || fail "(1) a12d4d5d is not the tree of $OLD"
for c in $OLD $MOVE $S246 $MERGE HEAD; do echo "    $(git log -1 --format='%h parents:%p  %ad  %s' --date=iso $c)"; done
git merge-base --is-ancestor $OLD $MOVE  || fail "(1) $MOVE does not descend from $OLD"
git merge-base --is-ancestor $OLD $S246  || fail "(1) $S246 does not descend from $OLD"
if git merge-base --is-ancestor $MOVE $S246; then fail "(1) $MOVE is an ancestor of 246's first commit"; fi
[ "$(git rev-parse $MERGE^1)" = "$(git rev-parse $MOVE)" ] && [ "$(git rev-parse $MERGE^2)" = "$(git rev-parse $S246)" ] || fail "(1) $MERGE is not the merge of $MOVE and $S246"
echo "    $MOVE and $S246 are siblings off $OLD; $MERGE merges them. 246 did not see $MOVE."
printf '    %-22s %-36s %s\n' "" "at a12d4d5d" "at HEAD ($(git rev-parse --short HEAD))"
printf '    %-22s %-36s %s\n' "Format phase consts" "$(phase_consts $OLD)" "$(phase_consts HEAD)"
printf '    %-22s %-36s %s\n' "Ambient::RADIX" "$(radix_type $OLD)" "$(radix_type HEAD)"
echo

echo "(2) when the obligations above Slots landed, against the three seats' trees"
echo "    $(git log -1 --format='%h  %ad  %s' --date=iso $OBL)"
printf '    %-30s %-22s %s\n' "tree" "ADMITTED on" "$OBL in tree?"
for pair in "244:$T244" "245:$T245" "246:$OLD" "247:HEAD"; do
  seat=${pair%%:*}; t=${pair##*:}
  if git merge-base --is-ancestor $OBL $t; then in=yes; else in=no; fi
  printf '    %-30s %-22s %s\n' "$seat at $(git rev-parse --short $t)" "$(admitted_traits $t)" "$in"
done
git merge-base --is-ancestor $OBL $T244 && fail "(2) $OBL is in 244's tree; the A4 timeline claim is wrong"
git merge-base --is-ancestor $OBL $T245 || fail "(2) $OBL is not in 245's tree"
git merge-base --is-ancestor $OBL $OLD  || fail "(2) $OBL is not in 246's tree"
echo
echo "what this makes stale, by file and section:"
echo "  244 A4   'the admission obligation exists at Slots and at no tier above it' : true at $T244, false at every later tree"
echo "  245 gate 'none of the growth touches the admission topic'                   : $OBL is in that growth and adds the three obligations 242 measured missing"
echo "  246 5.2  'the shipped mechanism enforces all three through one const'        : four consts at a12d4d5d already"
echo "  246 6,O4 carry 244's C5, 'what would decide the fourth option: a second independent derivation'   : the three were enforced at a12d4d5d already"
echo "  246 2.2  'Format declares PHASE_NUM and PHASE_DEN'                            : true at a12d4d5d, one PHASE of type Phase at HEAD"
