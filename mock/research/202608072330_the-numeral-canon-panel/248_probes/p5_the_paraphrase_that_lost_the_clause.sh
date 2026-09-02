#!/usr/bin/env nutshell
# p5. Where the permissive reading of `standing` came from.
#
# Written after the blind commit, for the reconciliation.
#
# `246` and `247` both quote `proposal.standing` as recording "how many
# independent instances back the claim" and both conclude the canon does not say
# what an instance is. That wording is not the schema's. The schema says the
# instances "reached it", and spells `two_experts` as "each deriving before
# reading the other", which is the clause that decides the question both files
# hand back.
#
# This locates the wording each of them read.
#
# TWO DEFECTS FOUND IN THIS SCRIPT ON ITS FIRST TWO RUNS, KEPT RATHER THAN DELETED.
#
# D1. `hits()` was `grep -c "$1" "$2" 2>/dev/null || echo 0`. `grep -c` prints
#     `0` and ALSO exits 1 when nothing matches, so the fallback fired on top of
#     the printed zero and every no-match cell emitted two lines. C2 then
#     compared `0\n0` as an integer and reported FAIL for a reason unrelated to
#     its subject. Caught because C2 stated its expectation before the run and
#     its failure did not look like its claim.
#
# D2. **The panel-file column returned a clean zero while broken, and no control
#     covered it.** The paraphrase is line-wrapped in both files: `246` writes
#     "back a claim" and `247` writes "back\nthe claim" across a newline, so a
#     fixed-string grep for either finds nothing in either. C3 guarded the agent
#     surfaces and nothing guarded the panel column, so a column whose whole
#     purpose was to show the two seats reading the paraphrase reported that
#     neither did. Found by reading the output against a `grep -rn` run by hand
#     before this script existed, which is not a method. **C5 is the repair**:
#     the panel column now normalises whitespace and an arm requires it non-zero.
use log

ROOT="$(git rev-parse --show-toplevel)"
cd "$ROOT" || exit 1
P="mock/research/202608072330_the-numeral-canon-panel"

SCHEMA_CLAUSE='each deriving before reading the other'
PARAPHRASE='independent instances back'
KNOWN_IN_AGENT='standing'

# Line-oriented, for files where the string is known to sit on one line.
hits() { local n; n=$(grep -c "$1" "$2" 2>/dev/null); [ -z "$n" ] && n=0; echo "$n"; }
# Whitespace-normalised, for prose that wraps. D2.
whits() { tr '\n' ' ' < "$2" | tr -s ' ' | grep -o "$1" | wc -l | tr -d ' '; }

echo "== where each wording lives (line-oriented) =="
for f in mockspace.toml mock/agent/MAIN.md.tmpl .claude/CLAUDE.md "$P/AGREEMENTS.md"; do
  printf "  %-44s clause x%s   paraphrase x%s\n" "$(basename "$f")" "$(hits "$SCHEMA_CLAUSE" "$f")" "$(hits "$PARAPHRASE" "$f")"
done
echo
echo "== the two panel files (whitespace-normalised, D2) =="
for f in 246_kiselyov_the_two_promotions_and_what_they_second 247_leroy_the_cold_seats_answered_rows_73_wrote; do
  printf "  %-44s clause x%s   paraphrase x%s\n" "${f:0:3}" "$(whits "$SCHEMA_CLAUSE" "$P/$f.md")" "$(whits "$PARAPHRASE" "$P/$f.md")"
done
echo

echo "== the schema's own sentence, printed rather than described =="
grep -A3 '^name = "standing"' mockspace.toml | grep '^description' | tail -1 | fold -w 96 -s | sed 's/^/  /'
echo
echo "== what the agent instructions say instead =="
grep -n -A1 "$PARAPHRASE" mock/agent/MAIN.md.tmpl | sed 's/^/  /'
echo

fail=0
chk() { if [ "$2" = "$3" ]; then echo "  PASS $1"; else echo "  FAIL $1: expected $2, got $3"; fail=1; fi }

echo "== the cases that had to fail =="
[ "$(hits "$SCHEMA_CLAUSE" mockspace.toml)" -ge 1 ] && r=yes || r=no
chk "C1 the schema clause exists in mockspace.toml" yes "$r"

A=$(whits "$SCHEMA_CLAUSE" mock/agent/MAIN.md.tmpl); B=$(whits "$SCHEMA_CLAUSE" .claude/CLAUDE.md)
[ "$A" -eq 0 ] && [ "$B" -eq 0 ] && r=yes || r=no
chk "C2 the clause is absent from both agent surfaces, wrapping allowed for" yes "$r"

C3a=$(hits "$KNOWN_IN_AGENT" mock/agent/MAIN.md.tmpl); C3b=$(hits "$KNOWN_IN_AGENT" .claude/CLAUDE.md)
[ "$C3a" -ge 1 ] && [ "$C3b" -ge 1 ] && r=yes || r=no
chk "C3 the same grep finds a string known to be in both ($C3a / $C3b)" yes "$r"

C4=$(grep -c "^### .*$SCHEMA_CLAUSE" "$P/AGREEMENTS.md")
[ "$C4" -ge 1 ] && r=yes || r=no
chk "C4 AGREEMENTS.md carries the clause as a section heading ($C4)" yes "$r"

# D2's repair. The panel column exists to show the two seats reading the
# paraphrase; a zero there is the instrument broken, not a result.
C5a=$(whits "$PARAPHRASE" "$P/246_kiselyov_the_two_promotions_and_what_they_second.md")
C5b=$(whits "$PARAPHRASE" "$P/247_leroy_the_cold_seats_answered_rows_73_wrote.md")
[ "$C5a" -ge 1 ] && [ "$C5b" -ge 1 ] && r=yes || r=no
chk "C5 both panel files carry the paraphrase ($C5a / $C5b)" yes "$r"
echo

echo "== the contested tier, since the ledger has one and the registry does not =="
LEDGER=$(grep -c "^### .*[Cc]ontested" "$P/AGREEMENTS.md")
CN=$(grep -h 'standing = "contested"' mock/registry/proposal.toml mock/registry/proposal-the-later-topics.toml | wc -l | tr -d ' ')
echo "  \"Contested or located\" sections in AGREEMENTS.md: $LEDGER"
echo "  proposal rows at standing=\"contested\":            $CN"
echo "  This says none of the ledger's contested material arrived as a proposal"
echo "  at that value. It may have arrived as a retirement or a question row;"
echo "  this instrument does not look and does not claim otherwise."
exit $fail
