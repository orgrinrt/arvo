#!/usr/bin/env nutshell
# Every `provenance` citation in the registry ending in a heading anchor, checked
# against the headings of the file it names.
#
# THE PREMISE THIS PROBE WAS BUILT ON WAS FALSE, and the correction is the most
# useful thing in it. Version one opened with "nothing resolves a heading
# anchor", because `mock/checks/src/citation.rs` only distinguishes a line
# anchor from a heading one. **The mockspace engine resolves them**, at
# `src/registry/refs.rs:357` `resolve_anchor`, and reports `unresolvable-heading`
# on a failure. I found that by writing a bad anchor and watching the lint refuse
# it, not by reading, and a probe built to fill a gap that is already filled is
# worth saying so about.
#
# So this is an independent reimplementation rather than a new check, and its
# value is the second instance: it agrees with the engine at 182 of 182.
#
# WHAT IT IS STILL FOR, and it is what cost me an hour. The engine accepts **two**
# slug forms and its own documentation says why:
#   PROJECT  every non-alphanumeric run becomes one hyphen.
#            "4.1 Multi-instance, per the consolidation's own account"
#            -> 4-1-multi-instance-per-the-consolidation-s-own-account
#   FORGE    punctuation inside a word is dropped rather than hyphenated, which
#            is what a browser's address bar shows.
#            -> 41-multi-instance-per-the-consolidations-own-account
# **A citation mixing the two resolves under neither**, and that is the failure
# that is easy to write and impossible to see: I wrote `4-1-` from the project
# form and `consolidations` from the forge form, and the engine refused it while
# both pure spellings pass. The error message names the anchor and not the near
# miss, so the two-form rule has to be known in advance.
#
# DEFECT IN VERSION ONE, kept for the same reason. It modelled two conventions
# and reported **31 unresolved of 182**. Reading the detail rather than the count
# showed most were apostrophes: `## 3. Warm's headroom rule` cited as
# `#3-warm-s-headroom-rule`, which is the project form and which neither of my
# two rules produced. The number in `_v1_two_rules.out` is wrong and the
# transcript is kept because the detail under it is what corrected it.
#
# THREE RULES, tried in turn, the third being the engine's project form:
#   STRICT  drop punctuation.        "Warm's headroom rule" -> warms-headroom-rule
#   LOOSE   a dot becomes a hyphen.  "4.1 Multi-instance"   -> 4-1-multi-instance
#   PUNCT   every non-alphanumeric run becomes one hyphen, which is the engine's
#           PROJECT form.            "Warm's headroom rule" -> warm-s-headroom-rule
#
# CONTROLS, four.
#   POSITIVE-A a citation known good must resolve.
#   POSITIVE-B a heading with a dot must resolve, and under which rule is the
#     evidence about which convention that citation was written in.
#   POSITIVE-C a heading with an apostrophe must resolve under PUNCT and not
#     under STRICT, which is the case version one got wrong.
#   NEGATIVE a fabricated anchor must resolve under none.
#     is the evidence about which convention that citation used.
#   POSITIVE-C a heading with an apostrophe must resolve under PUNCT and not
#     under STRICT, which is the case version one got wrong.
#   NEGATIVE a fabricated anchor must resolve under none.
set -euo pipefail
root="$PWD"
while [ "$root" != "/" ] && [ ! -f "$root/mockspace.toml" ]; do root="$(dirname "$root")"; done
reg="$root/mock/registry"
panel="$root/mock/research/202608072330_the-numeral-canon-panel"

slugs=$(mktemp); cites=$(mktemp); trap 'rm -f "$slugs" "$cites"' EXIT

for f in "$panel"/*.md; do
  b=$(basename "$f" .md)
  awk -v b="$b" '
    /^#{1,6} / {
      h=$0; sub(/^#+ +/, "", h); orig=h
      s=tolower(h)
      strict=s; gsub(/[^a-z0-9 -]/, "", strict); gsub(/ +/, "-", strict)
      loose=s;  gsub(/\./, "-", loose);  gsub(/[^a-z0-9 -]/, "", loose);  gsub(/ +/, "-", loose)
      punct=s;  gsub(/[^a-z0-9]/, "-", punct); gsub(/--+/, "-", punct)
      sub(/^-/, "", punct); sub(/-$/, "", punct)
      print b "\t" strict "\tSTRICT\t" orig
      print b "\t" loose  "\tLOOSE\t"  orig
      print b "\t" punct  "\tPUNCT\t"  orig
    }' "$f" >> "$slugs"
done
echo "### headings slugified: $(grep -c . "$slugs") entries over $(ls "$panel"/*.md | wc -l | tr -d ' ') files"

grep -rhoE '"panel::[^"]*::#[^"]*"' "$reg" | tr -d '"' | sort -u > "$cites"
echo "### heading-anchor citations: $(grep -c . "$cites")"
echo

unresolved=0; nofile=0
while read -r c; do
  file=$(printf '%s' "$c" | awk -F'::' '{print $(NF-1)}')
  anch=$(printf '%s' "$c" | awk -F'::#' '{print $NF}')
  hit=$({ grep -F "$(printf '%s\t%s\t' "$file" "$anch")" "$slugs" || true; })
  if [ -z "$hit" ]; then
    if [ -f "$panel/$file.md" ]; then
      unresolved=$((unresolved + 1))
      echo "  BROKEN      $file"
      echo "              #$anch"
    else
      nofile=$((nofile + 1))
      echo "  NOT-WALKED  $file  (not a markdown file at the panel root; this walker never opened it)"
    fi
  fi
done < "$cites"
echo
echo "### broken, file present:      $unresolved of $(grep -c . "$cites")"
echo "### unchecked, file not walked: $nofile"
echo

echo "######## CONTROLS"
ck() {
  lbl="$1"; f="$2"; a="$3"; want="$4"
  got=$({ grep -F "$(printf '%s\t%s\t' "$f" "$a")" "$slugs" || true; } | awk -F'\t' '{printf "%s ", $3}')
  [ -n "$got" ] || got="(none)"
  printf "  %-12s %-52s -> %-18s want: %s\n" "$lbl" "#$a" "$got" "$want"
}
ck "POSITIVE-A" "AGREEMENTS" "6-cross-topic-agreements" "resolves"
ck "POSITIVE-B" "AGREEMENTS" "4-1-multi-instance-per-the-consolidations-own-account" "resolves, dotted"
ck "POSITIVE-C" "01_op_answers" "3-warm-s-headroom-rule" "PUNCT only"
ck "POSITIVE-C" "01_op_answers" "3-warms-headroom-rule" "STRICT and LOOSE"
ck "NEGATIVE"   "AGREEMENTS" "zzz-no-such-heading-anywhere" "none"
