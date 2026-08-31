#!/usr/bin/env nutshell
# `191` measured whether `35`'s and `43`'s FIGURES occur in the registry, found
# two of fourteen and zero of seven, and concluded the port "kept those two
# files' retirements in full and dropped almost every positive result they
# established". This asks what the transmission path actually is.
#
# WHY THIS RUNS. The registry's `provenance` field is a `ref[]` and every row
# carries one. So the corpus states, mechanically, where each of its sentences
# came from. If member derivations reached the canon by being quoted directly,
# `35` and `43` should appear there. If they reached it by being absorbed into a
# consolidation or a canon candidate, they will not appear there however
# completely they were absorbed, and a figure search will report them missing
# whether they were dropped or carried.
#
# Those two possibilities produce the SAME reading on `191`'s instrument and
# opposite readings on this one. That is the whole reason to run it.
#
# ARM 1  which panel files does the registry cite at all
# ARM 2  do `35` and `43` appear anywhere in provenance
# ARM 3  which panel files cite `35` and `43`, and are THOSE cited by the registry
#
# THE CASE THAT MUST FAIL, and the run does not count without all three.
#   POS  `63_spj_consolidation_the_format_concept` must appear in ARM 1. It is
#        the most-cited non-anchor target and if the provenance reader misses it
#        the reader is broken.
#   NEG  a slug that is in no provenance, `99_no_such_file`, must not appear.
#   DISC the discriminator: if ARM 2 finds `35` or `43` in provenance, the
#        absorbed-not-dropped reading is WRONG and `191`'s reading survives. The
#        arm is built so that outcome is reachable and would be printed.
set -euo pipefail
root="$PWD"
while [ "$root" != "/" ] && [ ! -f "$root/mockspace.toml" ]; do root="$(dirname "$root")"; done
[ -f "$root/mockspace.toml" ] || { echo "run me from inside the repository" >&2; exit 2; }
reg="$root/mock/registry"
panel="$root/mock/research/202608072330_the-numeral-canon-panel"
echo "### registry: $reg"
echo

prov() { grep -rho '"panel::202608072330_the-numeral-canon-panel::[^"]*"' "$reg"/*.toml | sed 's/^"panel::202608072330_the-numeral-canon-panel:://; s/"$//' | sed 's/::.*//' | sort -u; }

echo "======== ARM 1. distinct panel targets the registry cites ========"
targets=$(prov)
printf '%s\n' "$targets" | grep -E '^[0-9]+_' | sed 's/^/   /'
echo
echo "   distinct numbered targets: $(printf '%s\n' "$targets" | grep -Ec '^[0-9]+_' || true)"
echo

echo "======== ARM 2. do 35 and 43 appear in any provenance ========"
for n in 35 43; do
  hit=$(printf '%s\n' "$targets" | grep -E "^${n}_" || true)
  if [ -n "$hit" ]; then
    echo "   $n  CITED   $hit    <-- 191's reading survives this arm"
  else
    echo "   $n  absent  neither the file nor its probe directory is cited anywhere"
  fi
done
echo

echo "======== ARM 3. who carried them, and is the carrier cited ========"
printf "   %-58s %5s %5s   registry-cited\n" "panel file" "->35" "->43"
for f in "$panel"/*.md; do
  b=$(basename "$f" .md)
  case "$b" in 35_*|43_*|196_*) continue;; esac
  a=$( { grep -o '`35`\|35:[0-9]\|35_probes\|35_mcsherry' "$f" 2>/dev/null || true; } | wc -l | tr -d ' ')
  c=$( { grep -o '`43`\|43:[0-9]\|43_probes\|43_rompf' "$f" 2>/dev/null || true; } | wc -l | tr -d ' ')
  if [ "$a" -eq 0 ] && [ "$c" -eq 0 ]; then continue; fi
  if printf '%s\n' "$targets" | grep -qx "$b"; then cited="YES"; else cited="no"; fi
  printf "   %-58s %5s %5s   %s\n" "$(echo "$b" | cut -c1-56)" "$a" "$c" "$cited"
done
echo

echo "======== CONTROLS ========"
if printf '%s\n' "$targets" | grep -qx "63_spj_consolidation_the_format_concept"; then
  echo "   POS  PASS  the most-cited consolidation is read out of provenance"
else
  echo "   POS  FAIL  *** the provenance reader is broken; every arm above is void ***"; exit 3
fi
if printf '%s\n' "$targets" | grep -qx "99_no_such_file"; then
  echo "   NEG  FAIL  *** the reader invents targets ***"; exit 3
else
  echo "   NEG  PASS  a nonexistent slug is not reported"
fi
