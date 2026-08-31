#!/usr/bin/env nutshell
# The bucket: rows where I could not tell whether a control was stated.
#
# The triage moved rows whose field SAYS no case that had to fail was run. That
# leaves a third group between those and the clean ones: rows whose field
# describes something the author offered as a control but never writes down an
# outcome that was required in advance. Those are where a reading has to be made
# and where a second reader is most likely to disagree with me.
#
# Found mechanically rather than asserted, so the bucket is the residue of two
# classifications rather than my list of rows I feel uneasy about:
#
#   admission  the field says none was run          -> already `uncontrolled`
#   required   the field states an outcome that was demanded in advance, in the
#              corpus's own vocabulary: must, had to, or a construction naming
#              what a different result would have meant
#   bucket     neither
#
# Required outcomes, written before the run:
#
#   C1  a row whose field plainly says `must` has to land in `required`.
#       `packing_is_a_weighting_not_a_policy` writes "which must differ, and
#       does", so it must not be in the bucket.
#   C2  no row already at `uncontrolled` may land in the bucket. They are
#       classified and the bucket is for rows that are not.
#   C3  the bucket must be non-empty and smaller than the corpus. Empty means
#       the requirement pattern matches everything and the bucket is hidden;
#       everything means it matches nothing.
set -euo pipefail

here="$(cd "$(dirname "$0")" && pwd)"
root="$(cd "$here/../../../.." && pwd)"
toml="$root/mock/registry/probe.toml"
out="$here/p3_where_the_requirement_is_implicit.out"
tsv="$here/p3_bucket.tsv"

# Run one used only the demand words (`must`, `had to`, `required`) and put 31
# of 79 in the bucket, most of them rows that state a requirement in the
# corpus's other vocabulary: a control that FIRED was a control, a NEGATIVE ARM
# is one, a MUTANT the construction must reject is one. The bucket was inflated
# by the pattern rather than populated by the corpus, and the rows worth reading
# were buried in it. Kept as `p3_run1_requirement_vocabulary_too_narrow.out`.
#
# Widened to how this corpus actually writes a control that came out one way.
# `disagree` is deliberately NOT here: "the two disagree or they do not" is the
# shape with no required outcome, and admitting the word would classify the
# clearest bucket row as required.
REQUIRED='must |must not|had to|has to|required outcome|which must|would be the case|would say|would have|cannot come out|planted|negative control|negative arm|it fired|fired:|and it fired|refused|could not fail|is the control|are each other|the arm that|separates them|reproduce|refutation|refuted'

awk '
  function flush(){ if(id!=""){ printf "%s\t%s\t%s\n", id, st, ctl } id="";st="";ctl="" }
  /^\[\[probe\]\]/{ flush(); next }
  /^id = /{ id=$0; sub(/id = "/,"",id); sub(/"$/,"",id); next }
  /^standing = /{ st=$0; sub(/.*= "/,"",st); sub(/"$/,"",st); next }
  /^control = /{ ctl=$0; sub(/control = "/,"",ctl); sub(/"$/,"",ctl); next }
  END{ flush() }
' "$toml" > "$tsv.all"

: > "$tsv"
while IFS=$(printf '\t') read -r id st ctl; do
  if [ "$st" = uncontrolled ]; then cls=admission
  elif printf '%s' "$ctl" | grep -qiE "$REQUIRED"; then cls=required
  else cls=BUCKET; fi
  printf '%s\t%s\t%s\n' "$id" "$st" "$cls" >> "$tsv"
done < "$tsv.all"

{
  printf '=== p3 where the requirement is implicit, %s\n\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)"

  printf '## the split\n'
  cut -f3 "$tsv" | sort | uniq -c | sed 's/^/  /'
  printf '\n'

  printf '## C1: a row writing `must differ` must be in `required`\n'
  c1=$(awk -F'\t' '$1=="packing_is_a_weighting_not_a_policy"{print $3}' "$tsv")
  if [ "$c1" = required ]; then printf 'C1 PASS\n'
  else printf 'C1 FAIL: classified `%s`\n' "$c1"; fi
  printf '\n'

  printf '## C2: no `uncontrolled` row in the bucket\n'
  leak=$(awk -F'\t' '$2=="uncontrolled" && $3=="BUCKET"' "$tsv" | wc -l | tr -d ' ')
  if [ "$leak" -eq 0 ]; then printf 'C2 PASS\n'; else printf 'C2 FAIL: %s\n' "$leak"; fi
  printf '\n'

  printf '## C3: the bucket is a proper non-empty subset\n'
  nb=$(awk -F'\t' '$3=="BUCKET"' "$tsv" | wc -l | tr -d ' ')
  nt=$(wc -l < "$tsv" | tr -d ' ')
  if [ "$nb" -gt 0 ] && [ "$nb" -lt "$nt" ]; then printf 'C3 PASS: %s of %s\n' "$nb" "$nt"
  else printf 'C3 FAIL: %s of %s\n' "$nb" "$nt"; fi
  printf '\n'

  printf '## the bucket, with its control field in full, for reading\n'
  while IFS=$(printf '\t') read -r id st cls; do
    [ "$cls" = BUCKET ] || continue
    ctl=$(awk -F'\t' -v i="$id" '$1==i{print $3}' "$tsv.all")
    printf '### %s  [%s]\n%s\n\n' "$id" "$st" "$ctl"
  done < "$tsv"
  printf 'WHAT THIS IS NOT. The pattern reads vocabulary, and a row can demand an\n'
  printf 'outcome without using any of these words. So the bucket is where to\n'
  printf 'look rather than a verdict, and the findings file carries my reading of\n'
  printf 'each one and what would settle it.\n'
} > "$out" 2>&1
rm -f "$tsv.all"
cat "$out"
