#!/usr/bin/env nutshell
# Triage every probe row onto `standing = "uncontrolled"` where its control says
# no case that had to fail was run.
#
# THE RULE, written before it is applied, because it decides every borderline:
#
#   `uncontrolled` where the `control` field STATES that no case whose outcome
#   was required in advance exists. Not where I judge the control weak.
#
#   Three things are NOT controls and do not by themselves make a row sound:
#   corroboration by a second instrument, a prerequisite arm, and a comparison
#   whose either outcome would have been informative.
#
#   But a required outcome may be IMPLICIT in what the claim needs. Where the
#   claim only stands if the comparison comes out one way, that way is the
#   required outcome even if the field does not use the word control. Those
#   rows stay `sound` and are listed in the findings as the bucket where the
#   requirement is implicit rather than stated, which is a real weakness and is
#   not this value's business.
#
# So the triage moves rows whose field says it plainly, and nothing else. Nine
# were found by reading all seventy-five in full; the prose matcher in
# `shape.rs` finds seven of the nine and is a backstop rather than the decider,
# which is what its own docstring says.
#
# Required outcomes, written before the run:
#
#   C1  the nine named below must all carry `standing = "sound"` BEFORE the run.
#       If one is already `uncontrolled` the list is stale and the count is wrong.
#   C2  after the edit, exactly nine rows carry `uncontrolled` and the other
#       counts move by exactly that: sound 72 -> 63, uncontrolled 0 -> 9,
#       defective and withdrawn unchanged.
#   C3  every row the prose matcher flags must now carry `uncontrolled`. A row
#       the matcher flags that I left `sound` is a disagreement between the
#       backstop and the data and has to be named rather than left.
#   C4  the reverse direction, which is the whole reason this pass exists: name
#       every row I moved that the matcher does NOT flag. Those are the false
#       negatives the word list cannot reach, and the count is the argument for
#       deleting it.
set -euo pipefail

here="$(cd "$(dirname "$0")" && pwd)"
root="$(cd "$here/../../../.." && pwd)"
toml="$root/mock/registry/probe.toml"
out="$here/p1_triage.out"

# The nine, by id. Each carries the words in its own control field that say it.
MOVE="chain_the_third_definition_is_not_observation_bounded
no_dependent_survives_the_rounding_units_defects
the_bench_tree_was_built_at_the_undocumented_profile
the_debug_release_gap_that_retired_a_true_finding
an_equivalence_checker_that_skips_panics_certifies_a_definedness_difference
chain_accuracy_needs_an_intermediate_wider_than_the_operand_type
the_four_const_available_constructions_bind_at_four_times
a_compile_time_strategy_selection_leaves_no_residue
the_argmin_mechanism_has_never_run_on_arms_that_disagree"

# One record per row: id, standing, control.
rows() {
  awk '
    function flush(){ if(id!=""){ printf "%s\t%s\t%s\n", id, st, ctl } id="";st="";ctl="" }
    /^\[\[probe\]\]/{ flush(); next }
    /^id = /{ id=$0; sub(/id = "/,"",id); sub(/"$/,"",id); next }
    /^standing = /{ st=$0; sub(/.*= "/,"",st); sub(/"$/,"",st); next }
    /^control = /{ ctl=$0; sub(/control = "/,"",ctl); sub(/"$/,"",ctl); next }
    END{ flush() }
  ' "$toml"
}

# The prose matcher, reimplemented from `shape.rs` so the comparison is against
# what the checker does rather than against what I remember it doing. The three
# PLAINLY phrases, then the opening-word rule with the FIRED carve-out.
PLAINLY='no control|nothing was run|no case that had to fail'
FIRED='fired|disagreed|refused|failed|reported'
matcher_says_none() {
  local c; c=$(printf '%s' "$1" | tr 'A-Z' 'a-z')
  printf '%s' "$c" | grep -qE "$PLAINLY" && { printf 'none'; return; }
  case "$c" in
    none[!a-z0-9]*) ;;
    *) printf 'has'; return ;;
  esac
  printf '%s' "$c" | grep -qE "$FIRED" && { printf 'has'; return; }
  printf 'none'
}

{
  printf '=== p1 triage, %s\n\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)"

  printf '## C1: all nine must be `sound` before the edit\n'
  bad=0
  while IFS= read -r m; do
    [ -z "$m" ] && continue
    st=$(rows | awk -F'\t' -v m="$m" '$1==m{print $2}')
    if [ "$st" != sound ]; then printf '  C1 FAIL: %s is already `%s`\n' "$m" "${st:-ABSENT}"; bad=$((bad+1)); fi
  done <<< "$MOVE"
  [ "$bad" -eq 0 ] && printf 'C1 PASS: nine rows, all `sound`\n'
  printf '\n'

  printf '## before\n'
  rows | cut -f2 | sort | uniq -c | sed 's/^/  /'
  printf '\n'

  printf '## what the prose matcher says about each of the nine\n'
  miss=0
  while IFS= read -r m; do
    [ -z "$m" ] && continue
    ctl=$(rows | awk -F'\t' -v m="$m" '$1==m{print $3}')
    v=$(matcher_says_none "$ctl")
    if [ "$v" = none ]; then printf '  matcher agrees   %s\n' "$m"
    else printf '  MATCHER MISSES   %s\n' "$m"; miss=$((miss+1)); fi
  done <<< "$MOVE"
  printf 'C4: the matcher misses %s of the nine by reading alone\n\n' "$miss"

  printf '## applying\n'
  while IFS= read -r m; do
    [ -z "$m" ] && continue
    awk -v want="$m" '
      /^\[\[probe\]\]/ { cur="" }
      /^id = /         { cur=$0; sub(/id = "/,"",cur); sub(/"$/,"",cur) }
      /^standing = "sound"$/ && cur==want { print "standing = \"uncontrolled\""; next }
      { print }
    ' "$toml" > "$toml.new" && mv "$toml.new" "$toml"
    printf '  moved %s\n' "$m"
  done <<< "$MOVE"
  printf '\n'

  printf '## after\n'
  rows | cut -f2 | sort | uniq -c | sed 's/^/  /'
  n_unc=$(rows | awk -F'\t' '$2=="uncontrolled"' | wc -l | tr -d ' ')
  printf '\n## C2: exactly nine uncontrolled\n'
  if [ "$n_unc" -eq 9 ]; then printf 'C2 PASS\n'; else printf 'C2 FAIL: %s\n' "$n_unc"; fi
  printf '\n'

  printf '## C3: every row the matcher flags must now be `uncontrolled`\n'
  dis=0
  while IFS=$(printf '\t') read -r id st ctl; do
    v=$(matcher_says_none "$ctl")
    if [ "$v" = none ] && [ "$st" != uncontrolled ]; then
      printf '  DISAGREEMENT: matcher says none, row says `%s`: %s\n' "$st" "$id"; dis=$((dis+1))
    fi
  done < <(rows)
  if [ "$dis" -eq 0 ]; then printf 'C3 PASS: backstop and data agree in that direction\n'
  else printf 'C3 FAIL: %s rows\n' "$dis"; fi
} > "$out" 2>&1
cat "$out"
