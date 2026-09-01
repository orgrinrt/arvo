#!/usr/bin/env bash
# What this file establishes, and what it does not.
#
# Every check below is a read over committed text: `grep`, `awk`, `sed`. NOTHING
# HERE RUNS THE COVERAGE TOOL. The dispatch that produced `235` was read-only in
# a worktree belonging to another seat and was told not to invoke cargo, so the
# tiering claims in that file are established by reading the tool's source and
# by walking the same registry edges the tool walks, never by executing it. Where
# `235` says the tool reports something, that is the dispatcher's quoted run plus
# my reading of `lib.rs`, and it is written as such.
#
# Each zero-returning check carries a positive control immediately after it,
# because a zero from a pipeline is a claim about the pipeline until the
# pipeline has been shown able to return anything else.
#
# Run from the repository root:  bash <this file>
set -uo pipefail

R=mock/registry
T=mock/tools/obligation-coverage/src
C=mock/crates

hdr() { printf '\n\n===== %s =====\n' "$1"; }

hdr "1. the tool never reads a rung"
echo "-- occurrences of 'rung' in the tool source and its tests:"
grep -c 'rung' "$T/lib.rs" "$T/tests.rs"
echo "-- POSITIVE CONTROL, the same pattern against a file that does carry it:"
grep -c 'rung' "$R/ruling.toml"
echo "-- the tiering table itself:"
sed -n '96,100p' "$T/lib.rs"
echo "-- the walk, with no filter between rows_in and the tier assignment:"
sed -n '136,143p' "$T/lib.rs"
echo "-- what the doc comment claims the Met tier means:"
sed -n '35,37p' "$T/lib.rs"; sed -n '66,69p' "$T/lib.rs"

hdr "2. every ruling -> obligation edge in the repository, and its rung"
echo "-- count and line:"
grep -n '^obligation = ' "$R/ruling.toml"
echo "-- the row that line belongs to, id and rung:"
awk '/^\[\[ruling\]\]/{id="";rung=""} /^id = /{if(id=="")id=$0} /^rung = /{if(rung=="")rung=$0} /^obligation = /{print "   "id; print "   "rung}' "$R/ruling.toml"
echo "-- op's words on that row, from its own note:"
grep -o 'I need elaboration on the Ingest vs C ABI[^"]*' "$R/ruling.toml"

