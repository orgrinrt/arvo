#!/usr/bin/env nutshell
# The ratio the dispatch asks for, counted over the rows rather than estimated.
#
# How many `probe` rows carry a control that fired, against how many carry none.
# Classified from the `control` field's own opening, which is written to a fixed
# convention for exactly this reason: a row where no control was run opens with
# the word `None`. That is a convention rather than a measurement, so this
# instrument reports what the convention says and the convention is checkable by
# reading the field.
#
# Required outcomes, written before the run:
#
#   C1  the three classes must partition the rows: with + without == total. A row
#       counted twice or dropped makes the ratio arithmetic on nothing.
#   C2  both classes must be non-empty. A classifier returning all-of-one is
#       reporting its own default.
#   C3  the standing tally must partition too, over the three schema values and
#       no fourth. A fourth value would mean the schema's enum is not enforced,
#       which is a finding about the schema rather than about the rows.
set -euo pipefail

here="$(cd "$(dirname "$0")" && pwd)"
repo="$(cd "$here/../../../.." && pwd)"
toml="$repo/mock/registry/probe.toml"
out="$here/p8_the_rows_control_ratio.out"

{
  printf '=== p8 the control ratio over the committed probe rows, %s\n\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)"

  total=$(grep -c '^\[\[probe\]\]' "$toml")
  ncontrol=$(grep -c '^control = ' "$toml")
  none=$(grep -c '^control = "None' "$toml")
  with=$((ncontrol - none))

  printf '## the ratio\n'
  printf 'rows:                                          %s\n' "$total"
  printf 'carrying a control field (schema requires it): %s\n' "$ncontrol"
  printf 'whose control field opens with None:           %s\n' "$none"
  printf 'naming a case that was run:                    %s\n' "$with"
  awk -v w="$with" -v t="$total" 'BEGIN{printf "share naming a run case: %.0f%%\n", 100*w/t}'
  printf '\n'

  printf '## C1: the classes must partition\n'
  if [ "$((none + with))" -eq "$total" ] && [ "$ncontrol" -eq "$total" ]; then
    printf 'C1 PASS: %s + %s = %s, and every row carries the field\n' "$with" "$none" "$total"
  else
    printf 'C1 FAIL: %s + %s against %s rows and %s control fields\n' "$with" "$none" "$total" "$ncontrol"
  fi
  printf '\n'
  printf '## C2: both classes non-empty\n'
  if [ "$none" -gt 0 ] && [ "$with" -gt 0 ]; then printf 'C2 PASS\n'
  else printf 'C2 FAIL: one class is empty, so the classifier reports its own default\n'; fi
  printf '\n'

  printf '## C3: standing, over the three schema values and no fourth\n'
  grep '^standing = ' "$toml" | sed 's/standing = //' | sort | uniq -c
  extra=$(grep '^standing = ' "$toml" | sed 's/standing = "//; s/"//' \
    | grep -vxE 'sound|defective|withdrawn' | sort -u | grep -c . || true)
  if [ "$extra" -eq 0 ]; then printf 'C3 PASS: no value outside the declared enum\n'
  else printf 'C3 FAIL: %s value(s) outside the enum, so the schema is not enforcing it\n' "$extra"; fi
  printf '\n'

  printf '## rows carrying a recorded defect\n'
  printf 'rows with a `defect` field: %s\n' "$(grep -c '^defect = ' "$toml")"
  printf 'rows with a `reproduced` field: %s\n' "$(grep -c '^reproduced = ' "$toml")"
  printf '\n'

  printf '## the rows whose control field opens with None\n'
  awk '/^\[\[probe\]\]/{id=""} /^id = /{id=$0} /^control = "None/{print "  " id}' "$toml"
  printf '\n'
  printf '## is 90 percent a fact about the corpus or about the selection\n'
  printf 'Almost certainly the selection: the rows here were chosen as the\n'
  printf 'instruments a live claim needs plus every instrument whose defect the\n'
  printf 'corpus records, and both populations are files whose authors were\n'
  printf 'paying attention. Checked rather than assumed, by splitting p4 own\n'
  printf 'per-file classification on whether the directory has a row here.\n\n'
  cited=$(grep -oE '::[0-9]+_probes::' "$toml" | tr -d ':' | sort -u)
  ptsv="$here/p4_who_states_a_control.tsv"
  if [ ! -f "$ptsv" ]; then
    printf 'p4 has not run; this arm needs its tsv and says so rather than guessing\n'
  else
    printf '%s\n' "$cited" > "$here/.p8_cited_dirs"
    awk -F'\t' 'NR==FNR{c[$0]=1; next}
      {split($1,a,"/"); d=a[1]; grp = (d in c) ? "represented" : "not_represented";
       tot[grp]++; if ($2=="stated") st[grp]++}
      END{for (g in tot) printf "%-16s %4d of %4d state a control  (%.1f%%)\n",
            g, st[g]+0, tot[g], 100*(st[g]+0)/tot[g]}' \
      "$here/.p8_cited_dirs" "$ptsv" | sort
    rm -f "$here/.p8_cited_dirs"
    printf '\n'
    printf 'C4: the two groups must both be non-empty and must sum to p4 total,\n'
    printf 'or the split is dropping files.\n'
    awk -F'\t' 'END{printf "p4 files: %d\n", NR}' "$ptsv"
  fi
  printf '\n'
  printf 'WHAT THIS DOES NOT SAY. It counts what the field claims, not what the\n'
  printf 'instrument did. A row saying a control fired is only as good as the\n'
  printf 'reading behind it, and four defects in this corpus were controls that\n'
  printf 'could not fail and would be counted here as fired.\n'
} > "$out" 2>&1
cat "$out"
