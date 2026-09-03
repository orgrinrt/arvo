#!/usr/bin/env bash
# Seat 258. Where the sitting's principal answer lives, and where it does not.
#
# 248's F9: the schema declares `two_experts` as "each deriving before reading the
# other"; `mock/agent/MAIN.md.tmpl` carried a paraphrase dropping that clause, and
# the paraphrase is what generates into `.claude/CLAUDE.md` and loads at the start
# of every session.
#
# Two things are measured here and they point opposite ways.
#
# The paraphrase is repaired. The template quotes the declaration and adds the
# strict gloss the sitting derived, that a seat handed another's conclusion has
# not reached anything and no quality of argument makes it count. A hard-error
# lint compares the template against the declaration in both directions.
#
# And the registry carries none of it. `standing` is declared in `mockspace.toml`,
# which `canon_paths` does not glob, and the answer to 247's O1 now binds every
# agent from a generated instruction file, which is not canon either. So the
# sentence that decides whether a claim is promotable is enforced everywhere and
# citeable by no slug.
#
# The second half of the first measurement is a defect in my own instrument, kept
# because it is the same defect 248's p5 disclosed about itself. A fixed-string
# grep for the clause over the template returns zero, because the template wraps
# the sentence and the clause spans a line break. Zero there reads exactly like
# "the clause is absent", which is the wrong answer, and it is the answer my first
# check gave.
#
# THE CASES THAT MUST FAIL, run before the verdict is reported:
#   C1  The unnormalised grep must return zero while the normalised one returns
#       one, or the wrap is not what separates them and the finding is something
#       else.
#   C2  A clause known absent must return zero under normalisation too, or
#       normalisation is matching anything.
#   C3  The declaration in `mockspace.toml` must itself contain the clause, or the
#       comparison has no left-hand side.
#   C4  The lint that holds the two in step must exist and must be declared at
#       `error`, or nothing keeps the repair.
#   C5  The registry search must find a phrase that is in the registry, or a zero
#       from it is a fact about the search.
set -u
cd "$(dirname "$0")/../../../.." || exit 1 # the repository root
fail() {
	echo "CONTROL FAILED: $1"
	exit 2
}

CLAUSE='each deriving before reading the other'
GLOSS='has not reached anything'
ABSENT='each deriving after reading the other'
TMPL=mock/agent/MAIN.md.tmpl
norm() { tr '\n' ' ' <"$1" | tr -s ' '; }

echo "tree: $(git rev-parse HEAD)"
echo

# --- C3 ------------------------------------------------------------------------
grep -qF "$CLAUSE" mockspace.toml || fail "C3: the clause is not in mockspace.toml"
echo "C3 the clause is in mockspace.toml                     : yes"

# --- C1 ------------------------------------------------------------------------
raw=$(grep -cF "$CLAUSE" "$TMPL" || true)
n=$(norm "$TMPL" | grep -cF "$CLAUSE" || true)
[ "$raw" -eq 0 ] || fail "C1: the unnormalised grep found it, so the wrap is not the separator"
[ "$n" -ge 1 ] || fail "C1: the normalised grep did not find it either, so the clause is genuinely absent"
echo "C1 unnormalised grep over the template                 : $raw   (must be 0)"
echo "C1 whitespace-normalised grep over the template        : $n   (must be >= 1)"

# --- C2 ------------------------------------------------------------------------
a=$(norm "$TMPL" | grep -cF "$ABSENT" || true)
[ "$a" -eq 0 ] || fail "C2: a clause known absent was found under normalisation"
echo "C2 a clause known absent, under normalisation          : 0"
echo

# --- the old paraphrase --------------------------------------------------------
for p in 'records how many independent instances back the claim' 'records how many independent instances back a claim'; do
	c=$(norm "$TMPL" | grep -cF "$p" || true)
	echo "old paraphrase, \"$p\" : $c"
done

# --- C4 ------------------------------------------------------------------------
echo
[ -f mock/lints/the_agent_surface_quotes_the_schema.rs ] || fail "C4: the lint does not exist"
grep -q '\[lints.the-agent-surface-quotes-the-schema\]' mockspace.toml || fail "C4: the lint is not declared"
echo "C4 the lint exists and is declared                     : yes"
sed -n '/\[lints.the-agent-surface-quotes-the-schema\]/,/^$/p' mockspace.toml | sed 's/^/    /'
echo "the repair commit:"
git log --format='    %h %ad %s' --date=short -- mock/lints/the_agent_surface_quotes_the_schema.rs | tail -1

# --- C5 and the registry arm ---------------------------------------------------
echo
KNOWN='the coordinator holds the gate'
k=$(cat mock/registry/*.toml | tr '\n' ' ' | tr -s ' ' | grep -cF "$KNOWN" || true)
[ "$k" -ge 1 ] || fail "C5: a phrase known to be in the registry was not found"
echo "C5 a phrase known present in the registry              : $k   (must be >= 1)"
for p in "$CLAUSE" "$GLOSS" "mockspace.toml" "canon_paths"; do
	c=$(cat mock/registry/*.toml | tr '\n' ' ' | tr -s ' ' | grep -cF "$p" || true)
	printf 'registry rows carrying "%s" : %s\n' "$p" "$c"
done
echo
echo "the word \`standing\` in ruling.toml, and what each occurrence is:"
grep -in 'standing' mock/registry/ruling.toml | cut -c1-150 | sed 's/^/    /'
echo
echo "the template's own words, as they stand:"
sed -n '57,73p' "$TMPL" | sed 's/^/    /'
echo
echo "VERDICT: 248's F9 names two things and only one survives. The lossy paraphrase"
echo "is repaired and a hard-error lint keeps it repaired, and the strict reading the"
echo "sitting derived is now in the instruction every session loads. The locus half is"
echo "untouched and is one layer worse than 248 measured it: neither the declaration"
echo "nor the answer to it is a row, so the sentence that decides a promotion binds"
echo "every agent and can be cited by no slug."
