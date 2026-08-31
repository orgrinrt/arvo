#!/usr/bin/env nutshell
# `196` section 3.4 connects tarina's "withdraw exactly its own contributions"
# to `35` section 3.9's withdrawn retraction requirement. The connection is a
# real find and I keep it. Its conclusion is that tarina "maintains a derived
# sheet under pack enable and disable, which is maintenance in place, which
# needs a group", and therefore that an obligation is established.
#
# `35` established its own version of that claim by a specific method: it read
# the downstream engine's source, found incremental *skip* rather than
# incremental *aggregate*, found the one running aggregate was an EMA that
# avoids needing an inverse, and withdrew. `196` did not repeat that method on
# tarina. This does.
#
# The question is one sentence: does tarina maintain the derived sheet in place,
# or recompute it? A group is needed only for the first. `35` 3.9's own words:
# "a contract distinguishing a monoid (maintainable only by recomputation or a
# tree) from a group (maintainable in place)".
#
# ARMS.
#   1. Every sentence in tarina's design stating how derivation runs.
#   2. Every sentence stating what a disabled pack does.
#   3. The vocabulary of in-place maintenance, searched for: incremental,
#      recompute, invalidate, cache, memo, dirty.
#
# CONTROLS, three.
#   POSITIVE-A "fold" must be found. It is the word the design uses for the
#     derivation and if ARM 1 misses it the extractor is broken.
#   POSITIVE-B "withdraw" must be found, because it is `196`'s own quotation and
#     if this instrument cannot see it, it cannot see the other side either.
#   NEGATIVE "zzz_not_in_this_document" must report zero.
#
# The answer is not assumed either way here: an ARM 1 reporting in-place
# maintenance would establish `196`'s obligation and I would concede it whole.
set -euo pipefail
root="$PWD"
while [ "$root" != "/" ] && [ ! -f "$root/mockspace.toml" ]; do root="$(dirname "$root")"; done
ws="$(dirname "$root")"
d="$ws/tarina/DESIGN.md"
[ -f "$d" ] || { echo "tarina not cloned beside this workspace" >&2; exit 2; }
echo "### source: $d  ($(wc -l < "$d") lines)"
echo

echo "######## ARM 1. how derivation runs"
{ grep -n -i -E "derivation is|ordered fold|each stage|consumes the record" "$d" || true; } | sed 's/^/  /'
echo
echo "######## ARM 2. what a disabled pack does"
{ grep -n -i -E "disabled|withdraw|contribution" "$d" || true; } | sed 's/^/  /'
echo
echo "######## ARM 3. the vocabulary of in-place maintenance"
for w in incremental recompute invalidat cache memo dirty "in place" "maintain"; do
  n=$({ grep -ci -- "$w" "$d" || true; })
  printf "  %-12s %s\n" "$w" "$n"
  [ "$n" -eq 0 ] || { grep -in -- "$w" "$d" || true; } | cut -c1-140 | sed 's/^/      /'
done
echo
echo "######## CONTROLS"
for w in fold withdraw zzz_not_in_this_document; do
  printf "  %-26s %s\n" "$w" "$({ grep -ci -- "$w" "$d" || true; })"
done
