#!/usr/bin/env bash
# p3: which delimiter can carry a warrant on the value side without changing
# what a single existing entry means.
#
# The append-only rule is the binding constraint: a grammar extension that
# reparses one committed entry has widened a predicate in place. So the
# candidate delimiter has to occur zero times in the 527 committed spans, and
# that is a measurement rather than a judgement.
#
# Run from the repo root.

set -uo pipefail
REG="mock/registry"
[ -d "$REG" ] || { echo "run me from the repo root; no $REG here" >&2; exit 2; }

spans() {
  awk '
    /^(predicate|holds|fails) = \[/ { inarr=1 }
    inarr {
      n = split($0, parts, "\"")
      for (i = 2; i <= n; i += 2) {
        s = parts[i]
        if (index(s, ":") == 0) continue
        print substr(s, index(s, ":") + 1)
      }
    }
    inarr && /\]/ { inarr=0 }
  ' "$REG"/*.toml
}

echo "### spans read: $(spans | wc -l | tr -d ' ')"
echo
echo "### occurrences of each candidate delimiter inside a span"
for d in ':' ';' '|' '@' '~' '::' '--' '=>'; do
  n=$(spans | grep -cF -- "$d" || true)
  printf "  %-4s %s\n" "$d" "$n"
done

echo
echo "### spans containing a comma followed by an explanatory clause"
echo "    (these are where a warrant is already being written as prose)"
spans | grep -E ', [a-z]' | sed 's/^/  /' | sort -u

echo
echo "### CONTROL. A delimiter known to be present must report non-zero."
present=$(spans | grep -cF -- "," || true)
echo "  ',' occurs $present times"
[ "$present" != "0" ] || echo "CONTROL FAILED: the reader cannot see a comma, so every zero above is meaningless"

echo
echo "### CONTROL. A string known absent must report zero."
absent=$(spans | grep -cF -- "zzqq" || true)
echo "  'zzqq' occurs $absent times"
[ "$absent" = "0" ] || echo "CONTROL FAILED"