hdr "3. the rung population, and ratification by proxy"
echo "-- rulings by rung:"
awk '/^rung = /{print $0}' "$R/ruling.toml" | sort | uniq -c
echo "-- ratified rulings and the proposals each ratifies:"
awk '/^\[\[ruling\]\]/{id="";rung=""} /^id = /{if(id=="")id=$0} /^rung = /{if(rung=="")rung=$0} /^ratifies = /{print "--- "id" | "rung; inr=1; next} inr{print; if($0 ~ /\]$/) inr=0}' "$R/ruling.toml"
echo "-- the proposal carrying an obligation edge that a ratified ruling ratifies:"
grep -n 'every_operation_arvo_declares_is_a_function_of_the_declared_width' "$R"/*.toml
echo "-- and the region that ratified claim was established over:"
awk '/^id = "every_operation_arvo_declares_is_a_function_of_the_declared_width"/{p=1} p&&/^predicate = /{inr=1} inr{print; if($0 ~ /^\]$/){exit}}' "$R/proposal-the-later-topics.toml"

hdr "4. the question file's header against its own rows"
echo "-- what the header says:"
sed -n '16,19p' "$R/question.toml"
echo "-- question rows, and rows carrying an answer:"
printf 'total   %s\n' "$(grep -c '^\[\[question\]\]' "$R/question.toml")"
printf 'answered %s\n' "$(grep -c '^answered = ' "$R/question.toml")"
echo "-- which ones:"
awk '/^\[\[question\]\]/{id="";a=0} /^id = /{if(id=="")id=$0} /^answered = /{a=1} /^keywords = /{if(a)print "   "id}' "$R/question.toml"

hdr "5. no ratified says reaches the algorithm, hashing or rendering subjects"
awk '/^\[\[ruling\]\]/{p=0;buf="";id=""} {buf=buf $0 "\n"} /^id = /{if(id=="")id=$0} /^rung = "ratified"/{p=1} /^keywords = /{if(p){n=split(buf,L,"\n"); for(i=1;i<=n;i++) if(L[i] ~ /^says = / && L[i] ~ /hash|graph|topolog|bitmask|bit set|Debug|fmt|sparse|spectral|dynamic program/) print id} buf="";id=""}' "$R/ruling.toml" | wc -l
echo "-- POSITIVE CONTROL, the same walk with a term the ratified set does carry:"
awk '/^\[\[ruling\]\]/{p=0;buf="";id=""} {buf=buf $0 "\n"} /^id = /{if(id=="")id=$0} /^rung = "ratified"/{p=1} /^keywords = /{if(p){n=split(buf,L,"\n"); for(i=1;i<=n;i++) if(L[i] ~ /^says = / && L[i] ~ /declared width/) print id} buf="";id=""}' "$R/ruling.toml"

hdr "6. the width surface was picked in code, and no round cites the open question"
echo "-- Q9's options, which is what was picked from:"
awk '/^id = "the_width_surface_crossing"/{p=1} p&&/^options = /{inr=1} inr{print; if($0 ~ /^\]$/){exit}}' "$R/question.toml"
echo "-- what the tree does:"
grep -n 'pub struct Signed<const\|pub struct Unsigned<const\|^admit_widths!' "$C/arvo-format/src/slots.rs"
grep -n 'pub struct Integer<const\|pub struct UFixed<const\|pub struct Biased<const\|pub struct Floating<const' "$C/arvo-format/src/lib.rs"
echo "-- citations of that question row anywhere in rounds or crates:"
grep -rho 'question::the_width_surface_crossing' mock/design_rounds mock/crates 2>/dev/null | wc -l
echo "-- POSITIVE CONTROL, question rows those same trees do cite:"
grep -rho 'question::[a-z_]*' mock/design_rounds mock/crates 2>/dev/null | sort | uniq -c | sort -rn
echo "-- the discipline, applied correctly one crate over:"
sed -n '25,27p' "$C/arvo-strategy/src/lib.rs"

hdr "7. a locked changelist's justification, against the file it licenses"
echo "-- the claim:"
sed -n '102,105p' mock/design_rounds/202608311902/202608311902_changelist.src.lock.md
echo "-- associated consts of a non-primitive type, in the crate that claim is about:"
grep -n 'const [A-Z_]*: Width' "$C/arvo-format/src/slots.rs" "$C/arvo-placement/src/lib.rs" | grep -v tests

hdr "8. that changelist's totality claim about value positions"
echo "-- public functions returning a bare host primitive, all of them, then the two that width.rs declares as its unwrap doors, then the rest:"
grep -n 'pub const fn .*-> \(bool\|i64\|i32\|u32\)' "$C"/arvo-format/src/*.rs | grep -v 'tests.rs' | wc -l
grep -n 'pub const fn .*-> \(bool\|i64\|i32\|u32\)' "$C"/arvo-format/src/*.rs | grep -v 'tests.rs'
echo "-- public functions returning the stack's own Bool:"
grep -cn 'pub const fn .*-> Bool' "$C"/arvo-format/src/width.rs

hdr "9. the tool's suite never plants a rung"
echo "-- the fixture every ruling in every test comes from:"
sed -n '57,72p' "$T/tests.rs"
echo "-- the test that pins the behaviour:"
sed -n '74,83p' "$T/tests.rs"
echo "-- test count:"
grep -c '^#\[test\]' "$T/tests.rs"

hdr "10. the two-expert union claim"
grep -o 'the deciding fact is that the two instances carry disjoint axes[^"]*' "$R/ruling.toml"
grep -o 'So the two agree over the union of what each covered[^\\]*' "$R/ruling.toml"
echo "-- what the governing rule says the tier is over:"
RULE=.claude/rules/expert-dispatch-defends-the-canon.md; [ -f "$RULE" ] || RULE=../../.claude/rules/expert-dispatch-defends-the-canon.md; echo "(read from $RULE)"; grep -o "the tier is over their intersection[^.]*." "$RULE"


hdr "8b. the same count split by whether the position is a declared door"
G='pub const fn .*-> \(bool\|i64\|i32\|u32\)'
echo -n "all public bare-primitive returns in arvo-format: "
grep -h "$G" "$C"/arvo-format/src/*.rs | grep -vc 'tests.rs'
echo -n "of those, in width.rs, which documents each as 'the unwrap door, declared as one': "
grep -hc "$G" "$C"/arvo-format/src/width.rs
echo -n "outside width.rs, where no door is declared: "
grep -h "$G" "$C"/arvo-format/src/format.rs "$C"/arvo-format/src/slots.rs "$C"/arvo-format/src/quantum.rs "$C"/arvo-format/src/apply.rs | wc -l
echo -n "public functions in the whole crate returning the stack's own Bool: "
grep -h 'pub const fn .*-> Bool' "$C"/arvo-format/src/*.rs | wc -l

hdr "done"
